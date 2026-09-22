using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using Avalonia.Controls;
using Avalonia.Interactivity;
using NotaryApp;

namespace lab1;

public partial class DealDialog : Window {
    private readonly Deal? _deal;

    public Deal Result { get; private set; } = null!;

    public DealDialog(Deal? deal, List<Client> clients, List<Service> services) {
        InitializeComponent();
        _deal = deal;

        ClientBox.ItemsSource = clients;
        ServiceBox.ItemsSource = services;

        if (deal is not null) {
            ClientBox.SelectedItem = clients.FirstOrDefault(c => c.Id == deal.ClientId);
            ServiceBox.SelectedItem = services.FirstOrDefault(s => s.Id == deal.ServiceId);
            AmountBox.Text = deal.Amount.ToString(CultureInfo.InvariantCulture);
            CommissionBox.Text = deal.Commission.ToString(CultureInfo.InvariantCulture);
            DescriptionBox.Text = deal.Description;
        } else {
            ClientBox.SelectedIndex = 0;
            ServiceBox.SelectedIndex = 0;
        }
    }

    private void Ok_Click(object? sender, RoutedEventArgs e) {
        if (ClientBox.SelectedItem is not Client client) return;
        if (ServiceBox.SelectedItem is not Service service) return;
        if (!TryParseMoney(AmountBox.Text, out var amount)) return;
        if (!TryParseMoney(CommissionBox.Text, out var commission)) return;

        if (_deal is null) {
            Result = new Deal {
                ClientId = client.Id,
                ServiceId = service.Id,
                Amount = amount,
                Commission = commission,
                Description = DescriptionBox.Text?.Trim() ?? string.Empty,
            };
        } else {
            _deal.ClientId = client.Id;
            _deal.ServiceId = service.Id;
            _deal.Amount = amount;
            _deal.Commission = commission;
            _deal.Description = DescriptionBox.Text?.Trim() ?? string.Empty;
            Result = _deal;
        }

        Close(true);
    }

    private void Cancel_Click(object? sender, RoutedEventArgs e) => Close(false);

    private static bool TryParseMoney(string? text, out decimal value) {
        var normalized = (text ?? string.Empty).Trim().Replace(',', '.');
        return decimal.TryParse(normalized, NumberStyles.Number, CultureInfo.InvariantCulture, out value);
    }
}