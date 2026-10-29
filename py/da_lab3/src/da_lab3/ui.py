import math
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg
import tkinter as tk
from tkinter import filedialog, messagebox, ttk
import scipy.stats as st

from .calculations import (
    load_dataset, build_variation_series, calculate_sturges_bins,
    build_histogram_data, calculate_base_statistics,
    calculate_confidence_intervals, calculate_bootstrap_intervals
)

class StatsApp:
    def __init__(self, root: tk.Tk):
        self.root = root
        self.root.title("Analyzer App")
        self.root.geometry("1400x850")
        self.raw_data: list[float] = []
        self._build_ui()

    def _build_ui(self):
        top_frame = ttk.Frame(self.root, padding=10)
        top_frame.pack(fill=tk.X)

        ttk.Button(top_frame, text="Open File", command=self.open_file_dialog).pack(side=tk.LEFT, padx=5)
        self.file_label = ttk.Label(top_frame, text="No file selected")
        self.file_label.pack(side=tk.LEFT, padx=10)

        ttk.Label(top_frame, text="Bins:").pack(side=tk.LEFT, padx=(20, 5))
        self.bin_slider = ttk.Scale(top_frame, from_=1, to=30, orient=tk.HORIZONTAL, command=self.on_bin_slider_move)
        self.bin_slider.pack(side=tk.LEFT, padx=5)
        self.bin_val_label = ttk.Label(top_frame, text="1")
        self.bin_val_label.pack(side=tk.LEFT, padx=5)

        ttk.Label(top_frame, text="Confidence Level (alpha):").pack(side=tk.LEFT, padx=(20, 5))
        self.conf_var = tk.StringVar(value="95% (a=0.05)")
        self.conf_cb = ttk.Combobox(top_frame, textvariable=self.conf_var, values=["90% (a=0.10)", "95% (a=0.05)", "99% (a=0.01)"], width=15, state="readonly")
        self.conf_cb.pack(side=tk.LEFT, padx=5)
        self.conf_cb.bind("<<ComboboxSelected>>", lambda e: self.render_all())

        paned = ttk.PanedWindow(self.root, orient=tk.HORIZONTAL)
        paned.pack(fill=tk.BOTH, expand=True, padx=10, pady=10)

        left_frame = ttk.Frame(paned)
        paned.add(left_frame, weight=1)

        ttk.Label(left_frame, text="Variation Series").pack(anchor=tk.W, pady=2)
        var_frame = ttk.Frame(left_frame)
        var_frame.pack(fill=tk.BOTH, expand=True)
        self.tree = ttk.Treeview(var_frame, columns=("val", "count", "rel", "emp_f", "quant"), show="headings")
        self.tree.heading("val", text="Variant (x_i)")
        self.tree.heading("count", text="Freq (n_i)")
        self.tree.heading("rel", text="Rel. Freq")
        self.tree.heading("emp_f", text="F_N(x)")
        self.tree.heading("quant", text="Quantile (u)")
        for col in self.tree["columns"]:
            self.tree.column(col, width=70, anchor=tk.CENTER)
        scroll1 = ttk.Scrollbar(var_frame, orient=tk.VERTICAL, command=self.tree.yview)
        self.tree.configure(yscroll=scroll1.set)
        self.tree.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        scroll1.pack(side=tk.RIGHT, fill=tk.Y)

        ttk.Label(left_frame, text="Statistical Characteristics & Confidence Intervals").pack(anchor=tk.W, pady=(10, 2))
        stat_frame = ttk.Frame(left_frame)
        stat_frame.pack(fill=tk.BOTH, expand=True)
        self.stats_tree = ttk.Treeview(stat_frame, columns=("name", "est", "ci", "boot_ci"), show="headings")
        self.stats_tree.heading("name", text="Metric")
        self.stats_tree.heading("est", text="Estimate")
        self.stats_tree.heading("ci", text="Classic CI")
        self.stats_tree.heading("boot_ci", text="Bootstrap CI")
        self.stats_tree.column("name", width=100, anchor=tk.W)
        self.stats_tree.column("est", width=70, anchor=tk.CENTER)
        self.stats_tree.column("ci", width=130, anchor=tk.CENTER)
        self.stats_tree.column("boot_ci", width=130, anchor=tk.CENTER)
        self.stats_tree.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

        ttk.Label(left_frame, text="Statistical Tests (Normality)").pack(anchor=tk.W, pady=(10, 2))
        test_frame = ttk.Frame(left_frame)
        test_frame.pack(fill=tk.BOTH, expand=True)
        self.test_tree = ttk.Treeview(test_frame, columns=("test", "stat", "crit", "res"), show="headings")
        self.test_tree.heading("test", text="Test")
        self.test_tree.heading("stat", text="Statistic")
        self.test_tree.heading("crit", text="Critical / p-val")
        self.test_tree.heading("res", text="Conclusion")
        self.test_tree.column("test", width=120, anchor=tk.W)
        self.test_tree.column("stat", width=90, anchor=tk.CENTER)
        self.test_tree.column("crit", width=90, anchor=tk.CENTER)
        self.test_tree.column("res", width=140, anchor=tk.CENTER)
        self.test_tree.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

        right_frame = ttk.Frame(paned)
        paned.add(right_frame, weight=2)
        self.fig, (self.ax1, self.ax2) = plt.subplots(1, 2, figsize=(10, 4))
        self.canvas = FigureCanvasTkAgg(self.fig, master=right_frame)
        self.canvas.get_tk_widget().pack(fill=tk.BOTH, expand=True)

    def load_data_from_path(self, file_path: str):
        try:
            data = load_dataset(file_path)
            if not data:
                messagebox.showwarning("Empty Data", "No numerical values found.")
                return
            self.raw_data = data
            n = len(self.raw_data)
            default_bins = calculate_sturges_bins(n)
            self.file_label.config(text=f"{file_path.split('/')[-1]} (N={n})")
            self.bin_slider.set(default_bins)
            self.bin_val_label.config(text=str(default_bins))
            self.render_all()
        except Exception as err:
            messagebox.showerror("Error", f"Failed to read file:\n{err}")

    def open_file_dialog(self):
        file_path = filedialog.askopenfilename(filetypes=[("Text files", "*.txt"), ("All files", "*.*")])
        if file_path:
            self.load_data_from_path(file_path)

    def on_bin_slider_move(self, val):
        self.bin_val_label.config(text=str(int(float(val))))
        if self.raw_data:
            self.draw_charts(int(float(val)))

    def render_all(self):
        if not self.raw_data: return
        self.tree.delete(*self.tree.get_children())
        self.stats_tree.delete(*self.stats_tree.get_children())
        self.test_tree.delete(*self.test_tree.get_children())

        series = build_variation_series(self.raw_data)
        emp_f = 0.0
        qq_x, qq_u = [], []
        for i, row in enumerate(series):
            emp_f += row['relative_freq']
            u_val = ""
            if i < len(series) - 1:
                u = st.norm.ppf(emp_f)
                u_val = f"{u:.4f}"
                qq_x.append(row['value'])
                qq_u.append(u)
            self.tree.insert("", tk.END, values=(f"{row['value']:.4g}", row['count'], f"{row['relative_freq']:.4f}", f"{emp_f:.4f}", u_val))

        conf_str = self.conf_var.get()
        alpha = 0.05 if "0.05" in conf_str else (0.01 if "0.01" in conf_str else 0.10)
        conf_level = 1.0 - alpha
        
        try:
            stats = calculate_base_statistics(self.raw_data)
            intervals = calculate_confidence_intervals(self.raw_data, stats, conf_level)
            boot_intervals = calculate_bootstrap_intervals(self.raw_data, conf_level)
            
            rows = [
                ("Mean", f"{stats['mean']:.4f}", f"[{intervals['mean_ci'][0]:.4f}; {intervals['mean_ci'][1]:.4f}]", f"[{boot_intervals['mean_boot_ci'][0]:.4f}; {boot_intervals['mean_boot_ci'][1]:.4f}]"),
                ("Median", f"{stats['median']:.4f}", f"[{intervals['median_ci'][0]:.4f}; {intervals['median_ci'][1]:.4f}]", f"[{boot_intervals['median_boot_ci'][0]:.4f}; {boot_intervals['median_boot_ci'][1]:.4f}]"),
                ("Std Dev (S)", f"{stats['s_unbias']:.4f}", f"[{intervals['s_unbias_ci'][0]:.4f}; {intervals['s_unbias_ci'][1]:.4f}]", f"[{boot_intervals['s_unbias_boot_ci'][0]:.4f}; {boot_intervals['s_unbias_boot_ci'][1]:.4f}]"),
                ("Skewness (A2)", f"{stats['skewness_a2']:.4f}", f"[{intervals['skewness_ci'][0]:.4f}; {intervals['skewness_ci'][1]:.4f}]", f"[{boot_intervals['skewness_boot_ci'][0]:.4f}; {boot_intervals['skewness_boot_ci'][1]:.4f}]"),
                ("Kurtosis (E2)", f"{stats['kurtosis_e2']:.4f}", f"[{intervals['kurtosis_ci'][0]:.4f}; {intervals['kurtosis_ci'][1]:.4f}]", f"[{boot_intervals['kurtosis_boot_ci'][0]:.4f}; {boot_intervals['kurtosis_boot_ci'][1]:.4f}]"),
            ]
            for r in rows:
                self.stats_tree.insert("", tk.END, values=r)

            n = stats["n"]
            se_a2 = math.sqrt((6 * n * (n - 1)) / ((n - 2) * (n + 1) * (n + 3)))
            se_e2 = math.sqrt((24 * n * (n - 1) ** 2) / ((n - 3) * (n - 2) * (n + 3) * (n + 5)))
            
            u_a = stats["skewness_a2"] / se_a2
            u_e = stats["kurtosis_e2"] / se_e2
            u_crit = st.norm.ppf(1 - alpha / 2)
            
            res_a = "Normal" if abs(u_a) <= u_crit else "Not Normal"
            res_e = "Normal" if abs(u_e) <= u_crit else "Not Normal"
            w_stat, p_val = st.shapiro(self.raw_data)
            res_shap = "Normal" if p_val >= alpha else "Not Normal"
            
            self.test_tree.insert("", tk.END, values=("Skewness (A=0)", f"{u_a:.4f}", f"u_crit={u_crit:.4f}", res_a))
            self.test_tree.insert("", tk.END, values=("Kurtosis (E=0)", f"{u_e:.4f}", f"u_crit={u_crit:.4f}", res_e))
            self.test_tree.insert("", tk.END, values=("Shapiro-Wilk", f"{w_stat:.4f}", f"p={p_val:.4g}", res_shap))
            
        except ValueError:
            pass

        self.draw_charts(int(self.bin_slider.get()), qq_x, qq_u)

    def draw_charts(self, num_bins: int, qq_x=None, qq_u=None):
        self.ax1.clear()
        self.ax2.clear()

        bins = build_histogram_data(self.raw_data, num_bins)
        centers = [(b["start"] + b["end"]) / 2 for b in bins]
        heights = [b["relative_freq"] for b in bins]
        widths = [b["end"] - b["start"] for b in bins]

        self.ax1.bar(centers, heights, width=widths, edgecolor="black", align="center", alpha=0.6, label="Empirical")
        stats = calculate_base_statistics(self.raw_data)
        x_min, x_max = min(self.raw_data), max(self.raw_data)
        x_val = np.linspace(x_min - stats["s_unbias"], x_max + stats["s_unbias"], 100)
        
        step = widths[0] if widths else 1
        pdf_val = st.norm.pdf(x_val, stats["mean"], stats["s_unbias"]) * step
        self.ax1.plot(x_val, pdf_val, color="red", linewidth=2, label="Normal PDF")
        
        self.ax1.set_title("Histogram & Normal Curve")
        self.ax1.set_xlabel("Values")
        self.ax1.set_ylabel("Relative Frequency")
        self.ax1.legend()
        self.ax1.grid(True, linestyle="--", alpha=0.5)

        if qq_x is None or qq_u is None:
            series = build_variation_series(self.raw_data)
            emp_f, qq_x, qq_u = 0.0, [], []
            for i in range(len(series) - 1):
                emp_f += series[i]['relative_freq']
                qq_x.append(series[i]['value'])
                qq_u.append(st.norm.ppf(emp_f))

        self.ax2.scatter(qq_x, qq_u, color="blue", edgecolors="k", zorder=3)
        self.ax2.set_title("Q-Q Plot (Probability Paper)")
        self.ax2.set_xlabel("Empirical Quantiles (x)")
        self.ax2.set_ylabel("Theoretical Quantiles (u)")
        self.ax2.grid(True, linestyle="--", alpha=0.5)

        self.fig.tight_layout()
        self.canvas.draw()
