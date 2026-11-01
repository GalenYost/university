using System;
using System.Collections.ObjectModel;
using System.Linq;
using Avalonia.Controls;
using Avalonia.Data;
using Avalonia.Interactivity;
using Avalonia.Platform.Storage;
using Microsoft.EntityFrameworkCore;
using lab2.Services;
using lab2.Views.Dialogs;
using NotaryApp;

namespace lab2.Views;

public partial class MainWindow : Window {
    private readonly AppDbContext _db = new();
    private readonly ObservableCollection<Client> _clients = new();
    private readonly ObservableCollection<Service> _services = new();
    private readonly ObservableCollection<Deal> _deals = new();
    private string? _lastQueryTitle;
    private string? _lastQuerySql;
    private QueryResult? _lastQueryResult;

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

    private async void RunQuery_Click(object? sender, RoutedEventArgs e) {
        if (sender is not Button { Tag: string tag } || !int.TryParse(tag, out var idx)) return;
        if (idx < 0 || idx >= ReportQuery.All.Length) return;
        var query = ReportQuery.All[idx];

        string sql;
        if (query.Parameters.Length > 0) {
            var dlg = new QueryParamsDialog(query.Title, query.Description, query.Parameters);
            if (!await dlg.ShowDialog<bool>(this)) return;
            sql = query.GetSql(dlg.Result);
        } else {
            sql = query.Sql;
        }

        QuerySqlBox.Text = sql;
        QueryResultGrid.ItemsSource = null;
        QueryResultGrid.Columns.Clear();

        try {
            var result = QueryExecutor.Execute(_db, sql);

            for (var i = 0; i < result.Headers.Count; i++) {
                QueryResultGrid.Columns.Add(new DataGridTextColumn {
                    Header = result.Headers[i],
                    Binding = new Binding($"Values[{i}]"),
                    Width = new DataGridLength(1, DataGridLengthUnitType.Auto),
                });
            }

            QueryResultGrid.ItemsSource = result.Rows;
            Status.Text = $"{query.Title} — знайдено рядків: {result.Rows.Count}.";
            _lastQueryTitle = query.Title;
            _lastQuerySql = sql;
            _lastQueryResult = result;
            ExportPdfButton.IsEnabled = result.Rows.Count > 0;
        } catch (Exception ex) {
            Status.Text = "Помилка запиту: " + ex.Message;
        }
    }

    private async void ExportPdf_Click(object? sender, RoutedEventArgs e) {
        if (_lastQueryResult == null || string.IsNullOrEmpty(_lastQueryTitle) || string.IsNullOrEmpty(_lastQuerySql)) {
            Status.Text = "Спочатку виконайте запит.";
            return;
        }

        var topLevel = TopLevel.GetTopLevel(this);
        if (topLevel == null) return;

        var file = await topLevel.StorageProvider.SaveFilePickerAsync(new FilePickerSaveOptions {
            Title = "Зберегти звіт у PDF",
            DefaultExtension = "pdf",
            SuggestedFileName = $"report_{DateTime.Now:yyyyMMdd_HHmmss}.pdf",
            FileTypeChoices = new[] {
                new FilePickerFileType("PDF Документ") {
                    Patterns = new[] { "*.pdf" }
                }
            }
        });

        if (file == null) return;

        try {
            var path = file.Path.LocalPath;
            PdfReportService.GenerateReport(path, _lastQueryTitle, _lastQuerySql, _lastQueryResult);
            Status.Text = $"Звіт успішно збережено: {path}";
        } catch (Exception ex) {
            Status.Text = "Помилка формування PDF: " + ex.Message;
        }
    }
}