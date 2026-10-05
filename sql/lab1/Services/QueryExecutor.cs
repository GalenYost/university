using System;
using System.Collections.Generic;
using Microsoft.EntityFrameworkCore;
using Npgsql;
using NotaryApp;

namespace lab1.Services;

public sealed class QueryResultRow {
    public object?[] Values { get; }
    public QueryResultRow(object?[] values) => Values = values;
}

public sealed class QueryResult {
    public List<string> Headers { get; }
    public List<QueryResultRow> Rows { get; }

    public QueryResult(List<string> headers, List<QueryResultRow> rows) {
        Headers = headers;
        Rows = rows;
    }
}

public static class QueryExecutor {
    public static QueryResult Execute(AppDbContext db, string sql) {
        var conn = (NpgsqlConnection)db.Database.GetDbConnection();
        conn.Open();
        try {
            using var cmd = new NpgsqlCommand(sql, conn);
            using var reader = cmd.ExecuteReader();

            var headers = new List<string>();
            for (var i = 0; i < reader.FieldCount; i++) headers.Add(reader.GetName(i));

            var rows = new List<QueryResultRow>();
            while (reader.Read()) {
                var values = new object?[reader.FieldCount];
                for (var i = 0; i < values.Length; i++) values[i] = reader.IsDBNull(i) ? null : reader.GetValue(i);
                rows.Add(new QueryResultRow(values));
            }

            return new QueryResult(headers, rows);
        } finally {
            conn.Close();
        }
    }
}
