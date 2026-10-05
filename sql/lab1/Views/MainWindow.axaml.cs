using System;
using System.Collections.ObjectModel;
using System.Linq;
using Avalonia.Controls;
using Avalonia.Data;
using Avalonia.Interactivity;
using Microsoft.EntityFrameworkCore;
using lab1.Services;
using lab1.Views.Dialogs;
using NotaryApp;

namespace lab1.Views;

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

    private void RunQuery_Click(object? sender, RoutedEventArgs e) {
        if (sender is not Button { Tag: string tag } || !int.TryParse(tag, out var idx)) return;
        if (idx < 0 || idx >= ReportQuery.All.Length) return;
        var query = ReportQuery.All[idx];

        QuerySqlBox.Text = query.Sql;
        QueryResultGrid.ItemsSource = null;
        QueryResultGrid.Columns.Clear();

        try {
            var result = QueryExecutor.Execute(_db, query.Sql);

            for (var i = 0; i < result.Headers.Count; i++) {
                QueryResultGrid.Columns.Add(new DataGridTextColumn {
                    Header = result.Headers[i],
                    Binding = new Binding($"Values[{i}]"),
                    Width = new DataGridLength(1, DataGridLengthUnitType.Auto),
                });
            }

            QueryResultGrid.ItemsSource = result.Rows;
            Status.Text = $"{query.Title} — знайдено рядків: {result.Rows.Count}.";
        } catch (Exception ex) {
            Status.Text = "Помилка запиту: " + ex.Message;
        }
    }
}