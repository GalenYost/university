using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Threading;
using Avalonia.Input;
using System;

namespace lab2_SocketApp;

public partial class MainWindow : Window {
    private readonly SocketService _socketService = new();

    public MainWindow() {
        InitializeComponent();
    }

    private async void OnStartServerClick(object? sender, RoutedEventArgs e) {
        int port = int.Parse(PortInput.Text ?? "8888");
        StatusText.Text = "Status: Awaiting connection...";
        
        await _socketService.StartServerAsync(port, OnMessageReceived);
        StatusText.Text = "Status: Client connected";
    }

    private async void OnConnectClientClick(object? sender, RoutedEventArgs e) {
        string ip = IpInput.Text ?? "127.0.0.1";
        int port = int.Parse(PortInput.Text ?? "8888");
        StatusText.Text = "Status: Connecting...";

        try {
            await _socketService.ConnectAsync(ip, port, OnMessageReceived);
            StatusText.Text = "Status: Connected.";
        } catch (Exception ex) {
            StatusText.Text = $"Error: {ex.Message}";
        }
    }

    private async void OnSendMessageClick(object? sender, RoutedEventArgs e) {
        if (string.IsNullOrWhiteSpace(MessageInput.Text)) return;

        string message = MessageInput.Text;
        await _socketService.SendMessageAsync(message);
        
        ChatBox.Items.Add($"Me: {message}");
        MessageInput.Clear();
    }

    private void OnMessageInputKeyDown(object? sender, KeyEventArgs e) {
        if (e.Key == Key.Enter) {
            OnSendMessageClick(sender, e);
        }
    }

    private void OnMessageReceived(string message) {
        Dispatcher.UIThread.Post(() => {
            ChatBox.Items.Add($"Client: {message}");
        });
    }

    protected override void OnClosing(WindowClosingEventArgs e) {
        _socketService.Stop();
        base.OnClosing(e);
    }
}
