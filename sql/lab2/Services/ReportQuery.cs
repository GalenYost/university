using System.Collections.Generic;
using System.Globalization;
using System.Linq;

namespace lab2.Services;

public record QueryParameter(
    string Name,
    string DisplayName,
    string DefaultValue,
    bool IsNumeric = false,
    string Description = ""
);

public record Definition {
    public string Title { get; init; }
    public string Category { get; init; }
    public string Description { get; init; }
    public string RawSql { get; init; }
    public QueryParameter[] Parameters { get; init; }

    public Definition(string title, string category, string description, string rawSql, QueryParameter[]? parameters = null) {
        Title = title;
        Category = category;
        Description = description;
        RawSql = rawSql;
        Parameters = parameters ?? [];
    }

    public string Sql => GetSql(null);

    public string GetSql(IReadOnlyDictionary<string, string>? customValues = null) {
        if (Parameters.Length == 0) return RawSql;

        var sql = RawSql;
        foreach (var p in Parameters) {
            var val = customValues != null && customValues.TryGetValue(p.Name, out var v) && !string.IsNullOrWhiteSpace(v)
                ? v
                : p.DefaultValue;

            string formatted;
            if (p.IsNumeric) {
                if (decimal.TryParse(val, NumberStyles.Any, CultureInfo.InvariantCulture, out var d))
                    formatted = d.ToString(CultureInfo.InvariantCulture);
                else
                    formatted = p.DefaultValue;
            } else {
                formatted = "'" + val.Replace("'", "''") + "'";
            }

            sql = sql.Replace(p.Name, formatted);
        }
        return sql;
    }
}

public static class ReportQuery {
    public static readonly Definition[] GroupingAndAggregation = {
        new(
            "1. Сума та сер. комісія по клієнтах і послугах",
            "Агрегація та групування",
            "Загальна сума угод, середня комісія та кількість угод за кожним клієнтом та послугою зі зв'язуванням 3-х таблиць",
            "SELECT c.name AS client_name, s.name AS service_name, COUNT(d.id) AS deals_count, SUM(d.amount) AS total_amount, ROUND(AVG(d.commission), 2) AS avg_commission " +
            "FROM deals d " +
            "JOIN clients c ON d.client_id = c.id " +
            "JOIN services s ON d.service_id = s.id " +
            "GROUP BY c.name, s.name " +
            "ORDER BY total_amount DESC;"
        ),
        new(
            "2. Послуги з обсягом угод > 20 000 грн",
            "Агрегація та групування",
            "Послуги з сумарним обсягом угод понад 20 000 грн та підрахунком унікальних клієнтів",
            "SELECT s.name AS service_name, COUNT(DISTINCT c.id) AS unique_clients, COUNT(d.id) AS total_deals, SUM(d.amount) AS total_amount, SUM(d.commission) AS total_commission " +
            "FROM services s " +
            "JOIN deals d ON s.id = d.service_id " +
            "JOIN clients c ON c.id = d.client_id " +
            "GROUP BY s.name " +
            "HAVING SUM(d.amount) > 20000.00 " +
            "ORDER BY total_amount DESC;"
        ),
        new(
            "3. Статистика угод за видами діяльності",
            "Агрегація та групування",
            "Аналіз угод за видами діяльності клієнтів: мінімальна, максимальна, середня сума угоди та дохід контори",
            "SELECT c.activity_type, COUNT(d.id) AS deals_count, MIN(d.amount) AS min_deal_amount, MAX(d.amount) AS max_deal_amount, ROUND(AVG(d.amount), 2) AS avg_deal_amount, SUM(d.commission) AS total_commission " +
            "FROM clients c " +
            "JOIN deals d ON c.id = d.client_id " +
            "JOIN services s ON d.service_id = s.id " +
            "WHERE c.activity_type <> '' " +
            "GROUP BY c.activity_type " +
            "ORDER BY total_commission DESC;"
        ),
        new(
            "4. Послуги та клієнти з сер. комісією >= 1000 грн",
            "Агрегація та групування",
            "Послуги та клієнти з високою середньою комісією (не менше 1000 грн) та максимальною комісією",
            "SELECT s.name AS service_name, c.name AS client_name, COUNT(d.id) AS deals_count, ROUND(AVG(d.commission), 2) AS avg_commission, MAX(d.commission) AS max_commission " +
            "FROM services s " +
            "JOIN deals d ON s.id = d.service_id " +
            "JOIN clients c ON c.id = d.client_id " +
            "GROUP BY s.name, c.name " +
            "HAVING AVG(d.commission) >= 1000.00 " +
            "ORDER BY avg_commission DESC;"
        ),
        new(
            "5. Клієнти з сер. сумою угоди > 50 000 грн",
            "Агрегація та групування",
            "Клієнти з середньою сумою угоди понад 50 000 грн та переліком замовлених послуг",
            "SELECT c.name AS client_name, c.phone, COUNT(d.id) AS deals_count, COUNT(DISTINCT s.id) AS distinct_services_count, ROUND(AVG(d.amount), 2) AS avg_deal_amount, SUM(d.commission) AS total_commission " +
            "FROM clients c " +
            "JOIN deals d ON c.id = d.client_id " +
            "JOIN services s ON s.id = d.service_id " +
            "GROUP BY c.name, c.phone " +
            "HAVING AVG(d.amount) > 50000.00 " +
            "ORDER BY avg_deal_amount DESC;"
        ),
        new(
            "6. Клієнти та послуги з понад 1 угодою",
            "Агрегація та групування",
            "Послуги та клієнти, що здійснили більше ніж 1 угоду, із загальною статистикою сум та комісійних",
            "SELECT s.name AS service_name, c.name AS client_name, COUNT(d.id) AS deals_count, SUM(d.amount) AS total_amount, SUM(d.commission) AS total_commission " +
            "FROM deals d " +
            "JOIN clients c ON d.client_id = c.id " +
            "JOIN services s ON d.service_id = s.id " +
            "GROUP BY s.name, c.name " +
            "HAVING COUNT(d.id) > 1 " +
            "ORDER BY total_commission DESC;"
        ),
        new(
            "7. Рентабельність послуг (% комісійних)",
            "Агрегація та групування",
            "Відсоток комісійних від загального обороту угод за послугами",
            "SELECT s.name AS service_name, COUNT(d.id) AS deals_count, SUM(d.amount) AS total_volume, SUM(d.commission) AS total_commission, ROUND((SUM(d.commission) * 100.0 / NULLIF(SUM(d.amount), 0)), 2) AS commission_percent " +
            "FROM services s " +
            "JOIN deals d ON s.id = d.service_id " +
            "JOIN clients c ON c.id = d.client_id " +
            "GROUP BY s.name " +
            "HAVING SUM(d.amount) > 0 " +
            "ORDER BY commission_percent DESC;"
        )
    };

    public static readonly Definition[] NestedAndDynamic = {
        new(
            "8. [IN] Клієнти з угодами від мін. суми",
            "Вкладені та параметризовані",
            "Клієнти, які уклали угоди на суму не менше заданої, з деталізацією угод та послуг",
            "SELECT c.name AS client_name, c.phone, s.name AS service_name, d.amount, d.commission, d.description " +
            "FROM deals d " +
            "JOIN clients c ON d.client_id = c.id " +
            "JOIN services s ON d.service_id = s.id " +
            "WHERE c.id IN ( " +
            "    SELECT d2.client_id FROM deals d2 WHERE d2.amount >= @min_amount " +
            ") " +
            "ORDER BY d.amount DESC;",
            [new QueryParameter("@min_amount", "Мін. сума угоди", "50000", true, "Мінімальна сума угоди в грн")]
        ),
        new(
            "9. [Корельований] Угоди вище середньої по послузі",
            "Вкладені та параметризовані",
            "Угоди обраної послуги, сума яких вища за середню суму угод саме цієї послуги",
            "SELECT c.name AS client_name, s.name AS service_name, d.amount, d.commission, d.description " +
            "FROM deals d " +
            "JOIN clients c ON d.client_id = c.id " +
            "JOIN services s ON d.service_id = s.id " +
            "WHERE s.name ILIKE @service_name " +
            "  AND d.amount > ( " +
            "      SELECT AVG(d2.amount) FROM deals d2 WHERE d2.service_id = d.service_id " +
            "  ) " +
            "ORDER BY d.amount DESC;",
            [new QueryParameter("@service_name", "Фільтр послуги", "%", false, "Назва послуги або шаблон (%)")]
        ),
        new(
            "10. [EXISTS] Угоди клієнтів за видом діяльності",
            "Вкладені та параметризовані",
            "Послуги та угоди тих клієнтів, які мають вказаний вид діяльності (підзапит EXISTS)",
            "SELECT c.name AS client_name, c.activity_type, s.name AS service_name, d.amount, d.commission " +
            "FROM deals d " +
            "JOIN services s ON d.service_id = s.id " +
            "JOIN clients c ON d.client_id = c.id " +
            "WHERE EXISTS ( " +
            "    SELECT 1 FROM clients c2 WHERE c2.id = d.client_id AND c2.activity_type ILIKE @activity_type " +
            ") " +
            "ORDER BY d.amount DESC;",
            [new QueryParameter("@activity_type", "Вид діяльності", "%IT%", false, "Вид діяльності клієнта")]
        ),
        new(
            "11. [BETWEEN] Фільтр угод за діапазоном сум",
            "Вкладені та параметризовані",
            "Динамічний фільтр угод за діапазоном сум з повною інформацією про клієнта та послугу",
            "SELECT d.id AS deal_id, c.name AS client_name, c.phone, s.name AS service_name, d.amount, d.commission, d.description " +
            "FROM deals d " +
            "JOIN clients c ON d.client_id = c.id " +
            "JOIN services s ON d.service_id = s.id " +
            "WHERE d.amount BETWEEN @min_amount AND @max_amount " +
            "ORDER BY d.amount DESC;",
            [
                new QueryParameter("@min_amount", "Від суми", "10000", true, "Нижня межа суми угоди"),
                new QueryParameter("@max_amount", "До суми", "200000", true, "Верхня межа суми угоди")
            ]
        ),
        new(
            "12. [Агрегатний підзапит] Топ-комісійні у системі",
            "Вкладені та параметризовані",
            "Угоди, де комісійні не менше заданих та складають не менше 50% від максимальної комісії в системі",
            "SELECT c.name AS client_name, s.name AS service_name, d.amount, d.commission, d.description " +
            "FROM deals d " +
            "JOIN clients c ON d.client_id = c.id " +
            "JOIN services s ON d.service_id = s.id " +
            "WHERE d.commission >= @min_commission " +
            "  AND d.commission >= (SELECT MAX(d2.commission) * 0.5 FROM deals d2) " +
            "ORDER BY d.commission DESC;",
            [new QueryParameter("@min_commission", "Мін. комісія", "1000", true, "Мінімальна сума комісійних")]
        ),
        new(
            "13. [Пошук] Контекстний пошук у 3-х таблицях",
            "Вкладені та параметризовані",
            "Динамічний контекстний пошук за підрядком у назвах клієнтів, послуг або описі угоди",
            "SELECT c.name AS client_name, c.activity_type, s.name AS service_name, d.amount, d.commission, d.description " +
            "FROM deals d " +
            "JOIN clients c ON d.client_id = c.id " +
            "JOIN services s ON d.service_id = s.id " +
            "WHERE c.name ILIKE @search_term " +
            "   OR s.name ILIKE @search_term " +
            "   OR d.description ILIKE @search_term " +
            "ORDER BY d.amount DESC;",
            [new QueryParameter("@search_term", "Пошуковий запит", "%тест%", false, "Підрядок для пошуку")]
        ),
        new(
            "14. [NOT IN] Клієнти без великих угод",
            "Вкладені та параметризовані",
            "Клієнти, які замовляли вказану послугу, але ще жодного разу не укладали угод на суму більше порогу",
            "SELECT DISTINCT c.name AS client_name, c.phone, c.activity_type, s.name AS service_name, d.amount " +
            "FROM clients c " +
            "JOIN deals d ON c.id = d.client_id " +
            "JOIN services s ON s.id = d.service_id " +
            "WHERE s.name ILIKE @service_name " +
            "  AND c.id NOT IN ( " +
            "      SELECT d2.client_id FROM deals d2 WHERE d2.amount > @threshold_amount " +
            "  ) " +
            "ORDER BY c.name;",
            [
                new QueryParameter("@service_name", "Фільтр послуги", "%", false, "Назва послуги"),
                new QueryParameter("@threshold_amount", "Поріг суми", "150000", true, "Поріг максимальної суми")
            ]
        )
    };

    public static readonly Definition[] All = GroupingAndAggregation.Concat(NestedAndDynamic).ToArray();
}
