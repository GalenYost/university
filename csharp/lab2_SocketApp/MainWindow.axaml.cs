using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Threading;
using Avalonia.Input;
using System;

namespace lab2_SocketApp;

public partial class MainWindow : Window {
    private readonly SocketService _socketService = new();
    private double _currentFontSize = 14.0;

    public MainWindow() {
        InitializeComponent();
    }

    private void OnWindowKeyDown(object? sender, KeyEventArgs e) {
        bool isCtrl = e.KeyModifiers.HasFlag(KeyModifiers.Control);

        if (isCtrl) {
            if (e.Key == Key.OemPlus || e.Key == Key.Add) {
                ZoomIn();
                e.Handled = true;
            } else if (e.Key == Key.OemMinus || e.Key == Key.Subtract) {
                ZoomOut();
                e.Handled = true;
            } else if (e.Key == Key.D0 || e.Key == Key.NumPad0) {
                ResetZoom();
                e.Handled = true;
            }
        }
    }

    private void ZoomIn() {
        if (_currentFontSize < 32.0) {
            _currentFontSize += 2.0;
            this.FontSize = _currentFontSize;
        }
    }

    private void ZoomOut() {
        if (_currentFontSize > 8.0) {
            _currentFontSize -= 2.0;
            this.FontSize = _currentFontSize;
        }
    }

    private void ResetZoom() {
        _currentFontSize = 14.0;
        this.FontSize = _currentFontSize;
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
