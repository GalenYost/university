import math
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg
import tkinter as tk
from tkinter import filedialog, messagebox, ttk
import scipy.stats as st

from .calculations import (
    load_paired_dataset, compute_differences, calculate_sturges_bins,
    build_histogram_data, calculate_base_statistics,
    calculate_confidence_intervals, calculate_bootstrap_intervals,
    paired_t_test, wilcoxon_signed_rank_test, check_normality,
    analyze_central_tendencies
)

class StatsApp:
    def __init__(self, root: tk.Tk):
        self.root = root
        self.root.title("Лабораторна робота №4: Порівняння двох залежних вибірок")
        self.root.geometry("1400x900")
        
        self.x_data: list[float] = []
        self.y_data: list[float] = []
        self.d_data: list[float] = []
        self.current_file_path: str = ""
        
        self._build_ui()

    def _build_ui(self):
        # Верхня панель інструментів
        top_frame = ttk.Frame(self.root, padding=8)
        top_frame.pack(fill=tk.X, side=tk.TOP)

        ttk.Button(top_frame, text="Відкрити файл...", command=self.open_file_dialog).pack(side=tk.LEFT, padx=5)
        self.file_label = ttk.Label(top_frame, text="Файл не вибрано", font=("sans-serif", 9, "bold"))
        self.file_label.pack(side=tk.LEFT, padx=10)

        ttk.Separator(top_frame, orient=tk.VERTICAL).pack(side=tk.LEFT, fill=tk.Y, padx=10)

        ttk.Label(top_frame, text="Рівень значущості α:").pack(side=tk.LEFT, padx=(5, 2))
        self.conf_var = tk.StringVar(value="α = 0.05 (95%)")
        self.conf_cb = ttk.Combobox(
            top_frame, textvariable=self.conf_var,
            values=["α = 0.05 (95%)", "α = 0.01 (99%)", "α = 0.10 (90%)"],
            width=15, state="readonly"
        )
        self.conf_cb.pack(side=tk.LEFT, padx=5)
        self.conf_cb.bind("<<ComboboxSelected>>", lambda e: self.render_all())

        ttk.Label(top_frame, text="Кількість інтервалів (Bins):").pack(side=tk.LEFT, padx=(15, 2))
        self.bin_slider = ttk.Scale(top_frame, from_=1, to=20, orient=tk.HORIZONTAL, command=self.on_bin_slider_move)
        self.bin_slider.pack(side=tk.LEFT, padx=5)
        self.bin_val_label = ttk.Label(top_frame, text="1")
        self.bin_val_label.pack(side=tk.LEFT, padx=5)

        ttk.Separator(top_frame, orient=tk.VERTICAL).pack(side=tk.LEFT, fill=tk.Y, padx=10)
        
        ttk.Button(top_frame, text="Скопіювати весь звіт у буфер", command=self.copy_full_report).pack(side=tk.RIGHT, padx=5)

        # Notebook з вкладками
        self.notebook = ttk.Notebook(self.root)
        self.notebook.pack(fill=tk.BOTH, expand=True, padx=10, pady=5)

        # Вкладка 1: Основні результати та порівняння критеріїв
        self.tab_summary = ttk.Frame(self.notebook, padding=8)
        self.notebook.add(self.tab_summary, text="Основні результати")
        self._build_summary_tab()

        # Вкладка 2: Таблиця 11 (Парний t-критерій Стьюдента)
        self.tab_t_test = ttk.Frame(self.notebook, padding=8)
        self.notebook.add(self.tab_t_test, text="Таблиця 11 (t-критерій)")
        self._build_t_test_tab()

        # Вкладка 3: Таблиця 12 (Критерій рангів Вілкоксона)
        self.tab_wilcoxon = ttk.Frame(self.notebook, padding=8)
        self.notebook.add(self.tab_wilcoxon, text="Таблиця 12 (Вілкоксон)")
        self._build_wilcoxon_tab()

        # Вкладка 4: Графіки
        self.tab_charts = ttk.Frame(self.notebook, padding=8)
        self.notebook.add(self.tab_charts, text="Графіки")
        self._build_charts_tab()

        # Вкладка 5: Текст звіту
        self.tab_report = ttk.Frame(self.notebook, padding=8)
        self.notebook.add(self.tab_report, text="Готовий звіт")
        self._build_report_tab()

    def _build_summary_tab(self):
        paned = ttk.PanedWindow(self.tab_summary, orient=tk.HORIZONTAL)
        paned.pack(fill=tk.BOTH, expand=True)

        left_frame = ttk.Frame(paned)
        paned.add(left_frame, weight=3)

        # 1. Таблиця описових характеристик
        self.stats_title_label = ttk.Label(left_frame, text="Статистичні характеристики вибірок та різниць:", font=("sans-serif", 10, "bold"))
        self.stats_title_label.pack(anchor=tk.W, pady=(0, 2))
        stat_frame = ttk.Frame(left_frame)
        stat_frame.pack(fill=tk.X, pady=(0, 10))
        
        self.stats_tree = ttk.Treeview(stat_frame, columns=("param", "x_val", "y_val", "d_val", "d_ci"), show="headings", height=7)
        self.stats_tree.heading("param", text="Характеристика")
        self.stats_tree.heading("x_val", text="Вибірка X")
        self.stats_tree.heading("y_val", text="Вибірка Y")
        self.stats_tree.heading("d_val", text="Вибірка різниць (D)")
        self.stats_tree.heading("d_ci", text="Довірчий інтервал для D")
        
        self.stats_tree.column("param", width=190, anchor=tk.W)
        self.stats_tree.column("x_val", width=110, anchor=tk.CENTER)
        self.stats_tree.column("y_val", width=110, anchor=tk.CENTER)
        self.stats_tree.column("d_val", width=130, anchor=tk.CENTER)
        self.stats_tree.column("d_ci", width=220, anchor=tk.CENTER)
        self.stats_tree.pack(fill=tk.X)

        # 2. Таблиця порівняння критеріїв
        ttk.Label(left_frame, text="Результати перевірки гіпотез про рівність вибірок:", font=("sans-serif", 10, "bold")).pack(anchor=tk.W, pady=(5, 2))
        hyp_frame = ttk.Frame(left_frame)
        hyp_frame.pack(fill=tk.X, pady=(0, 10))

        self.hyp_tree = ttk.Treeview(hyp_frame, columns=("test", "stat", "crit", "pval", "conclusion"), show="headings", height=3)
        self.hyp_tree.heading("test", text="Статистичний критерій")
        self.hyp_tree.heading("stat", text="Статистика")
        self.hyp_tree.heading("crit", text="Критичне значення")
        self.hyp_tree.heading("pval", text="p-значення")
        self.hyp_tree.heading("conclusion", text="Статистичне рішення (H0)")

        self.hyp_tree.column("test", width=210, anchor=tk.W)
        self.hyp_tree.column("stat", width=110, anchor=tk.CENTER)
        self.hyp_tree.column("crit", width=130, anchor=tk.CENTER)
        self.hyp_tree.column("pval", width=110, anchor=tk.CENTER)
        self.hyp_tree.column("conclusion", width=220, anchor=tk.W)
        self.hyp_tree.pack(fill=tk.X)

        # 3. Перевірка нормальності різниць
        ttk.Label(left_frame, text="Перевірка нормальності розподілу вибірки різниць D:", font=("sans-serif", 10, "bold")).pack(anchor=tk.W, pady=(5, 2))
        norm_frame = ttk.Frame(left_frame)
        norm_frame.pack(fill=tk.X, pady=(0, 10))

        self.norm_tree = ttk.Treeview(norm_frame, columns=("test", "stat", "crit", "conclusion"), show="headings", height=4)
        self.norm_tree.heading("test", text="Метод тестування нормальності")
        self.norm_tree.heading("stat", text="Значення статистики")
        self.norm_tree.heading("crit", text="Критерій / Поріг")
        self.norm_tree.heading("conclusion", text="Висновок")

        self.norm_tree.column("test", width=230, anchor=tk.W)
        self.norm_tree.column("stat", width=130, anchor=tk.CENTER)
        self.norm_tree.column("crit", width=130, anchor=tk.CENTER)
        self.norm_tree.column("conclusion", width=250, anchor=tk.W)
        self.norm_tree.pack(fill=tk.X)

        # Права панель: Аналітичні текстові висновки
        right_frame = ttk.Frame(paned, padding=5)
        paned.add(right_frame, weight=2)

        ttk.Label(right_frame, text="Аналіз центральних тенденцій та розбіжностей:", font=("sans-serif", 10, "bold")).pack(anchor=tk.W, pady=(0, 5))
        self.summary_text = tk.Text(right_frame, wrap=tk.WORD, font=("Courier", 10), padx=8, pady=8)
        self.summary_text.pack(fill=tk.BOTH, expand=True)

    def _build_t_test_tab(self):
        ttk.Label(
            self.tab_t_test,
            text="Таблиця 11. Допоміжні розрахунки для перевірки рівності середніх (парний t-критерій Стьюдента):",
            font=("sans-serif", 10, "bold")
        ).pack(anchor=tk.W, pady=(0, 5))

        btn_box = ttk.Frame(self.tab_t_test)
        btn_box.pack(fill=tk.X, pady=(0, 5))
        ttk.Button(btn_box, text="Скопіювати Таблицю 11 у буфер (TSV)", command=self.copy_tab11).pack(side=tk.LEFT)
        self.t_formula_label = ttk.Label(btn_box, text="", font=("sans-serif", 9))
        self.t_formula_label.pack(side=tk.LEFT, padx=20)

        t_frame = ttk.Frame(self.tab_t_test)
        t_frame.pack(fill=tk.BOTH, expand=True)

        cols = ("idx", "x", "y", "d", "dev", "dev_sq")
        self.tab11_tree = ttk.Treeview(t_frame, columns=cols, show="headings")
        self.tab11_tree.heading("idx", text="№ (i)")
        self.tab11_tree.heading("x", text="x_{1,i}")
        self.tab11_tree.heading("y", text="x_{2,i}")
        self.tab11_tree.heading("d", text="d_i = x_{1,i} - x_{2,i}")
        self.tab11_tree.heading("dev", text="d_i - d̄")
        self.tab11_tree.heading("dev_sq", text="(d_i - d̄)^2")

        for col in cols:
            self.tab11_tree.column(col, width=120, anchor=tk.CENTER)

        scroll_y = ttk.Scrollbar(t_frame, orient=tk.VERTICAL, command=self.tab11_tree.yview)
        self.tab11_tree.configure(yscroll=scroll_y.set)
        self.tab11_tree.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        scroll_y.pack(side=tk.RIGHT, fill=tk.Y)

    def _build_wilcoxon_tab(self):
        ttk.Label(
            self.tab_wilcoxon,
            text="Таблиця 12. Розрахунок статистики критерію знакових рангів Вілкоксона:",
            font=("sans-serif", 10, "bold")
        ).pack(anchor=tk.W, pady=(0, 5))

        btn_box = ttk.Frame(self.tab_wilcoxon)
        btn_box.pack(fill=tk.X, pady=(0, 5))
        ttk.Button(btn_box, text="Скопіювати Таблицю 12 у буфер (TSV)", command=self.copy_tab12).pack(side=tk.LEFT)
        self.w_formula_label = ttk.Label(btn_box, text="", font=("sans-serif", 9))
        self.w_formula_label.pack(side=tk.LEFT, padx=20)

        paned = ttk.PanedWindow(self.tab_wilcoxon, orient=tk.VERTICAL)
        paned.pack(fill=tk.BOTH, expand=True)

        top_w_frame = ttk.Frame(paned)
        paned.add(top_w_frame, weight=3)

        cols = ("idx", "x", "y", "d", "abs_d", "rank", "r_pos", "r_neg")
        self.tab12_tree = ttk.Treeview(top_w_frame, columns=cols, show="headings")
        self.tab12_tree.heading("idx", text="№ (i)")
        self.tab12_tree.heading("x", text="x_{1,i}")
        self.tab12_tree.heading("y", text="x_{2,i}")
        self.tab12_tree.heading("d", text="d_i = x_{1,i} - x_{2,i}")
        self.tab12_tree.heading("abs_d", text="|d_i|")
        self.tab12_tree.heading("rank", text="r_i - ранг |d_i|")
        self.tab12_tree.heading("r_pos", text="r_i для d_i > 0")
        self.tab12_tree.heading("r_neg", text="r_i для d_i < 0")

        for col in cols:
            self.tab12_tree.column(col, width=110, anchor=tk.CENTER)

        scroll_y = ttk.Scrollbar(top_w_frame, orient=tk.VERTICAL, command=self.tab12_tree.yview)
        self.tab12_tree.configure(yscroll=scroll_y.set)
        self.tab12_tree.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        scroll_y.pack(side=tk.RIGHT, fill=tk.Y)

        bottom_w_frame = ttk.Frame(paned)
        paned.add(bottom_w_frame, weight=2)
        ttk.Label(bottom_w_frame, text="Таблиця 13. Визначення рангів для впорядкованих |d_i|:", font=("sans-serif", 9, "bold")).pack(anchor=tk.W, pady=(5, 2))

        cols13 = ("order", "abs_d", "rank")
        self.tab13_tree = ttk.Treeview(bottom_w_frame, columns=cols13, show="headings", height=5)
        self.tab13_tree.heading("order", text="Порядковий номер")
        self.tab13_tree.heading("abs_d", text="Впорядковані |d_i|")
        self.tab13_tree.heading("rank", text="Присвоєний ранг r_i")
        for col in cols13:
            self.tab13_tree.column(col, width=120, anchor=tk.CENTER)
        self.tab13_tree.pack(fill=tk.BOTH, expand=True)

    def _build_charts_tab(self):
        self.fig, (self.ax1, self.ax2) = plt.subplots(1, 2, figsize=(11, 4.5))
        self.canvas = FigureCanvasTkAgg(self.fig, master=self.tab_charts)
        self.canvas.get_tk_widget().pack(fill=tk.BOTH, expand=True)

    def _build_report_tab(self):
        btn_box = ttk.Frame(self.tab_report)
        btn_box.pack(fill=tk.X, pady=(0, 5))
        ttk.Button(btn_box, text="Скопіювати повний текст у буфер", command=self.copy_full_report).pack(side=tk.LEFT)
        ttk.Label(btn_box, text="Цей текст відповідає структурі розділів звіту за методичкою (Розділи 3 та 4).").pack(side=tk.LEFT, padx=15)

        self.report_text = tk.Text(self.tab_report, wrap=tk.WORD, font=("Courier", 10), padx=10, pady=10)
        scroll_rep = ttk.Scrollbar(self.tab_report, orient=tk.VERTICAL, command=self.report_text.yview)
        self.report_text.configure(yscroll=scroll_rep.set)
        self.report_text.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        scroll_rep.pack(side=tk.RIGHT, fill=tk.Y)

    def _get_alpha(self) -> float:
        conf_str = self.conf_var.get()
        if "0.01" in conf_str:
            return 0.01
        elif "0.10" in conf_str:
            return 0.10
        return 0.05

    def on_bin_slider_move(self, val):
        self.bin_val_label.config(text=str(int(float(val))))
        if self.d_data:
            self.draw_charts(int(float(val)))

    def load_data_from_path(self, file_path: str):
        try:
            x, y = load_paired_dataset(file_path)
            self.x_data = x
            self.y_data = y
            self.d_data = compute_differences(x, y)
            self.current_file_path = file_path
            
            n = len(self.x_data)
            fname = file_path.split("/")[-1]
            self.file_label.config(text=f"{fname} (N = {n} пар)")
            
            default_bins = calculate_sturges_bins(n)
            self.bin_slider.set(default_bins)
            self.bin_val_label.config(text=str(default_bins))
            
            self.render_all()
        except Exception as err:
            messagebox.showerror("Помилка", f"Не вдалося прочитати файл:\n{err}")

    def open_file_dialog(self):
        file_path = filedialog.askopenfilename(
            filetypes=[("Текстові файли", "*.txt"), ("CSV файли", "*.csv"), ("Всі файли", "*.*")]
        )
        if file_path:
            self.load_data_from_path(file_path)

    def render_all(self):
        if not self.x_data or not self.y_data:
            return

        alpha = self._get_alpha()
        conf_level = 1.0 - alpha
        conf_pct = int(round(conf_level * 100))

        # Оновлення заголовка та назви колонки відповідно до обраного рівня довіри
        self.stats_title_label.config(text=f"Статистичні характеристики вибірок та різниць з {conf_pct}% ДІ:")
        self.stats_tree.heading("d_ci", text=f"{conf_pct}% Довірчий інтервал для D")

        # 1. Базові описові характеристики для X, Y, D
        stats_x = calculate_base_statistics(self.x_data)
        stats_y = calculate_base_statistics(self.y_data)
        stats_d = calculate_base_statistics(self.d_data)

        ci_x = calculate_confidence_intervals(self.x_data, stats_x, conf_level)
        ci_y = calculate_confidence_intervals(self.y_data, stats_y, conf_level)
        ci_d = calculate_confidence_intervals(self.d_data, stats_d, conf_level)

        # Очищення дерева описових характеристик
        self.stats_tree.delete(*self.stats_tree.get_children())
        rows = [
            ("Обсяг вибірки (N)", f"{stats_x['n']}", f"{stats_y['n']}", f"{stats_d['n']}", "-"),
            ("Середнє арифметичне", f"{stats_x['mean']:.4f}", f"{stats_y['mean']:.4f}", f"{stats_d['mean']:.4f}", f"[{ci_d['mean_ci'][0]:.4f}; {ci_d['mean_ci'][1]:.4f}]"),
            ("Медіана", f"{stats_x['median']:.4f}", f"{stats_y['median']:.4f}", f"{stats_d['median']:.4f}", f"[{ci_d['median_ci'][0]:.4f}; {ci_d['median_ci'][1]:.4f}]"),
            ("Незміщене СКВ (S)", f"{stats_x['s_unbias']:.4f}", f"{stats_y['s_unbias']:.4f}", f"{stats_d['s_unbias']:.4f}", f"[{ci_d['s_unbias_ci'][0]:.4f}; {ci_d['s_unbias_ci'][1]:.4f}]"),
            ("Асиметрія (A2)", f"{stats_x['skewness_a2']:.4f}", f"{stats_y['skewness_a2']:.4f}", f"{stats_d['skewness_a2']:.4f}", f"[{ci_d['skewness_ci'][0]:.4f}; {ci_d['skewness_ci'][1]:.4f}]"),
            ("Ексцес (E2)", f"{stats_x['kurtosis_e2']:.4f}", f"{stats_y['kurtosis_e2']:.4f}", f"{stats_d['kurtosis_e2']:.4f}", f"[{ci_d['kurtosis_ci'][0]:.4f}; {ci_d['kurtosis_ci'][1]:.4f}]"),
        ]
        for r in rows:
            self.stats_tree.insert("", tk.END, values=r)

        # 2. Перевірка гіпотез (t-критерій та Вілкоксон)
        t_res = paired_t_test(self.x_data, self.y_data, alpha)
        w_res = wilcoxon_signed_rank_test(self.x_data, self.y_data, alpha)

        self.hyp_tree.delete(*self.hyp_tree.get_children())
        
        t_decision = "H0 відхиляється (різниця є)" if t_res["is_significant"] else "H0 приймається (різниці немає)"
        self.hyp_tree.insert("", tk.END, values=(
            "Парний t-критерій Стьюдента",
            f"t = {t_res['t_stat']:.4f}",
            f"t_crit = ±{t_res['t_crit']:.4f} (df={t_res['df']})",
            f"p = {t_res['p_value']:.4f}",
            t_decision
        ))

        w_decision = "H0 відхиляється (зсув є)" if w_res["is_significant"] else "H0 приймається (зсуву немає)"
        if w_res["t_crit_table"] is not None and abs(alpha - 0.05) < 1e-4:
            w_crit_str = f"T_крит = {w_res['t_crit_table']} (табл)"
        else:
            w_crit_str = f"u_crit = ±{w_res['u_crit']:.2f}"
            
        self.hyp_tree.insert("", tk.END, values=(
            "Критерій знакових рангів Вілкоксона",
            f"T = {w_res['t_stat']:.1f} (T+={w_res['t_plus']:.1f}, T-={w_res['t_minus']:.1f})",
            w_crit_str,
            f"p = {w_res['p_value_norm']:.4f}",
            w_decision
        ))

        # 3. Перевірка нормальності розподілу різниць D
        self.norm_tree.delete(*self.norm_tree.get_children())
        norm_results = check_normality(self.d_data, alpha)
        for nr in norm_results:
            self.norm_tree.insert("", tk.END, values=(nr["test"], nr["stat"], nr["crit"], nr["conclusion"]))

        # 4. Текстовий аналіз та центральні тенденції
        ct_analysis = analyze_central_tendencies(stats_d, ci_d, conf_level)
        
        # Порівняльний коментар
        both_agree = (t_res["is_significant"] == w_res["is_significant"])
        is_normal = any("Нормальний" in nr["conclusion"] for nr in norm_results if "Шапіро" in nr["test"])
        
        comparison_comment = (
            "\nПОРІВНЯННЯ КРИТЕРІЇВ ТА РЕКОМЕНДАЦІЇ:\n"
            f"• Узгодженість результатів: {'Критерії ДАЮТЬ УЗГОДЖЕНИЙ висновок.' if both_agree else 'Критерії РОЗХОДЯТЬСЯ у висновках!'}\n"
            f"• Перевірка нормальності: {'Нормальний розподіл різниць ПІДТВЕРДЖЕНО.' if is_normal else 'Розподіл різниць НЕ Є нормальним (або є помітні викиди).'}\n"
            f"• Рекомендація: {'Оскільки розподіл різниць нормальний, парний t-критерій є більш потужним і пріоритетним.' if is_normal else 'Оскільки припущення про нормальність порушено, слід віддати перевагу критерію знакових рангів Вілкоксона як більш надійному.'}\n"
        )
        
        self.summary_text.delete("1.0", tk.END)
        self.summary_text.insert(tk.END, "АНАЛІЗ ЦЕНТРАЛЬНИХ ТЕНДЕНЦІЙ РІЗНИЦЬ:\n" + ct_analysis + "\n" + comparison_comment)

        # 5. Заповнення Таблиці 11
        self.tab11_tree.delete(*self.tab11_tree.get_children())
        for row in t_res["table_rows"]:
            self.tab11_tree.insert("", tk.END, values=(
                row["idx"], f"{row['x']:.4g}", f"{row['y']:.4g}",
                f"{row['d']:.4f}", f"{row['dev']:.4f}", f"{row['dev_sq']:.4f}"
            ))
        # Підсумковий рядок Таблиці 11
        self.tab11_tree.insert("", tk.END, values=(
            "Разом ∑", "-", "-", f"{t_res['sum_diff']:.4f}", "-", f"{t_res['sum_dev_sq']:.4f}"
        ), tags=("total",))
        self.tab11_tree.tag_configure("total", background="#f0f0f0", font=("sans-serif", 9, "bold"))
        self.t_formula_label.config(
            text=f"d̄ = {t_res['mean_diff']:.4f},  Sd = {t_res['sd_diff']:.4f},  t = {t_res['t_stat']:.4f},  df = {t_res['df']},  p = {t_res['p_value']:.4f}"
        )

        # 6. Заповнення Таблиці 12
        self.tab12_tree.delete(*self.tab12_tree.get_children())
        for row in w_res["table_rows"]:
            self.tab12_tree.insert("", tk.END, values=(
                row["idx"], f"{row['x']:.4g}", f"{row['y']:.4g}",
                f"{row['d']:.4f}", f"{row['abs_d']:.4f}",
                f"{row['rank']:.1f}",
                f"{row['r_pos']:.1f}" if row['r_pos'] > 0 else "-",
                f"{row['r_neg']:.1f}" if row['r_neg'] > 0 else "-"
            ))
        self.tab12_tree.insert("", tk.END, values=(
            "Разом", "-", "-", "-", "-", "-",
            f"T+ = {w_res['t_plus']:.1f}", f"T- = {w_res['t_minus']:.1f}"
        ), tags=("total",))
        self.tab12_tree.tag_configure("total", background="#f0f0f0", font=("sans-serif", 9, "bold"))

        crit_info = f"T_крит = {w_res['t_crit_table']}" if w_res['t_crit_table'] is not None else f"u_crit = {w_res['u_crit']:.2f}"
        self.w_formula_label.config(
            text=f"N' = {w_res['n_nonzero']},  T = min(T+, T-) = {w_res['t_stat']:.1f},  {crit_info},  E{{T}} = {w_res['e_t']:.1f},  D{{T}} = {w_res['d_t']:.2f},  u = {w_res['u_stat']:.2f},  p = {w_res['p_value_norm']:.4f}"
        )

        # 7. Заповнення Таблиці 13
        self.tab13_tree.delete(*self.tab13_tree.get_children())
        for rk in w_res["table_ranking"]:
            self.tab13_tree.insert("", tk.END, values=(rk["order"], f"{rk['abs_d']:.4f}", f"{rk['rank']:.1f}"))

        # 8. Графіки
        self.draw_charts(int(self.bin_slider.get()))

        # 9. Формування тексту готового звіту
        self._generate_report_text(stats_x, stats_y, stats_d, ci_d, t_res, w_res, norm_results, conf_level)

    def draw_charts(self, num_bins: int):
        self.ax1.clear()
        self.ax2.clear()

        if not self.d_data:
            return

        # Графік 1: Гістограма різниць D з нормальною кривою
        bins = build_histogram_data(self.d_data, num_bins)
        centers = [(b["start"] + b["end"]) / 2.0 for b in bins]
        heights = [b["relative_freq"] for b in bins]
        widths = [b["end"] - b["start"] for b in bins]

        self.ax1.bar(centers, heights, width=widths, edgecolor="black", color="#4a90e2", alpha=0.6, label="Емпіричні частоти")
        
        stats_d = calculate_base_statistics(self.d_data)
        d_min, d_max = min(self.d_data), max(self.d_data)
        span = max(1.0, d_max - d_min)
        x_vals = np.linspace(d_min - 0.3 * span, d_max + 0.3 * span, 150)
        
        step = widths[0] if widths else 1.0
        if stats_d["s_unbias"] > 1e-6:
            pdf_vals = st.norm.pdf(x_vals, stats_d["mean"], stats_d["s_unbias"]) * step
            self.ax1.plot(x_vals, pdf_vals, color="red", linewidth=2, label="Нормальна щільність")

        self.ax1.axvline(stats_d["mean"], color="darkgreen", linestyle="--", linewidth=1.8, label=f"Середнє ({stats_d['mean']:.2f})")
        self.ax1.axvline(stats_d["median"], color="purple", linestyle=":", linewidth=1.8, label=f"Медіана ({stats_d['median']:.2f})")
        self.ax1.axvline(0, color="gray", linestyle="-", linewidth=1.0, alpha=0.7, label="d = 0")

        self.ax1.set_title("Розподіл різниць D = X - Y")
        self.ax1.set_xlabel("Значення різниці d_i")
        self.ax1.set_ylabel("Відносна частота")
        self.ax1.legend(fontsize=8)
        self.ax1.grid(True, linestyle="--", alpha=0.5)

        # Графік 2: Порівняння парних спостережень X vs Y (Scatter + лінія рівності y = x)
        self.ax2.scatter(self.x_data, self.y_data, color="#d9534f", edgecolors="black", s=60, zorder=3, label="Спостереження (x_i, y_i)")
        
        all_vals = self.x_data + self.y_data
        min_xy, max_xy = min(all_vals), max(all_vals)
        pad = (max_xy - min_xy) * 0.08 if max_xy > min_xy else 1.0
        diag_x = np.linspace(min_xy - pad, max_xy + pad, 50)
        self.ax2.plot(diag_x, diag_x, color="black", linestyle="--", linewidth=1.5, label="Лінія рівності (y = x)")

        # З'єднувальні лінії між парами для наочності
        for idx, (xi, yi) in enumerate(zip(self.x_data, self.y_data)):
            self.ax2.annotate(str(idx + 1), (xi, yi), textcoords="offset points", xytext=(5, 5), fontsize=8)

        self.ax2.set_title("Парна діаграма розсіювання (X проти Y)")
        self.ax2.set_xlabel("Показник X (алгоритм 1 / до)")
        self.ax2.set_ylabel("Показник Y (алгоритм 2 / після)")
        self.ax2.legend(fontsize=8)
        self.ax2.grid(True, linestyle="--", alpha=0.5)

        self.fig.tight_layout()
        self.canvas.draw()

    def _generate_report_text(self, sx, sy, sd, ci_d, t_res, w_res, norm_res, conf_level: float = 0.95):
        alpha = self._get_alpha()
        conf_pct = int(round(conf_level * 100))
        mean_ci = ci_d["mean_ci"]
        med_ci = ci_d["median_ci"]
        
        is_shap_norm = any(nr["is_normal"] for nr in norm_res if "Шапіро" in nr["test"])
        
        lines = []
        lines.append("РОЗДІЛ 3. РЕЗУЛЬТАТИ АНАЛІЗУ\n")
        lines.append("1. Обґрунтування залежності вибірок:")
        lines.append("   Вибірки є залежними (парними), оскільки вони одержані в результаті вимірювань")
        lines.append("   характеристик над одними й тими самими об'єктами (парні спостереження за однакових умов).")
        lines.append(f"   Обсяг кожної вибірки однаковий і становить N = {sx['n']} пар.\n")
        
        lines.append("2. Статистичні характеристики вибірок та різниць:")
        lines.append(f"    Вибірка X:  Середнє = {sx['mean']:.4f}, Медіана = {sx['median']:.4f}, S = {sx['s_unbias']:.4f}, A2 = {sx['skewness_a2']:.4f}, E2 = {sx['kurtosis_e2']:.4f}")
        lines.append(f"    Вибірка Y:  Середнє = {sy['mean']:.4f}, Медіана = {sy['median']:.4f}, S = {sy['s_unbias']:.4f}, A2 = {sy['skewness_a2']:.4f}, E2 = {sy['kurtosis_e2']:.4f}")
        lines.append(f"    Вибірка різниць D:  Середнє = {sd['mean']:.4f}, Медіана = {sd['median']:.4f}, S = {sd['s_unbias']:.4f}, A2 = {sd['skewness_a2']:.4f}, E2 = {sd['kurtosis_e2']:.4f}\n")
        
        lines.append(f"3. Аналіз центральних тенденцій за вибіркою різниць ({conf_pct}% довірчі інтервали):")
        lines.append(f"    • {conf_pct}% ДІ для середнього різниць: [{mean_ci[0]:.4f}; {mean_ci[1]:.4f}]")
        lines.append(f"    • {conf_pct}% ДІ для медіани різниць:   [{med_ci[0]:.4f}; {med_ci[1]:.4f}]")
        has_zero = (mean_ci[0] <= 0 <= mean_ci[1]) and (med_ci[0] <= 0 <= med_ci[1])
        if has_zero:
            lines.append("   Висновок: Обидва довірчі інтервали містять 0, що є попередньою оцінкою відсутності")
            lines.append("   істотної різниці між центральними тенденціями обох вибірок.\n")
        else:
            lines.append("   Висновок: Довірчі інтервали не містять 0 (або спостерігається зсув), що вказує на наявність")
            lines.append("   систематичної різниці між показниками.\n")
            
        lines.append("4. Перевірка вибірки різниць на нормальність:")
        for nr in norm_res:
            lines.append(f"   • {nr['test']}: {nr['stat']}, {nr['crit']} => {nr['conclusion']}")
        lines.append(f"   Підсумок: Розподіл різниць {'можна вважати нормальним' if is_shap_norm else 'НЕ є нормальним (наявні сильні відхилення/викиди)'}.\n")
        
        lines.append("5. Результати застосування статистичних критеріїв:")
        lines.append("   а) Парний t-критерій Стьюдента:")
        lines.append(f"        Статистика: t = {t_res['t_stat']:.4f}, df = {t_res['df']}")
        lines.append(f"        Критичне значення: t_crit = {t_res['t_crit']:.4f} (при alpha = {alpha})")
        lines.append(f"        p-значення: p = {t_res['p_value']:.4f}")
        lines.append(f"        Висновок: {t_res['conclusion']}\n")
        
        lines.append("   б) Критерій знакових рангів Вілкоксона:")
        lines.append(f"        Кількість ненульових різниць N' = {w_res['n_nonzero']}")
        lines.append(f"        Суми рангів: T+ = {w_res['t_plus']:.1f}, T- = {w_res['t_minus']:.1f}")
        lines.append(f"        Статистика: T = min(T+, T-) = {w_res['t_stat']:.1f}")
        if w_res["t_crit_table"] is not None:
            lines.append(f"        Табличне критичне значення T_крит = {w_res['t_crit_table']} (за табл. В.1 при alpha=0.05)")
        lines.append(f"        Нормальна апроксимація: E{{T}} = {w_res['e_t']:.1f}, D{{T}} = {w_res['d_t']:.2f}, u = {w_res['u_stat']:.2f}, u_crit = {w_res['u_crit']:.2f}")
        lines.append(f"        p-значення: p = {w_res['p_value_norm']:.4f} (точне p = {w_res['p_value_exact']:.4f})")
        lines.append(f"        Висновок: {w_res['conclusion']}\n")
        
        lines.append("6. Порівняння результатів t-критерію та критерію Вілкоксона:")
        agree = (t_res["is_significant"] == w_res["is_significant"])
        lines.append(f"     Чи узгоджуються вони: {'Так, обидва критерії призводять до однакового висновку.' if agree else 'Ні, критерії розходяться через специфіку розподілу різниць.'}")
        if not is_shap_norm:
            lines.append("     Який краще застосовувати: Оскільки припущення про нормальність розподілу різниць")
            lines.append("     порушено (Shapiro-Wilk p < 0.05), застосування параметричного t-критерію є некоректним.")
            lines.append("     Перевагу слід однозначно віддати непараметричному критерію знакових рангів Вілкоксона.")
        else:
            lines.append("     Який краще застосовувати: Оскільки розподіл різниць є нормальним, параметричний t-критерій")
            lines.append("     Стьюдента є більш потужним і його результати є визначальними.")
        lines.append("\n7. Відповідь на запитання задачі:")
        lines.append(f"   {w_res['conclusion'] if not is_shap_norm else t_res['conclusion']}")
        lines.append("\n--------------------------------------------------------------------------------")
        lines.append("РОЗДІЛ 4. ВИСНОВКИ\n")
        lines.append("1. Що таке залежні вибірки:")
        lines.append("   Залежні (парні) вибірки — це сукупності даних, одержані в результаті спостереження одних і")
        lines.append("   тих самих об'єктів (явищ, процесів), але в різний час, за різних умов чи різними методами.")
        lines.append("   Їхньою характерною рисою є однакова розмірність та наявність попарного зв'язку між елементами.\n")
        lines.append("2. Параметричні та непараметричні критерії:")
        lines.append("     Парний t-критерій Стьюдента є параметричним: він перевіряє рівність середніх значень і")
        lines.append("     строго вимагає виконання припущення про нормальний розподіл різниць між спостереженнями.")
        lines.append("     Критерій знакових рангів Вілкоксона є непараметричним: він спирається на ранжування")
        lines.append("     абсолютних різниць і перевіряє наявність систематичного зсуву розподілів без вимоги нормальності.\n")
        lines.append("3. Коли віддавати перевагу тим чи іншим критеріям на практиці:")
        lines.append("     Якщо різниці між парними спостереженнями мають нормальний розподіл і вибірка достатнього")
        lines.append("     обсягу — перевагу віддають t-критерію Стьюдента як більш потужному (більша ймовірність виявити відмінність).")
        lines.append("     Якщо розподіл різниць істотно відхиляється від нормального, вибірка мала (N <= 15) або")
        lines.append("     наявні суттєві викиди — перевагу обов'язково слід віддавати критерію Вілкоксона.")
        lines.append("================================================================================")
        
        full_text = "\n".join(lines)
        self.report_text.delete("1.0", tk.END)
        self.report_text.insert(tk.END, full_text)

    def copy_tab11(self):
        items = self.tab11_tree.get_children()
        if not items: return
        tsv_lines = ["№\tx1,i\tx2,i\td_i\td_i - d̄\t(d_i - d̄)^2"]
        for item in items:
            vals = self.tab11_tree.item(item, "values")
            tsv_lines.append("\t".join(str(v) for v in vals))
        tsv_text = "\n".join(tsv_lines)
        self.root.clipboard_clear()
        self.root.clipboard_append(tsv_text)
        messagebox.showinfo("Скопійовано", "Таблицю 11 скопійовано у буфер обміну (формат для Excel/Word)!")

    def copy_tab12(self):
        items = self.tab12_tree.get_children()
        if not items: return
        tsv_lines = ["№\tx1,i\tx2,i\td_i\t|d_i|\tr_i\tr_i (d>0)\tr_i (d<0)"]
        for item in items:
            vals = self.tab12_tree.item(item, "values")
            tsv_lines.append("\t".join(str(v) for v in vals))
        tsv_text = "\n".join(tsv_lines)
        self.root.clipboard_clear()
        self.root.clipboard_append(tsv_text)
        messagebox.showinfo("Скопійовано", "Таблицю 12 скопійовано у буфер обміну (формат для Excel/Word)!")

    def copy_full_report(self):
        text = self.report_text.get("1.0", tk.END).strip()
        if not text:
            messagebox.showwarning("Попередження", "Спочатку завантажте дані для розрахунку.")
            return
        self.root.clipboard_clear()
        self.root.clipboard_append(text)
        messagebox.showinfo("Скопійовано", "Повний текст звіту скопійовано в буфер обміну!")

