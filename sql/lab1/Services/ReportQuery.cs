namespace lab1.Services;

public static class ReportQuery {
    public record Definition(string Title, string Sql);

    public static readonly Definition[] All = {
        new("1. Клієнти по алфавіту",
            "SELECT id, name, activity_type, address, phone FROM clients ORDER BY name;"),
        new("2. Угоди понад 10000",
            "SELECT id, amount, commission FROM deals WHERE amount > 10000.00 ORDER BY amount DESC;"),
        new("3. Угоди: клієнт і послуга",
            "SELECT d.id, c.name AS client_name, s.name AS service_name, d.amount, d.commission " +
            "FROM deals d JOIN clients c ON c.id = d.client_id " +
            "JOIN services s ON s.id = d.service_id ORDER BY d.id;"),
        new("4. Дохід по клієнту",
            "SELECT c.id, c.name, COUNT(d.id) AS deal_count, SUM(d.commission) AS total_commission " +
            "FROM clients c LEFT JOIN deals d ON d.client_id = c.id " +
            "GROUP BY c.id, c.name HAVING SUM(d.commission) IS NOT NULL ORDER BY total_commission DESC;"),
        new("5. Клієнти «ТОВ»",
            "SELECT id, name, phone FROM clients WHERE name LIKE '%ТОВ%';"),
        new("6. Усі послуги",
            "SELECT id, name, description FROM services ORDER BY name;"),
        new("7. Перші 5 послуг за алфавітом",
            "SELECT id, name FROM services ORDER BY name LIMIT 5;"),
        new("8. Клієнти без вказаної діяльності",
            "SELECT id, name FROM clients WHERE activity_type = '' ORDER BY name;"),
        new("9. Угоди конкретної суми",
            "SELECT id, amount, commission FROM deals WHERE amount = 5000.00;"),
        new("10. Угоди з комісійними понад 500",
            "SELECT id, amount, commission FROM deals WHERE commission > 500.00 ORDER BY commission DESC;"),
        new("11. Найбільша сума угоди",
            "SELECT id, amount, commission FROM deals ORDER BY amount DESC LIMIT 1;"),
        new("12. Угоди з назвою клієнта",
            "SELECT d.id, c.name AS client_name, d.amount " +
            "FROM deals d JOIN clients c ON c.id = d.client_id ORDER BY d.id;"),
    };
}
