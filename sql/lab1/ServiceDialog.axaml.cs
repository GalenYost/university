using Avalonia.Controls;
using Avalonia.Interactivity;
using NotaryApp;

namespace lab1;

public partial class ServiceDialog : Window {
    public Service Result { get; }

    public ServiceDialog(Service? service) {
        InitializeComponent();
        Result = service ?? new Service();

        NameBox.Text = Result.Name;
        DescriptionBox.Text = Result.Description;
    }

    private void Ok_Click(object? sender, RoutedEventArgs e) {
        if (string.IsNullOrWhiteSpace(NameBox.Text)) return;

        Result.Name = NameBox.Text.Trim();
        Result.Description = DescriptionBox.Text?.Trim() ?? string.Empty;
        Close(true);
    }

    private void Cancel_Click(object? sender, RoutedEventArgs e) => Close(false);
}