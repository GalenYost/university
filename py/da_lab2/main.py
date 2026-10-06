# /// script
# dependencies = [
#     "matplotlib",
#     "scipy",
# ]
# ///

import math
import sys
import random

import matplotlib.pyplot as plt
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg

import tkinter as tk
from tkinter import filedialog, messagebox, ttk

import scipy.stats as st

def load_dataset(file_path: str) -> list[float]:
    with open(file_path, "r", encoding="utf-8") as file:
        content = file.read()

    cleaned_content = content.replace(",", " ")

    data = []
    for token in cleaned_content.split():
        try:
            data.append(float(token))
        except ValueError:
            continue

    return data

def build_variation_series(data: list[float]) -> list[dict]:
    if not data:
        return []

    sorted_data = sorted(data)
    total = len(sorted_data)
    series = []

    current_val = sorted_data[0]
    count = 0

    for val in sorted_data:
        if abs(val - current_val) < 1e-9:
            count += 1
        else:
            series.append({
                "value": current_val,
                "count": count,
                "relative_freq": count / total
            })
            current_val = val
            count = 1

    series.append({
        "value": current_val,
        "count": count,
        "relative_freq": count / total
    })

    return series

def calculate_sturges_bins(n: int) -> int:
    if n <= 0:
        return 1
    return max(1, int(round(1 + math.log2(n))))

def build_histogram_data(data: list[float], num_bins: int) -> list[dict]:
    if not data or num_bins <= 0:
        return []

    min_val = min(data)
    max_val = max(data)

    if abs(max_val - min_val) < 1e-9:
        return [{
            "start": min_val - 0.5,
            "end": max_val + 0.5,
            "count": len(data),
            "relative_freq": 1.0
        }]

    step = (max_val - min_val) / num_bins
    total = len(data)

    bins = []
    for i in range(num_bins):
        start = min_val + i * step
        end = max_val + 1e-9 if i == num_bins - 1 else start + step
        bins.append({
            "start": start,
            "end": end,
            "count": 0,
            "relative_freq": 0.0
        })

    for val in data:
        idx = int((val - min_val) / step)
        idx = min(idx, num_bins - 1)
        bins[idx]["count"] += 1

    for b in bins:
        b["relative_freq"] = b["count"] / total

    return bins

def calculate_base_statistics(data: list[float]) -> dict:
    n = len(data)
    if n < 4:
        raise ValueError("calculate_base_statistics requires at least 4 elements.")

    mean = sum(data) / n

    sorted_data = sorted(data)
    if n % 2 != 0:
        median = sorted_data[n // 2]
    else:
        median = (sorted_data[n // 2 - 1] + sorted_data[n // 2]) / 2.0

    sum_sq = sum((x - mean) ** 2 for x in data)
    sum_cube = sum((x - mean) ** 3 for x in data)
    sum_quad = sum((x - mean) ** 4 for x in data)

    if sum_sq == 0:
        return {
            "mean": mean, "median": median, "s_unbias": 0.0, 
            "skewness_a2": 0.0, "kurtosis_e2": 0.0, "n": n
        }

    s_unbias = math.sqrt(sum_sq / (n - 1))
    s_bias = math.sqrt(sum_sq / n)

    a1 = sum_cube / (n * (s_bias ** 3))
    a2 = (math.sqrt(n * (n - 1)) / (n - 2)) * a1

    e1 = sum_quad / (n * (s_bias ** 4)) - 3
    e2 = ((n ** 2 - 1) / ((n - 2) * (n - 3))) * (e1 + 6 / (n + 1))

    return {
        "mean": mean,
        "median": median,
        "s_unbias": s_unbias,
        "skewness_a2": a2,
        "kurtosis_e2": e2,
        "n": n
    }

def calculate_confidence_intervals(data: list[float], stats: dict, confidence_level: float = 0.95) -> dict:
    n = stats["n"]
    if n < 4:
        return {}

    alpha = 1.0 - confidence_level
    sorted_data = sorted(data)

    u_val = st.norm.ppf(1 - alpha / 2)
    t_val = st.t.ppf(1 - alpha / 2, df=n - 1)
    chi2_lower_q = st.chi2.ppf(1 - alpha / 2, df=n - 1)
    chi2_upper_q = st.chi2.ppf(alpha / 2, df=n - 1)

    se_mean = stats["s_unbias"] / math.sqrt(n)
    mean_ci_lower = stats["mean"] - t_val * se_mean
    mean_ci_upper = stats["mean"] + t_val * se_mean

    j_val = (n / 2) - u_val * (math.sqrt(n) / 2)
    k_val = (n / 2) + 1 + u_val * (math.sqrt(n) / 2)
    
    j_idx = max(0, int(round(j_val)) - 1)
    k_idx = min(n - 1, int(round(k_val)) - 1)
    
    median_ci_lower = sorted_data[j_idx]
    median_ci_upper = sorted_data[k_idx]

    s_sq_sum = (n - 1) * (stats["s_unbias"] ** 2)
    s_ci_lower = math.sqrt(s_sq_sum / chi2_lower_q)
    s_ci_upper = math.sqrt(s_sq_sum / chi2_upper_q)

    se_skew = math.sqrt((6 * n * (n - 1)) / ((n - 2) * (n + 1) * (n + 3)))
    skew_ci_lower = stats["skewness_a2"] - u_val * se_skew
    skew_ci_upper = stats["skewness_a2"] + u_val * se_skew

    se_kurt = math.sqrt((24 * n * (n - 1) ** 2) / ((n - 3) * (n - 2) * (n + 3) * (n + 5)))
    kurt_ci_lower = stats["kurtosis_e2"] - u_val * se_kurt
    kurt_ci_upper = stats["kurtosis_e2"] + u_val * se_kurt

    return {
        "mean_ci": (mean_ci_lower, mean_ci_upper),
        "median_ci": (median_ci_lower, median_ci_upper),
        "s_unbias_ci": (s_ci_lower, s_ci_upper),
        "skewness_ci": (skew_ci_lower, skew_ci_upper),
        "kurtosis_ci": (kurt_ci_lower, kurt_ci_upper)
    }

def calculate_bootstrap_intervals(data: list[float], confidence_level: float = 0.95, iterations: int = 1000) -> dict:
    n = len(data)
    if n < 4:
        return {}

    means = []
    medians = []
    s_unbiases = []
    skewnesses = []
    kurtosises = []

    for _ in range(iterations):
        sample = random.choices(data, k=n)
        
        mean = sum(sample) / n
        means.append(mean)
        
        sorted_sample = sorted(sample)
        if n % 2 != 0:
            medians.append(sorted_sample[n // 2])
        else:
            medians.append((sorted_sample[n // 2 - 1] + sorted_sample[n // 2]) / 2.0)
            
        sum_sq = sum((x - mean) ** 2 for x in sample)
        sum_cube = sum((x - mean) ** 3 for x in sample)
        sum_quad = sum((x - mean) ** 4 for x in sample)

        if sum_sq == 0:
            s_unbiases.append(0.0)
            skewnesses.append(0.0)
            kurtosises.append(0.0)
        else:
            s_unbias = math.sqrt(sum_sq / (n - 1))
            s_bias = math.sqrt(sum_sq / n)
            s_unbiases.append(s_unbias)
            
            a1 = sum_cube / (n * (s_bias ** 3))
            a2 = (math.sqrt(n * (n - 1)) / (n - 2)) * a1
            skewnesses.append(a2)
            
            e1 = sum_quad / (n * (s_bias ** 4)) - 3
            e2 = ((n ** 2 - 1) / ((n - 2) * (n - 3))) * (e1 + 6 / (n + 1))
            kurtosises.append(e2)

    means.sort()
    medians.sort()
    s_unbiases.sort()
    skewnesses.sort()
    kurtosises.sort()

    alpha = 1.0 - confidence_level
    lower_idx = int(iterations * (alpha / 2))
    upper_idx = int(iterations * (1 - alpha / 2)) - 1

    return {
        "mean_boot_ci": (means[lower_idx], means[upper_idx]),
        "median_boot_ci": (medians[lower_idx], medians[upper_idx]),
        "s_unbias_boot_ci": (s_unbiases[lower_idx], s_unbiases[upper_idx]),
        "skewness_boot_ci": (skewnesses[lower_idx], skewnesses[upper_idx]),
        "kurtosis_boot_ci": (kurtosises[lower_idx], kurtosises[upper_idx])
    }

class StatsApp:
    def __init__(self, root: tk.Tk):
        self.root = root
        self.root.title("Analyzer")
        self.root.geometry("1050x600")

        self.raw_data: list[float] = []

        self._build_ui()

    def _build_ui(self):
        top_frame = ttk.Frame(self.root, padding=10)
        top_frame.pack(fill=tk.X)

        ttk.Button(top_frame, text="Open File", command=self.open_file_dialog).pack(side=tk.LEFT, padx=5)
        self.file_label = ttk.Label(top_frame, text="No file selected")
        self.file_label.pack(side=tk.LEFT, padx=10)

        ttk.Label(top_frame, text="Bins:").pack(side=tk.LEFT, padx=(20, 5))
        self.bin_slider = ttk.Scale(
            top_frame, from_=1, to=30, orient=tk.HORIZONTAL, command=self.on_bin_slider_move
        )
        self.bin_slider.pack(side=tk.LEFT, padx=5)
        self.bin_val_label = ttk.Label(top_frame, text="1")
        self.bin_val_label.pack(side=tk.LEFT, padx=5)

        ttk.Label(top_frame, text="Confidence Level:").pack(side=tk.LEFT, padx=(20, 5))
        self.conf_var = tk.StringVar(value="95%")
        self.conf_cb = ttk.Combobox(
            top_frame, textvariable=self.conf_var, values=["90%", "95%", "99%"], width=5, state="readonly"
        )
        self.conf_cb.pack(side=tk.LEFT, padx=5)
        self.conf_cb.bind("<<ComboboxSelected>>", lambda e: self.render_all())

        paned = ttk.PanedWindow(self.root, orient=tk.HORIZONTAL)
        paned.pack(fill=tk.BOTH, expand=True, padx=10, pady=10)

        left_frame = ttk.Frame(paned)
        paned.add(left_frame, weight=1)

        ttk.Label(left_frame, text="Variation Series").pack(anchor=tk.W, pady=2)
        var_frame = ttk.Frame(left_frame)
        var_frame.pack(fill=tk.BOTH, expand=True)
        
        self.tree = ttk.Treeview(var_frame, columns=("val", "count", "rel"), show="headings")
        self.tree.heading("val", text="Variant (x_i)")
        self.tree.heading("count", text="Freq (n_i)")
        self.tree.heading("rel", text="Rel. Freq (w_i)")
        self.tree.column("val", width=90, anchor=tk.CENTER)
        self.tree.column("count", width=80, anchor=tk.CENTER)
        self.tree.column("rel", width=100, anchor=tk.CENTER)
        
        scroll1 = ttk.Scrollbar(var_frame, orient=tk.VERTICAL, command=self.tree.yview)
        self.tree.configure(yscroll=scroll1.set)
        self.tree.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        scroll1.pack(side=tk.RIGHT, fill=tk.Y)

        ttk.Label(left_frame, text="Statistical Characteristics & Confidence Intervals").pack(anchor=tk.W, pady=(15, 2))
        stat_frame = ttk.Frame(left_frame)
        stat_frame.pack(fill=tk.BOTH, expand=False)
        
        self.stats_tree = ttk.Treeview(stat_frame, columns=("name", "est", "ci", "boot_ci"), show="headings", height=6)
        self.stats_tree.heading("name", text="Metric")
        self.stats_tree.heading("est", text="Estimate")
        self.stats_tree.heading("ci", text="Classic CI")
        self.stats_tree.heading("boot_ci", text="Bootstrap CI")
        
        self.stats_tree.column("name", width=110, anchor=tk.W)
        self.stats_tree.column("est", width=80, anchor=tk.CENTER)
        self.stats_tree.column("ci", width=140, anchor=tk.CENTER)
        self.stats_tree.column("boot_ci", width=140, anchor=tk.CENTER)
        self.stats_tree.pack(side=tk.LEFT, fill=tk.X, expand=True)

        chart_frame = ttk.Frame(paned)
        paned.add(chart_frame, weight=2)
        ttk.Label(chart_frame, text="Histogram").pack(anchor=tk.W, pady=2)
        self.fig, self.ax = plt.subplots(figsize=(5, 4))
        self.canvas = FigureCanvasTkAgg(self.fig, master=chart_frame)
        self.canvas.get_tk_widget().pack(fill=tk.BOTH, expand=True)

    def load_data_from_path(self, file_path: str):
        try:
            data = load_dataset(file_path)
            if not data:
                messagebox.showwarning("Empty Data", "No numerical values found in file.")
                return

            self.raw_data = data
            n = len(self.raw_data)

            default_bins = calculate_sturges_bins(n)
            filename = file_path.split("/")[-1]

            self.file_label.config(text=f"{filename} (N={n})")
            self.bin_slider.set(default_bins)
            self.bin_val_label.config(text=str(default_bins))

            self.render_all()

        except Exception as err:
            messagebox.showerror("File Error", f"Failed to read file:\n{err}")

    def open_file_dialog(self):
        file_path = filedialog.askopenfilename(
            title="Select Text File",
            filetypes=[("Text files", "*.txt"), ("All files", "*.*")],
        )
        if file_path:
            self.load_data_from_path(file_path)

    def on_bin_slider_move(self, val):
        num_bins = int(float(val))
        self.bin_val_label.config(text=str(num_bins))
        if self.raw_data:
            self.draw_histogram(num_bins)

    def render_all(self):
        if not self.raw_data:
            return

        for item in self.tree.get_children():
            self.tree.delete(item)
        for item in self.stats_tree.get_children():
            self.stats_tree.delete(item)

        series = build_variation_series(self.raw_data)
        for row in series:
            self.tree.insert("", tk.END, values=(f"{row['value']:.4g}", row['count'], f"{row['relative_freq']:.4f}"))

        conf_str = self.conf_var.get().replace("%", "")
        conf_level = float(conf_str) / 100.0

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
        except ValueError:
            pass 

        num_bins = int(self.bin_slider.get())
        self.draw_histogram(num_bins)

    def draw_histogram(self, num_bins: int):
        bins = build_histogram_data(self.raw_data, num_bins)

        self.ax.clear()

        centers = [(b["start"] + b["end"]) / 2 for b in bins]
        heights = [b["count"] for b in bins]
        widths = [b["end"] - b["start"] for b in bins]

        self.ax.bar(centers, heights, width=widths, edgecolor="black", align="center", alpha=0.7)
        self.ax.set_ylabel("Frequency (n_i)")
        self.ax.set_xlabel("Intervals")
        self.ax.grid(True, linestyle="--", alpha=0.5)

        self.fig.tight_layout()
        self.canvas.draw()

if __name__ == "__main__":
    root = tk.Tk()
    app = StatsApp(root)

    if len(sys.argv) > 1:
        app.load_data_from_path(sys.argv[1])

    root.mainloop()
