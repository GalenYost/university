using System;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Threading.Tasks;

namespace lab2_SocketApp;

public class SocketService {
    private TcpListener? _listener;
    private TcpClient? _client;
    private NetworkStream? _stream;

    public async Task StartServerAsync(int port, Action<string> onMessageReceived) {
        _listener = new TcpListener(IPAddress.Any, port);
        _listener.Start();

        _client = await _listener.AcceptTcpClientAsync();
        _stream = _client.GetStream();

        _ = Task.Run(() => ListenForMessagesAsync(onMessageReceived));
    }

    public async Task ConnectAsync(string ip, int port, Action<string> onMessageReceived) {
        _client = new TcpClient();
        await _client.ConnectAsync(ip, port);
        _stream = _client.GetStream();

        _ = Task.Run(() => ListenForMessagesAsync(onMessageReceived));
    }

    public async Task SendMessageAsync(string message) {
        if (_stream == null) return;

        byte[] buffer = Encoding.UTF8.GetBytes(message);
        await _stream.WriteAsync(buffer, 0, buffer.Length);
    }

    private async Task ListenForMessagesAsync(Action<string> onMessageReceived) {
        byte[] buffer = new byte[1024];
        while (_stream != null) {
            try {
                int bytesRead = await _stream.ReadAsync(buffer, 0, buffer.Length);
                if (bytesRead == 0) break;

                string message = Encoding.UTF8.GetString(buffer, 0, bytesRead);
                onMessageReceived(message);
            }
            catch {
                break;
            }
        }
    }

    public void Stop() {
        _stream?.Close();
        _client?.Close();
        _listener?.Stop();
    }
}
