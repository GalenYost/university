using Microsoft.EntityFrameworkCore;
using System;
using System.Collections.Generic;

namespace NotaryApp;

public class Client {
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string ActivityType { get; set; } = string.Empty;
    public string Address { get; set; } = string.Empty;
    public string Phone { get; set; } = string.Empty;
    public List<Deal> Deals { get; set; } = new();
}

public class Service {
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string Description { get; set; } = string.Empty;
    public List<Deal> Deals { get; set; } = new();
}

public class Deal {
    public Guid Id { get; set; }
    public Guid ClientId { get; set; }
    public Client Client { get; set; } = null!;
    public Guid ServiceId { get; set; }
    public Service Service { get; set; } = null!;
    public decimal Amount { get; set; }
    public decimal Commission { get; set; }
    public string Description { get; set; } = string.Empty;
}

public class AppDbContext : DbContext {
    public DbSet<Client> Clients { get; set; }
    public DbSet<Service> Services { get; set; }
    public DbSet<Deal> Deals { get; set; }

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder) {
        var password = Environment.GetEnvironmentVariable("POSTGRES_PASSWORD") ?? "твої_пароль";
        optionsBuilder
            .UseNpgsql($"Host=localhost;Database=db_lab1;Username=postgres;Password={password}")
            .UseSnakeCaseNamingConvention();
    }
}
