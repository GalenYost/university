using System;
using System.Collections.Generic;
using System.Globalization;
using Avalonia.Controls;
using Avalonia.Interactivity;
using lab2.Services;

namespace lab2.Views.Dialogs;

public partial class QueryParamsDialog : Window {
    private readonly IReadOnlyList<QueryParameter> _parameters;
    private readonly Dictionary<string, TextBox> _inputs = new();

    public Dictionary<string, string> Result { get; } = new();

    public QueryParamsDialog() : this("", "", Array.Empty<QueryParameter>()) { }

    public QueryParamsDialog(string title, string description, IReadOnlyList<QueryParameter> parameters) {
        InitializeComponent();
        _parameters = parameters;

        TitleBlock.Text = title;
        DescBlock.Text = description;

        foreach (var p in _parameters) {
            var label = new TextBlock {
                Text = string.IsNullOrWhiteSpace(p.Description) ? p.DisplayName : $"{p.DisplayName} ({p.Description}):"
            };
            var box = new TextBox {
                Text = p.DefaultValue
            };
            _inputs[p.Name] = box;

            ParamsPanel.Children.Add(label);
            ParamsPanel.Children.Add(box);
        }
    }

    private void Ok_Click(object? sender, RoutedEventArgs e) {
        ErrorBlock.IsVisible = false;

        foreach (var p in _parameters) {
            var box = _inputs[p.Name];
            var text = box.Text?.Trim() ?? string.Empty;

            if (p.IsNumeric) {
                if (!decimal.TryParse(text, NumberStyles.Any, CultureInfo.InvariantCulture, out _)) {
                    ErrorBlock.Text = $"Поле '{p.DisplayName}' має бути числом.";
                    ErrorBlock.IsVisible = true;
                    return;
                }
            } else if (string.IsNullOrWhiteSpace(text)) {
                ErrorBlock.Text = $"Поле '{p.DisplayName}' не може бути порожнім.";
                ErrorBlock.IsVisible = true;
                return;
            }

            Result[p.Name] = text;
        }

        Close(true);
    }

    private void Cancel_Click(object? sender, RoutedEventArgs e) => Close(false);
}
