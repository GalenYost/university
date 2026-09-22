using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using Avalonia.Controls;
using Avalonia.Data;
using Avalonia.Interactivity;
using Microsoft.EntityFrameworkCore;
using Npgsql;
using NotaryApp;

namespace lab1;

public partial class MainWindow : Window {
    private readonly AppDbContext _db = new();
    private readonly ObservableCollection<Client> _clients = new();
    private readonly ObservableCollection<Service> _services = new();
    private readonly ObservableCollection<Deal> _deals = new();

    public MainWindow() {
        InitializeComponent();

        ClientsGrid.ItemsSource = _clients;
        ServicesGrid.ItemsSource = _services;
        DealsGrid.ItemsSource = _deals;

        RefreshAll();
    }

    private void RefreshAll() {
        try {
            _clients.Clear();
            foreach (var item in _db.Clients.OrderBy(c => c.Name)) _clients.Add(item);

            _services.Clear();
            foreach (var item in _db.Services.OrderBy(s => s.Name)) _services.Add(item);

            _deals.Clear();
            foreach (var item in _db.Deals.Include(d => d.Client).Include(d => d.Service).OrderByDescending(d => d.Id)) _deals.Add(item);
        } catch (Exception ex) {
            Status.Text = "Помилка доступу до БД: " + ex.Message;
        }
    }

    private void SaveAndRefresh(string success) {
        try {
            _db.SaveChanges();
            RefreshAll();
            Status.Text = success;
        } catch (Exception ex) {
            Status.Text = "Помилка збереження: " + ex.Message;
        }
    }

    private async void AddClient_Click(object? sender, RoutedEventArgs e) {
        var dlg = new ClientDialog(null);
        if (await dlg.ShowDialog<bool>(this)) {
            _db.Clients.Add(dlg.Result);
            SaveAndRefresh("Клієнта додано.");
        }
    }

    private async void EditClient_Click(object? sender, RoutedEventArgs e) {
        if (ClientsGrid.SelectedItem is not Client client) {
            Status.Text = "Оберіть клієнта в списку.";
            return;
        }

        var dlg = new ClientDialog(client);
        if (await dlg.ShowDialog<bool>(this)) SaveAndRefresh("Дані клієнта оновлено.");
    }

    private async void DeleteClient_Click(object? sender, RoutedEventArgs e) {
        if (ClientsGrid.SelectedItem is not Client client) {
            Status.Text = "Оберіть клієнта в списку.";
            return;
        }

        var confirm = new ConfirmDialog("Видалити клієнта? Пов'язані угоди також буде видалено.");
        if (!await confirm.ShowDialog<bool>(this)) return;

        _db.Clients.Remove(client);
        SaveAndRefresh("Клієнта видалено.");
    }

    private async void AddService_Click(object? sender, RoutedEventArgs e) {
        var dlg = new ServiceDialog(null);
        if (await dlg.ShowDialog<bool>(this)) {
            _db.Services.Add(dlg.Result);
            SaveAndRefresh("Послугу додано.");
        }
    }

    private async void EditService_Click(object? sender, RoutedEventArgs e) {
        if (ServicesGrid.SelectedItem is not Service service) {
            Status.Text = "Оберіть послугу в списку.";
            return;
        }

        var dlg = new ServiceDialog(service);
        if (await dlg.ShowDialog<bool>(this)) SaveAndRefresh("Дані послуги оновлено.");
    }

    private async void DeleteService_Click(object? sender, RoutedEventArgs e) {
        if (ServicesGrid.SelectedItem is not Service service) {
            Status.Text = "Оберіть послугу в списку.";
            return;
        }

        var confirm = new ConfirmDialog("Видалити послугу? Пов'язані угоди також буде видалено.");
        if (!await confirm.ShowDialog<bool>(this)) return;

        _db.Services.Remove(service);
        SaveAndRefresh("Послугу видалено.");
    }

    private async void AddDeal_Click(object? sender, RoutedEventArgs e) {
        if (_clients.Count == 0 || _services.Count == 0) {
            Status.Text = "Спочатку додайте щонайменше одного клієнта й одну послугу.";
            return;
        }

        var dlg = new DealDialog(null, _clients.ToList(), _services.ToList());
        if (await dlg.ShowDialog<bool>(this)) {
            _db.Deals.Add(dlg.Result);
            SaveAndRefresh("Угоду додано.");
        }
    }

    private async void EditDeal_Click(object? sender, RoutedEventArgs e) {
        if (DealsGrid.SelectedItem is not Deal deal) {
            Status.Text = "Оберіть угоду в списку.";
            return;
        }

        var dlg = new DealDialog(deal, _clients.ToList(), _services.ToList());
        if (await dlg.ShowDialog<bool>(this)) SaveAndRefresh("Дані угоди оновлено.");
    }

    private async void DeleteDeal_Click(object? sender, RoutedEventArgs e) {
        if (DealsGrid.SelectedItem is not Deal deal) {
            Status.Text = "Оберіть угоду в списку.";
            return;
        }

        var confirm = new ConfirmDialog("Видалити угоду?");
        if (!await confirm.ShowDialog<bool>(this)) return;

        _db.Deals.Remove(deal);
        SaveAndRefresh("Угоду видалено.");
    }

    private static readonly (string Label, string Sql)[] _queries = {
        ("1. Клієнти по алфавіту", "SELECT name, activity_type, address, phone FROM clients ORDER BY name;"),
        ("2. Угоди понад 10000", "SELECT amount, commission FROM deals WHERE amount > 10000.00 ORDER BY amount DESC;"),
        ("3. Угоди: клієнт і послуга",
         "SELECT d.id, c.name AS client_name, s.name AS service_name, d.amount, d.commission " +
         "FROM deals d JOIN clients c ON c.id = d.client_id " +
         "JOIN services s ON s.id = d.service_id ORDER BY d.id;"),
        ("4. Дохід по клієнту",
         "SELECT c.name, COUNT(d.id) AS deal_count, SUM(d.commission) AS total_commission " +
         "FROM clients c LEFT JOIN deals d ON d.client_id = c.id " +
         "GROUP BY c.id, c.name HAVING SUM(d.commission) IS NOT NULL ORDER BY total_commission DESC;"),
        ("5. Клієнти «ТОВ»", "SELECT id, name, phone FROM clients WHERE name LIKE '%ТОВ%';"),
    };

    private sealed class QueryResultRow {
        public object?[] Values { get; }
        public QueryResultRow(object?[] values) => Values = values;
    }

    private void RunQuery_Click(object? sender, RoutedEventArgs e) {
        if (sender is not Button { Tag: string tag } || !int.TryParse(tag, out var idx) || idx < 0 || idx >= _queries.Length) return;
        var (label, sql) = _queries[idx];

        QueryResultGrid.ItemsSource = null;
        QueryResultGrid.Columns.Clear();

        try {
            var conn = (NpgsqlConnection)_db.Database.GetDbConnection();
            conn.Open();
            try {
                using var cmd = new NpgsqlCommand(sql, conn);
                using var reader = cmd.ExecuteReader();

                var headers = new List<string>();
                for (var i = 0; i < reader.FieldCount; i++) headers.Add(reader.GetName(i));

                foreach (var h in headers) {
                    QueryResultGrid.Columns.Add(new DataGridTextColumn {
                        Header = h,
                        Binding = new Binding($"Values[{QueryResultGrid.Columns.Count}]"),
                        Width = new DataGridLength(1, DataGridLengthUnitType.Auto),
                    });
                }

                var rows = new List<QueryResultRow>();
                while (reader.Read()) {
                    var vals = new object?[reader.FieldCount];
                    for (var i = 0; i < vals.Length; i++) vals[i] = reader.IsDBNull(i) ? null : reader.GetValue(i);
                    rows.Add(new QueryResultRow(vals));
                }

                QueryResultGrid.ItemsSource = rows;
                Status.Text = label + $" — знайдено рядків: {rows.Count}.";
            } finally {
                conn.Close();
            }
        } catch (Exception ex) {
            Status.Text = "Помилка запиту: " + ex.Message;
        }
    }
}