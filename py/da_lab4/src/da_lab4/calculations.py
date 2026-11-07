import math
import random
import scipy.stats as st

# Таблиця В.1 з методичних вказівок: Критичні значення T_крит для критерію знакових рангів Вілкоксона
# для alpha = 0.05 та двосторонньої альтернативи залежно від N'
WILCOXON_CRITICAL_TABLE_005 = {
    5: None, 6: 0, 7: 2, 8: 3, 9: 5, 10: 8,
    11: 10, 12: 13, 13: 17, 14: 21, 15: 25, 16: 29,
    17: 34, 18: 40, 19: 46, 20: 52, 21: 58, 22: 65,
    23: 73, 24: 81, 25: 89, 26: 98, 27: 107, 28: 116,
    29: 126, 30: 137, 31: 147, 32: 159, 33: 170, 34: 182,
    35: 195, 36: 208, 37: 221, 38: 235, 39: 249, 40: 264,
    41: 279, 42: 294, 43: 310, 44: 327, 45: 343, 46: 361,
    47: 378, 48: 396, 49: 415, 50: 434
}

def parse_paired_line(line: str) -> tuple[float, float] | None:
    line = line.strip()
    if not line or line.startswith("#"):
        return None
    # 1. Розділювач крапка з комою ';'
    if ";" in line:
        parts = [p.strip().replace(",", ".") for p in line.split(";") if p.strip()]
        if len(parts) >= 2:
            try:
                return float(parts[0]), float(parts[1])
            except ValueError:
                pass
    # 2. Розділювач таб '\t'
    if "\t" in line:
        parts = [p.strip().replace(",", ".") for p in line.split("\t") if p.strip()]
        if len(parts) >= 2:
            try:
                return float(parts[0]), float(parts[1])
            except ValueError:
                pass
    # 3. Кома як розділювач стовпців при наявності десяткових крапок (CSV: "8.5, 9.6")
    if "," in line and "." in line:
        parts = [p.strip() for p in line.split(",") if p.strip()]
        if len(parts) >= 2:
            try:
                return float(parts[0]), float(parts[1])
            except ValueError:
                pass
    # 4. Розділювач пробіл
    tokens = line.split()
    if len(tokens) == 2:
        clean_tokens = [t.rstrip(",;").replace(",", ".") for t in tokens]
        try:
            return float(clean_tokens[0]), float(clean_tokens[1])
        except ValueError:
            pass
    # 5. Кома без десяткових крапок (наприклад "10, 20")
    if "," in line:
        parts = [p.strip() for p in line.split(",") if p.strip()]
        if len(parts) == 2:
            try:
                return float(parts[0]), float(parts[1])
            except ValueError:
                pass
    return None

def load_paired_dataset(file_path: str) -> tuple[list[float], list[float]]:
    x, y = [], []
    with open(file_path, "r", encoding="utf-8") as f:
        for line in f:
            pair = parse_paired_line(line)
            if pair is not None:
                x.append(pair[0])
                y.append(pair[1])
    if not x:
        raise ValueError("Файл не містить числових даних або неправильний формат (очікується 2 стовпці).")
    return x, y

def compute_differences(x: list[float], y: list[float]) -> list[float]:
    if len(x) != len(y):
        raise ValueError("Розміри вибірок не співпадають.")
    return [round(a - b, 9) for a, b in zip(x, y)]

def calculate_sturges_bins(n: int) -> int:
    if n <= 0:
        return 1
    return max(1, int(round(1 + math.log2(n))))

def build_histogram_data(data: list[float], num_bins: int) -> list[dict]:
    if not data or num_bins <= 0:
        return []
    min_val, max_val = min(data), max(data)
    if abs(max_val - min_val) < 1e-9:
        return [{"start": min_val - 0.5, "end": max_val + 0.5, "count": len(data), "relative_freq": 1.0}]
    step = (max_val - min_val) / num_bins
    total = len(data)
    bins = [
        {"start": min_val + i * step, "end": max_val + 1e-9 if i == num_bins - 1 else min_val + (i + 1) * step, "count": 0, "relative_freq": 0.0}
        for i in range(num_bins)
    ]
    for val in data:
        idx = min(int((val - min_val) / step), num_bins - 1)
        bins[idx]["count"] += 1
    for b in bins:
        b["relative_freq"] = b["count"] / total
    return bins

def calculate_base_statistics(data: list[float]) -> dict:
    n = len(data)
    if n == 0:
        raise ValueError("Вибірка порожня.")
    mean = sum(data) / n
    sorted_data = sorted(data)
    median = sorted_data[n // 2] if n % 2 != 0 else (sorted_data[n // 2 - 1] + sorted_data[n // 2]) / 2.0
    
    if n == 1:
        return {
            "n": 1, "mean": mean, "median": median,
            "s_unbias": 0.0, "s_bias": 0.0,
            "skewness_a2": 0.0, "kurtosis_e2": 0.0
        }
    
    sum_sq = sum((val - mean) ** 2 for val in data)
    s_unbias = math.sqrt(sum_sq / (n - 1))
    s_bias = math.sqrt(sum_sq / n)
    
    if sum_sq < 1e-12 or n < 4:
        a2 = 0.0
        e2 = 0.0
    else:
        sum_cube = sum((val - mean) ** 3 for val in data)
        sum_quad = sum((val - mean) ** 4 for val in data)
        a1 = sum_cube / (n * (s_bias ** 3))
        a2 = (math.sqrt(n * (n - 1)) / (n - 2)) * a1
        e1 = sum_quad / (n * (s_bias ** 4)) - 3.0
        e2 = ((n ** 2 - 1) / ((n - 2) * (n - 3))) * (e1 + 6.0 / (n + 1))

    return {
        "n": n,
        "mean": mean,
        "median": median,
        "s_unbias": s_unbias,
        "s_bias": s_bias,
        "skewness_a2": a2,
        "kurtosis_e2": e2
    }

def calculate_confidence_intervals(data: list[float], stats: dict, confidence_level: float = 0.95) -> dict:
    n = stats["n"]
    if n < 2:
        m = stats["mean"]
        return {
            "mean_ci": (m, m), "median_ci": (stats["median"], stats["median"]),
            "s_unbias_ci": (0.0, 0.0), "skewness_ci": (0.0, 0.0), "kurtosis_ci": (0.0, 0.0)
        }
    
    alpha = 1.0 - confidence_level
    sorted_data = sorted(data)
    u_val = float(st.norm.ppf(1 - alpha / 2))
    t_val = float(st.t.ppf(1 - alpha / 2, df=n - 1))
    
    # CI для середнього (Стьюдент)
    se_mean = stats["s_unbias"] / math.sqrt(n)
    mean_ci = (stats["mean"] - t_val * se_mean, stats["mean"] + t_val * se_mean)
    
    # CI для медіани (ранговий метод)
    j_val = (n / 2.0) - u_val * (math.sqrt(n) / 2.0)
    k_val = (n / 2.0) + 1.0 + u_val * (math.sqrt(n) / 2.0)
    j_idx = max(0, min(n - 1, int(round(j_val)) - 1))
    k_idx = max(0, min(n - 1, int(round(k_val)) - 1))
    median_ci = (sorted_data[j_idx], sorted_data[k_idx])
    
    # CI для СКВ (хі-квадрат)
    if stats["s_unbias"] < 1e-12:
        s_ci = (0.0, 0.0)
    else:
        chi2_lower = float(st.chi2.ppf(1 - alpha / 2, df=n - 1))
        chi2_upper = float(st.chi2.ppf(alpha / 2, df=n - 1))
        s_sq_sum = (n - 1) * (stats["s_unbias"] ** 2)
        s_ci = (math.sqrt(s_sq_sum / chi2_lower), math.sqrt(s_sq_sum / chi2_upper))
    
    # CI для асиметрії та ексцесу
    if n >= 4 and stats["s_unbias"] >= 1e-12:
        se_skew = math.sqrt((6.0 * n * (n - 1)) / ((n - 2) * (n + 1) * (n + 3)))
        skew_ci = (stats["skewness_a2"] - u_val * se_skew, stats["skewness_a2"] + u_val * se_skew)
        
        se_kurt = math.sqrt((24.0 * n * (n - 1) ** 2) / ((n - 3) * (n - 2) * (n + 3) * (n + 5)))
        kurt_ci = (stats["kurtosis_e2"] - u_val * se_kurt, stats["kurtosis_e2"] + u_val * se_kurt)
    else:
        skew_ci = (stats["skewness_a2"], stats["skewness_a2"])
        kurt_ci = (stats["kurtosis_e2"], stats["kurtosis_e2"])

    return {
        "mean_ci": mean_ci,
        "median_ci": median_ci,
        "s_unbias_ci": s_ci,
        "skewness_ci": skew_ci,
        "kurtosis_ci": kurt_ci
    }

def calculate_bootstrap_intervals(data: list[float], confidence_level: float = 0.95, iterations: int = 1000) -> dict:
    n = len(data)
    if n < 2:
        m = data[0] if data else 0.0
        return {
            "mean_boot_ci": (m, m), "median_boot_ci": (m, m),
            "s_unbias_boot_ci": (0.0, 0.0), "skewness_boot_ci": (0.0, 0.0),
            "kurtosis_boot_ci": (0.0, 0.0)
        }
    means, medians, s_unbiases, skewnesses, kurtosises = [], [], [], [], []
    for _ in range(iterations):
        sample = random.choices(data, k=n)
        st_sample = calculate_base_statistics(sample)
        means.append(st_sample["mean"])
        medians.append(st_sample["median"])
        s_unbiases.append(st_sample["s_unbias"])
        skewnesses.append(st_sample["skewness_a2"])
        kurtosises.append(st_sample["kurtosis_e2"])
        
    means.sort(); medians.sort(); s_unbiases.sort(); skewnesses.sort(); kurtosises.sort()
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

def compute_ranks(values: list[float]) -> list[float]:
    """
    Власна реалізація розрахунку рангів (Таблиця 9 методичних вказівок).
    Ранг - це порядковий номер у впорядкованій за зростанням вибірці.
    Якщо є однакові елементи (в'язки), ранг дорівнює їх середньому арифметичному.
    """
    if not values:
        return []
    # Зберігаємо індекси елементів
    indexed = sorted(enumerate(values), key=lambda item: item[1])
    ranks = [0.0] * len(values)
    i = 0
    n = len(indexed)
    while i < n:
        j = i
        # Знаходимо групу однакових значень (в'язку)
        while j + 1 < n and abs(indexed[j + 1][1] - indexed[i][1]) < 1e-9:
            j += 1
        # Порядкові номери від (i + 1) до (j + 1)
        avg_rank = (i + 1 + j + 1) / 2.0
        for k in range(i, j + 1):
            ranks[indexed[k][0]] = avg_rank
        i = j + 1
    return ranks

def paired_t_test(x: list[float], y: list[float], alpha: float = 0.05) -> dict:
    """
    Власна реалізація парного t-критерію Стьюдента за розділом 2 методичних вказівок.
    """
    if len(x) != len(y):
        raise ValueError("Вибірки повинні мати однаковий розмір.")
    n = len(x)
    if n < 2:
        raise ValueError("Для парного t-критерію потрібно щонайменше 2 спостереження.")
    
    diffs = [round(a - b, 9) for a, b in zip(x, y)]
    mean_diff = sum(diffs) / n
    sum_sq_diff = sum((d - mean_diff) ** 2 for d in diffs)
    df = n - 1
    t_crit = float(st.t.ppf(1 - alpha / 2, df=df))
    
    # Формування допоміжних даних для Таблиці 11
    table_rows = []
    for i in range(n):
        dev = diffs[i] - mean_diff
        dev_sq = dev ** 2
        table_rows.append({
            "idx": i + 1,
            "x": x[i],
            "y": y[i],
            "d": diffs[i],
            "dev": dev,
            "dev_sq": dev_sq
        })

    # Випадок, коли вибірки повністю збігаються
    if sum_sq_diff < 1e-12:
        if abs(mean_diff) < 1e-12:
            return {
                "t_stat": 0.0,
                "df": df,
                "p_value": 1.0,
                "t_crit": t_crit,
                "mean_diff": 0.0,
                "sd_diff": 0.0,
                "se_diff": 0.0,
                "sum_diff": 0.0,
                "sum_dev_sq": 0.0,
                "alpha": alpha,
                "is_identical": True,
                "is_significant": False,
                "table_rows": table_rows,
                "conclusion": "Вибірки повністю збігаються (всі d_i = 0). H0 не відхиляється (p = 1.0000 >= alpha)."
            }
        else:
            return {
                "t_stat": float("inf") if mean_diff > 0 else float("-inf"),
                "df": df,
                "p_value": 0.0,
                "t_crit": t_crit,
                "mean_diff": mean_diff,
                "sd_diff": 0.0,
                "se_diff": 0.0,
                "sum_diff": sum(diffs),
                "sum_dev_sq": 0.0,
                "alpha": alpha,
                "is_identical": False,
                "is_significant": True,
                "table_rows": table_rows,
                "conclusion": "Дисперсія різниць дорівнює 0 при ненульовій різниці. Різниця константна і статистично значуща."
            }

    sd_diff = math.sqrt(sum_sq_diff / (n - 1))
    se_diff = sd_diff / math.sqrt(n)
    t_stat = (mean_diff * math.sqrt(n)) / sd_diff
    p_value = float(2.0 * (1.0 - st.t.cdf(abs(t_stat), df=df)))
    is_significant = p_value < alpha
    
    if is_significant:
        conclusion = (
            f"Нульова гіпотеза H0 відхиляється (|t| = {abs(t_stat):.4f} > t_crit = {t_crit:.4f}, p = {p_value:.4f} < {alpha}). "
            f"Середні значення сукупностей істотно відрізняються."
        )
    else:
        conclusion = (
            f"Немає підстав відхиляти нульову гіпотезу H0 (|t| = {abs(t_stat):.4f} <= t_crit = {t_crit:.4f}, p = {p_value:.4f} >= {alpha}). "
            f"Середні значення сукупностей можна вважати рівними."
        )

    return {
        "t_stat": t_stat,
        "df": df,
        "p_value": p_value,
        "t_crit": t_crit,
        "mean_diff": mean_diff,
        "sd_diff": sd_diff,
        "se_diff": se_diff,
        "sum_diff": sum(diffs),
        "sum_dev_sq": sum_sq_diff,
        "alpha": alpha,
        "is_identical": False,
        "is_significant": is_significant,
        "table_rows": table_rows,
        "conclusion": conclusion
    }

def wilcoxon_signed_rank_test(x: list[float], y: list[float], alpha: float = 0.05) -> dict:
    """
    Власна реалізація критерію знакових рангів Вілкоксона за розділом 3 методичних вказівок.
    Використовує власну функцію compute_ranks.
    """
    if len(x) != len(y):
        raise ValueError("Вибірки повинні мати однаковий розмір.")
    n_total = len(x)
    diffs = [round(a - b, 9) for a, b in zip(x, y)]
    
    # 2. Якщо серед елементів є нульові, їх вилучають. N' - кількість ненульових різниць.
    non_zero = [(d, i) for i, d in enumerate(diffs) if abs(d) > 1e-9]
    n_r = len(non_zero)
    
    if n_r == 0:
        return {
            "n_total": n_total,
            "n_nonzero": 0,
            "w_plus": 0.0,
            "w_minus": 0.0,
            "t_stat": 0.0,
            "t_crit_table": None,
            "u_stat": 0.0,
            "u_crit": float(st.norm.ppf(1 - alpha / 2)),
            "p_value_norm": 1.0,
            "p_value_exact": 1.0,
            "alpha": alpha,
            "is_identical": True,
            "is_significant": False,
            "table_rows": [],
            "table_ranking": [],
            "conclusion": "Вибірки повністю збігаються (всі d_i = 0). H0 не відхиляється."
        }

    # 3. Переходять до абсолютних значень різниць
    abs_diffs = [abs(item[0]) for item in non_zero]
    
    # 4. Присвоюють ранги r_i
    ranks = compute_ranks(abs_diffs)
    
    # 5. Суми рангів позитивних та негативних різниць
    t_plus = 0.0
    t_minus = 0.0
    table_rows = []
    
    for (d, orig_idx), rank in zip(non_zero, ranks):
        r_pos = rank if d > 0 else 0.0
        r_neg = rank if d < 0 else 0.0
        if d > 0:
            t_plus += rank
        else:
            t_minus += rank
        table_rows.append({
            "idx": orig_idx + 1,
            "x": x[orig_idx],
            "y": y[orig_idx],
            "d": d,
            "abs_d": abs(d),
            "rank": rank,
            "r_pos": r_pos,
            "r_neg": r_neg
        })

    # T = min(T+, T-)
    t_stat = min(t_plus, t_minus)
    
    # Таблиця 13 (визначення рангів для |d_i|)
    sorted_pairs = sorted(zip(abs_diffs, ranks), key=lambda p: p[0])
    table_ranking = []
    for rank_idx, (abs_val, r) in enumerate(sorted_pairs, start=1):
        table_ranking.append({
            "order": rank_idx,
            "abs_d": abs_val,
            "rank": r
        })

    # Математичне сподівання та дисперсія за методичкою
    e_t = (n_r * (n_r + 1)) / 4.0
    
    # В'язки для дисперсії: sum A_j(A_j - 1)(A_j + 1)
    tie_counts = {}
    for r in ranks:
        tie_counts[r] = tie_counts.get(r, 0) + 1
    tie_term = sum(aj * (aj - 1) * (aj + 1) for aj in tie_counts.values() if aj > 1)
    
    d_t = (n_r * (n_r + 1) * (2 * n_r + 1)) / 24.0 - (tie_term / 48.0)
    std_t = math.sqrt(max(1e-9, d_t))
    
    # Стандартизована статистика u = (T - E{T}) / sqrt(D{T})
    u_stat = (t_stat - e_t) / std_t
    u_crit = float(st.norm.ppf(1 - alpha / 2))
    p_value_norm = float(2.0 * (1.0 - st.norm.cdf(abs(u_stat))))
    
    # Точне p-значення за scipy для порівняння
    try:
        scipy_w = st.wilcoxon([item[0] for item in non_zero])
        p_value_exact = float(scipy_w.pvalue)
    except Exception:
        p_value_exact = p_value_norm

    # Табличне критичне значення T_крит (для alpha=0.05)
    t_crit_table = WILCOXON_CRITICAL_TABLE_005.get(n_r, None)
    
    # Висновок за методичкою:
    # Для N' <= 15 порівнюють T з T_крит (якщо alpha == 0.05)
    if n_r <= 15 and t_crit_table is not None and abs(alpha - 0.05) < 1e-4:
        if t_stat > t_crit_table:
            is_significant = False
            conclusion = (
                f"Немає підстав відхиляти нульову гіпотезу H0 (T = {t_stat:.1f} > T_крит = {t_crit_table}, N' = {n_r}). "
                f"Систематичного зсуву між розподілами немає, центральні тенденції можна вважати однаковими."
            )
        else:
            is_significant = True
            conclusion = (
                f"Нульова гіпотеза H0 відхиляється (T = {t_stat:.1f} <= T_крит = {t_crit_table}, N' = {n_r}). "
                f"Є систематичний зсув між розподілами, центральні тенденції розподілів різняться."
            )
    else:
        # За нормальним наближенням (u-статистика або p-значення)
        if p_value_norm < alpha or abs(u_stat) > u_crit:
            is_significant = True
            conclusion = (
                f"Нульова гіпотеза H0 відхиляється (|u| = {abs(u_stat):.2f} > u_crit = {u_crit:.2f}, p = {p_value_norm:.4f} < {alpha}). "
                f"Є систематичний зсув між розподілами, центральні тенденції розподілів різняться."
            )
        else:
            is_significant = False
            conclusion = (
                f"Немає підстав відхиляти нульову гіпотезу H0 (|u| = {abs(u_stat):.2f} <= u_crit = {u_crit:.2f}, p = {p_value_norm:.4f} >= {alpha}). "
                f"Систематичного зсуву між розподілами немає, центральні тенденції можна вважати однаковими."
            )

    return {
        "n_total": n_total,
        "n_nonzero": n_r,
        "t_plus": t_plus,
        "t_minus": t_minus,
        "t_stat": t_stat,
        "t_crit_table": t_crit_table,
        "e_t": e_t,
        "d_t": d_t,
        "std_t": std_t,
        "u_stat": u_stat,
        "u_crit": u_crit,
        "p_value_norm": p_value_norm,
        "p_value_exact": p_value_exact,
        "alpha": alpha,
        "is_identical": False,
        "is_significant": is_significant,
        "table_rows": table_rows,
        "table_ranking": table_ranking,
        "conclusion": conclusion
    }

def check_normality(data: list[float], alpha: float = 0.05) -> list[dict]:
    n = len(data)
    results = []
    
    if n >= 3:
        w_stat, p_val = st.shapiro(data)
        is_norm = p_val >= alpha
        results.append({
            "test": "Критерій Шапіро-Вілка",
            "stat": f"W = {w_stat:.4f}",
            "crit": f"p = {p_val:.4g}",
            "conclusion": "Нормальний" if is_norm else "Не є нормальним (відхилення)",
            "is_normal": is_norm
        })
    else:
        results.append({
            "test": "Критерій Шапіро-Вілка",
            "stat": "N/A", "crit": "N/A",
            "conclusion": "Замало даних (n < 3)", "is_normal": False
        })
    
    if n >= 4:
        stats = calculate_base_statistics(data)
        if stats["s_unbias"] > 1e-12:
            se_a2 = math.sqrt((6.0 * n * (n - 1)) / ((n - 2) * (n + 1) * (n + 3)))
            se_e2 = math.sqrt((24.0 * n * (n - 1) ** 2) / ((n - 3) * (n - 2) * (n + 3) * (n + 5)))
            u_a = stats["skewness_a2"] / se_a2
            u_e = stats["kurtosis_e2"] / se_e2
            u_crit = float(st.norm.ppf(1 - alpha / 2))
            
            is_norm_a = abs(u_a) <= u_crit
            is_norm_e = abs(u_e) <= u_crit
            
            results.append({
                "test": "Критерій асиметрії (|u_A| <= u_crit)",
                "stat": f"u_A = {u_a:.4f}",
                "crit": f"u_crit = {u_crit:.4f}",
                "conclusion": "Нормальний" if is_norm_a else "Не є нормальним",
                "is_normal": is_norm_a
            })
            results.append({
                "test": "Критерій ексцесу (|u_E| <= u_crit)",
                "stat": f"u_E = {u_e:.4f}",
                "crit": f"u_crit = {u_crit:.4f}",
                "conclusion": "Нормальний" if is_norm_e else "Не є нормальним",
                "is_normal": is_norm_e
            })
    return results

def analyze_central_tendencies(stats_diff: dict, ci_diff: dict, confidence_level: float = 0.95) -> str:
    mean_ci = ci_diff.get("mean_ci", (0.0, 0.0))
    median_ci = ci_diff.get("median_ci", (0.0, 0.0))
    
    mean_has_zero = (mean_ci[0] <= 0.0 <= mean_ci[1])
    median_has_zero = (median_ci[0] <= 0.0 <= median_ci[1])
    pct = int(round(confidence_level * 100))
    
    lines = [
        f"• Середнє значення різниць: d̄ = {stats_diff['mean']:.4f}, {pct}% ДІ: [{mean_ci[0]:.4f}; {mean_ci[1]:.4f}]",
        f"• Медіана різниць: Me = {stats_diff['median']:.4f}, {pct}% ДІ: [{median_ci[0]:.4f}; {median_ci[1]:.4f}]"
    ]
    
    if mean_has_zero and median_has_zero:
        lines.append(
            "• Висновок: Обидва довірчі інтервали (для середнього та медіани) МІСТЯТЬ нуль (0). "
            "Це є попереднім свідченням близькості центральних тенденцій двох вибірок (відсутність статистично відчутної систематичної переваги жодної з вибірок)."
        )
    elif mean_has_zero or median_has_zero:
        lines.append(
            "• Висновок: Один із довірчих інтервалів містить 0, а інший — ні. "
            "Це вказує на помітну асиметрію різниць та наявність можливих викидів. Остаточне рішення слід приймати за непараметричним критерієм."
        )
    else:
        lines.append(
            "• Висновок: Жоден із довірчих інтервалів НЕ містить нуль (0). "
            "Це є свідченням наявності систематичного зсуву між вибірками."
        )
    return "\n".join(lines)
