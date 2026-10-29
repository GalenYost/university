import math
import random
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
    if not data: return []
    sorted_data = sorted(data)
    total = len(sorted_data)
    series = []
    current_val = sorted_data[0]
    count = 0
    for val in sorted_data:
        if abs(val - current_val) < 1e-9:
            count += 1
        else:
            series.append({"value": current_val, "count": count, "relative_freq": count / total})
            current_val = val
            count = 1
    series.append({"value": current_val, "count": count, "relative_freq": count / total})
    return series

def calculate_sturges_bins(n: int) -> int:
    if n <= 0: return 1
    return max(1, int(round(1 + math.log2(n))))

def build_histogram_data(data: list[float], num_bins: int) -> list[dict]:
    if not data or num_bins <= 0: return []
    min_val, max_val = min(data), max(data)
    if abs(max_val - min_val) < 1e-9:
        return [{"start": min_val - 0.5, "end": max_val + 0.5, "count": len(data), "relative_freq": 1.0}]
    step = (max_val - min_val) / num_bins
    total = len(data)
    bins = [{"start": min_val + i * step, "end": max_val + 1e-9 if i == num_bins - 1 else min_val + (i + 1) * step, "count": 0, "relative_freq": 0.0} for i in range(num_bins)]
    for val in data:
        idx = min(int((val - min_val) / step), num_bins - 1)
        bins[idx]["count"] += 1
    for b in bins:
        b["relative_freq"] = b["count"] / total
    return bins

def calculate_base_statistics(data: list[float]) -> dict:
    n = len(data)
    if n < 4: raise ValueError("calculate_base_statistics requires at least 4 elements.")
    mean = sum(data) / n
    sorted_data = sorted(data)
    median = sorted_data[n // 2] if n % 2 != 0 else (sorted_data[n // 2 - 1] + sorted_data[n // 2]) / 2.0
    sum_sq = sum((x - mean) ** 2 for x in data)
    sum_cube = sum((x - mean) ** 3 for x in data)
    sum_quad = sum((x - mean) ** 4 for x in data)
    if sum_sq == 0:
        return {"mean": mean, "median": median, "s_unbias": 0.0, "skewness_a2": 0.0, "kurtosis_e2": 0.0, "n": n}
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
    if n < 4: return {}
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
    if n < 4: return {}
    means, medians, s_unbiases, skewnesses, kurtosises = [], [], [], [], []
    for _ in range(iterations):
        sample = random.choices(data, k=n)
        mean = sum(sample) / n
        means.append(mean)
        sorted_sample = sorted(sample)
        medians.append(sorted_sample[n // 2] if n % 2 != 0 else (sorted_sample[n // 2 - 1] + sorted_sample[n // 2]) / 2.0)
        sum_sq = sum((x - mean) ** 2 for x in sample)
        sum_cube = sum((x - mean) ** 3 for x in sample)
        sum_quad = sum((x - mean) ** 4 for x in sample)
        if sum_sq == 0:
            s_unbiases.append(0.0); skewnesses.append(0.0); kurtosises.append(0.0)
        else:
            s_unbias = math.sqrt(sum_sq / (n - 1))
            s_bias = math.sqrt(sum_sq / n)
            s_unbiases.append(s_unbias)
            a1 = sum_cube / (n * (s_bias ** 3))
            skewnesses.append((math.sqrt(n * (n - 1)) / (n - 2)) * a1)
            e1 = sum_quad / (n * (s_bias ** 4)) - 3
            kurtosises.append(((n ** 2 - 1) / ((n - 2) * (n - 3))) * (e1 + 6 / (n + 1)))
    means.sort(); medians.sort(); s_unbiases.sort(); skewnesses.sort(); kurtosises.sort()
    alpha = 1.0 - confidence_level
    lower_idx, upper_idx = int(iterations * (alpha / 2)), int(iterations * (1 - alpha / 2)) - 1
    return {
        "mean_boot_ci": (means[lower_idx], means[upper_idx]), "median_boot_ci": (medians[lower_idx], medians[upper_idx]),
        "s_unbias_boot_ci": (s_unbiases[lower_idx], s_unbiases[upper_idx]),
        "skewness_boot_ci": (skewnesses[lower_idx], skewnesses[upper_idx]),
        "kurtosis_boot_ci": (kurtosises[lower_idx], kurtosises[upper_idx])
    }
