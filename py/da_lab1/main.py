# /// script
# dependencies = [
#     "matplotlib",
# ]
# ///

import math
import sys

import matplotlib.pyplot as plt
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg

import tkinter as tk
from tkinter import filedialog, messagebox, ttk

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

        ttk.Label(top_frame, text="Bins (Classes):").pack(side=tk.LEFT, padx=(20, 5))

        self.bin_slider = ttk.Scale(
            top_frame,
            from_=1,
            to=30,
            orient=tk.HORIZONTAL,
            command=self.on_bin_slider_move,
        )
        self.bin_slider.pack(side=tk.LEFT, padx=5)

        self.bin_val_label = ttk.Label(top_frame, text="1")
        self.bin_val_label.pack(side=tk.LEFT, padx=5)

        paned = ttk.PanedWindow(self.root, orient=tk.HORIZONTAL)
        paned.pack(fill=tk.BOTH, expand=True, padx=10, pady=10)

        table_frame = ttk.Frame(paned)
        paned.add(table_frame, weight=1)

        ttk.Label(table_frame, text="Variation Series").pack(anchor=tk.W, pady=2)

        cols = ("val", "count", "rel")
        self.tree = ttk.Treeview(table_frame, columns=cols, show="headings")
        self.tree.heading("val", text="Variant (x_i)")
        self.tree.heading("count", text="Freq (n_i)")
        self.tree.heading("rel", text="Rel. Freq (w_i)")

        self.tree.column("val", width=90, anchor=tk.CENTER)
        self.tree.column("count", width=80, anchor=tk.CENTER)
        self.tree.column("rel", width=100, anchor=tk.CENTER)

        scrollbar = ttk.Scrollbar(table_frame, orient=tk.VERTICAL, command=self.tree.yview)
        self.tree.configure(yscroll=scrollbar.set)

        self.tree.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        scrollbar.pack(side=tk.RIGHT, fill=tk.Y)

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

        series = build_variation_series(self.raw_data)
        for row in series:
            self.tree.insert("", tk.END, values=(
                f"{row['value']:.4g}",
                row['count'],
                f"{row['relative_freq']:.4f}",
            ))

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
