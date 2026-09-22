using Avalonia.Controls;
using Avalonia.Interactivity;
using NotaryApp;

namespace lab1;

public partial class ClientDialog : Window {
    public Client Result { get; }

    public ClientDialog(Client? client) {
        InitializeComponent();
        Result = client ?? new Client();

        NameBox.Text = Result.Name;
        ActivityBox.Text = Result.ActivityType;
        AddressBox.Text = Result.Address;
        PhoneBox.Text = Result.Phone;
    }

    private void Ok_Click(object? sender, RoutedEventArgs e) {
        if (string.IsNullOrWhiteSpace(NameBox.Text)) return;

        Result.Name = NameBox.Text.Trim();
        Result.ActivityType = ActivityBox.Text?.Trim() ?? string.Empty;
        Result.Address = AddressBox.Text?.Trim() ?? string.Empty;
        Result.Phone = PhoneBox.Text?.Trim() ?? string.Empty;
        Close(true);
    }

    private void Cancel_Click(object? sender, RoutedEventArgs e) => Close(false);
}