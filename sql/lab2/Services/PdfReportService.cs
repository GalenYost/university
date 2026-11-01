using System;
using QuestPDF.Fluent;
using QuestPDF.Helpers;
using QuestPDF.Infrastructure;

namespace lab2.Services;

public static class PdfReportService {
    static PdfReportService() {
        QuestPDF.Settings.License = LicenseType.Community;
        QuestPDF.Settings.UseSystemFonts = true;
        QuestPDF.Settings.ThrowOnMissingFontFamilies = false;
    }

    public static void GenerateReport(string filePath, string title, string sql, QueryResult result) {
        Document.Create(container => {
            container.Page(page => {
                page.Size(PageSizes.A4.Landscape());
                page.Margin(20);
                page.DefaultTextStyle(x => x.FontSize(9).FontFamily("DejaVu Sans", "Liberation Sans", "Arial"));

                page.Header().Column(col => {
                    col.Item().Text("Нотаріальна контора — Звіт за результатами запиту")
                        .FontSize(15).Bold().FontColor(Colors.Blue.Darken2);
                    col.Item().Text(title).FontSize(12).SemiBold();
                    col.Item().Text($"Дата формування: {DateTime.Now:dd.MM.yyyy HH:mm:ss}").FontSize(8).FontColor(Colors.Grey.Medium);
                    col.Item().PaddingTop(4).Text($"SQL-запит:\n{sql}").FontSize(7).FontFamily("Courier New", "monospace").FontColor(Colors.Grey.Darken2);
                    col.Item().PaddingVertical(6).LineHorizontal(1).LineColor(Colors.Grey.Lighten2);
                });

                page.Content().Table(table => {
                    table.ColumnsDefinition(columns => {
                        for (int i = 0; i < result.Headers.Count; i++) {
                            columns.RelativeColumn();
                        }
                    });

                    table.Header(header => {
                        for (int i = 0; i < result.Headers.Count; i++) {
                            header.Cell().Background(Colors.Grey.Lighten3)
                                .BorderBottom(1).BorderColor(Colors.Grey.Medium)
                                .Padding(4)
                                .Text(result.Headers[i])
                                .Bold().FontSize(8);
                        }
                    });

                    for (int r = 0; r < result.Rows.Count; r++) {
                        var row = result.Rows[r];
                        var bg = r % 2 == 1 ? Colors.Grey.Lighten4 : Colors.White;
                        for (int c = 0; c < row.Values.Length; c++) {
                            var val = row.Values[c]?.ToString() ?? "NULL";
                            table.Cell().Background(bg)
                                .BorderBottom(0.5f).BorderColor(Colors.Grey.Lighten2)
                                .Padding(4)
                                .Text(val).FontSize(8);
                        }
                    }
                });

                page.Footer().Row(row => {
                    row.RelativeItem().Text($"Всього знайдено записів: {result.Rows.Count}").FontSize(8);
                    row.RelativeItem().AlignRight().Text(x => {
                        x.Span("Сторінка ");
                        x.CurrentPageNumber();
                        x.Span(" з ");
                        x.TotalPages();
                    });
                });
            });
        }).GeneratePdf(filePath);
    }
}
