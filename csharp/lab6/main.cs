using System.Collections;
using System.ComponentModel;

public enum Genre
{
    Pop, Rock, Electronic, Classical
}

public interface IDateAndCopy
{
    object DeepCopy();
    DateTime Date { get; set; }
}

public class Person : IDateAndCopy
{
    private string name;
    private string surname;
    private DateTime birth;

    public string Name { get => name; set => name = value; }
    public string Surname { get => surname; set => surname = value; }
    public DateTime Birthday { get => birth; set => birth = value; }
    public int BirthYear { get => birth.Year; set => birth = new DateTime(value, birth.Month, birth.Day); }

    public virtual DateTime Date { get => Birthday; set => Birthday = value; }

    public Person(string first_name, string surname, DateTime birth)
    {
        this.Name = first_name;
        this.Surname = surname;
        this.Birthday = birth;
    }

    public Person() : this("Name", "Surname", new DateTime(2007, 6, 7)) { }

    public virtual string ToString()
    {
        return $"{this.Name} {this.Surname}, {this.Birthday}";
    }

    public virtual string ToShortString()
    {
        return $"{this.Name} {this.Surname}";
    }

    public virtual bool Equals(object obj)
    {
        if (ReferenceEquals(this, obj)) return true;
        if (obj is not Person other) return false;
        return Name == other.Name && Surname == other.Surname && Birthday.Equals(other.Birthday);
    }

    public static bool operator ==(Person left, Person right)
    {
        if (ReferenceEquals(left, right)) return true;
        if (left is null || right is null) return false;
        return left.Equals(right);
    }

    public static bool operator !=(Person left, Person right) => !(left == right);
    public override int GetHashCode() => HashCode.Combine(Name, Surname, Birthday);
    public virtual object DeepCopy() => new Person(Name, Surname, Birthday);
}

public class Song : IDateAndCopy
{
    public string Name { get; set; }
    public Genre SongGenre { get; set; }
    public DateTime ReleaseDate { get; set; }

    public DateTime Date { get => ReleaseDate; set => ReleaseDate = value; }

    public Song(string name, Genre genre, DateTime release)
    {
        this.Name = name;
        this.SongGenre = genre;
        this.ReleaseDate = release;
    }

    public Song() : this("Unnamed", Genre.Pop, DateTime.Now) { }

    public override string ToString() => $"{this.Name} ({this.SongGenre}) - {this.ReleaseDate.ToShortDateString()}";

    public override bool Equals(object obj)
    {
        if (ReferenceEquals(this, obj)) return true;
        if (obj is not Song other) return false;
        return Name == other.Name && SongGenre == other.SongGenre && ReleaseDate.Equals(other.ReleaseDate);
    }

    public static bool operator ==(Song left, Song right)
    {
        if (ReferenceEquals(left, right)) return true;
        if (left is null || right is null) return false;
        return left.Equals(right);
    }

    public static bool operator !=(Song left, Song right) => !(left == right);
    public override int GetHashCode() => HashCode.Combine(Name, SongGenre, ReleaseDate);
    public virtual object DeepCopy() => new Song(Name, SongGenre, ReleaseDate);
}

public class MusicianEnumerator : IEnumerator
{
    private readonly List<string> titles;
    private int position = -1;

    public MusicianEnumerator(Musician m)
    {
        titles = m.Songs.Select(s => s.Name).Intersect(m.Concerts.Select(c => c.Title)).ToList();
    }

    public object Current
    {
        get
        {
            if (position < 0 || position >= titles.Count) throw new InvalidOperationException();
            return titles[position];
        }
    }

    public bool MoveNext()
    {
        position++;
        return position < titles.Count;
    }

    public void Reset() { position = -1; }
}

public delegate TKey KeySelector<TKey>(Musician mus);
public delegate void MusiciansChangedHandler<TKey>(object source, MusiciansChangedEventArgs<TKey> args);

public class MusiciansChangedEventArgs<TKey> : EventArgs
{
    public string CollectionName { get; set; }
    public Action ChangeType { get; set; }
    public string PropertyName { get; set; }
    public TKey Key { get; set; }

    public MusiciansChangedEventArgs(string collectionName, Action changeType, string propertyName, TKey key)
    {
        CollectionName = collectionName;
        ChangeType = changeType;
        PropertyName = propertyName;
        Key = key;
    }

    public override string ToString()
    {
        string propPart = string.IsNullOrEmpty(PropertyName) ? "" : $", Property: {PropertyName}";
        return $"Collection: {CollectionName}, Action: {ChangeType}, Key: {Key}{propPart}";
    }
}

public class MusicianCollection<TKey>
{
    public string CollectionName { get; set; }
    private Dictionary<TKey, Musician> musicians;
    private KeySelector<TKey> keySelector;

    public event MusiciansChangedHandler<TKey> MusiciansChanged;

    public MusicianCollection(KeySelector<TKey> keySelector)
    {
        this.keySelector = keySelector ?? throw new ArgumentNullException(nameof(keySelector));
        musicians = new Dictionary<TKey, Musician>();
        CollectionName = "Default";
    }

    public void AddDefaults()
    {
        var m1 = new Musician("John", "Doe", new DateTime(1990, 5, 10), "JD", 5);
        m1.AddSongs(new Song("Dreams", Genre.Rock, new DateTime(2020, 1, 15)));
        m1.AddConcerts(new Concert("Live in London", "London", new DateTime(2021, 7, 20)));

        var m2 = new Musician("Alice", "Moore", new DateTime(1995, 3, 22), "A-M", 4);
        m2.AddSongs(new Song("Lights", Genre.Pop, new DateTime(2022, 5, 1)));
        m2.AddConcerts(new Concert("Summer Fest", "Berlin", new DateTime(2023, 8, 5)));

        AddMusicians(m1, m2);
    }

    public void AddMusicians(params Musician[] newMusicians)
    {
        foreach (var mus in newMusicians)
        {
            var key = keySelector(mus);
            if (!musicians.ContainsKey(key))
            {
                musicians.Add(key, mus);
                MusiciansChanged?.Invoke(this, new MusiciansChangedEventArgs<TKey>(
                    CollectionName, Action.Add, "Add", key));
            }
        }
    }

    public bool Remove(Musician mus)
    {
        var key = keySelector(mus);
        if (musicians.ContainsKey(key))
        {
            musicians.Remove(key);
            MusiciansChanged?.Invoke(this, new MusiciansChangedEventArgs<TKey>(
                CollectionName, Action.Remove, "Remove", key));
            return true;
        }
        return false;
    }

    public override string ToString()
    {
        var result = $"Collection: {CollectionName}\n";
        foreach (var kvp in musicians)
        {
            result += $"Key: {kvp.Key}\n{kvp.Value}\n\n";
        }
        return result;
    }

    public string ToShortString()
    {
        var result = $"Collection: {CollectionName}\n";
        foreach (var kvp in musicians)
        {
            var mus = kvp.Value;
            result += $"Key: {kvp.Key}, {mus.ToShortString()}, " +
                      $"Last Song: {mus.LastSong?.Name ?? "None"}, " +
                      $"Songs: {mus.Songs.Count}, Concerts: {mus.Concerts.Count}\n";
        }
        return result;
    }

    public DateTime LastSongDate
    {
        get
        {
            if (musicians.Count == 0 || !musicians.Values.Any(m => m.Songs.Any()))
                return default;

            return musicians.Values
                .Where(m => m.Songs.Any())
                .Max(m => m.Songs.Max(s => s.ReleaseDate));
        }
    }

    public IEnumerable<KeyValuePair<TKey, Musician>> BirthYear(int year)
    {
        return musicians.Where(kvp => kvp.Value.BirthYear == year);
    }

    public IEnumerable<IGrouping<int, KeyValuePair<TKey, Musician>>> GroupByBirthYear
    {
        get
        {
            return musicians.GroupBy(kvp => kvp.Value.BirthYear);
        }
    }
}

public enum Action
{
    Add,
    Remove,
    Property
}

public class Musician : Person, IDateAndCopy, IEnumerable, INotifyPropertyChanged
{
    private string pseudo;
    private int rating;
    private List<Song> songs;
    private List<Concert> concerts;

    public event PropertyChangedEventHandler PropertyChanged;

    public string Pseudo
    {
        get => pseudo; set
        {
            if (pseudo != value)
            {
                pseudo = value;
                OnPropertyChanged(nameof(Pseudo));
            }
        }
    }
    public int Rating
    {
        get => rating;
        set
        {
            if (value <= 0 || value > 5)
                throw new ArgumentOutOfRangeException(nameof(value), "Rating must be between 1 and 5");

            if (rating != value)
            {
                rating = value;
                OnPropertyChanged(nameof(Rating));
            }
        }
    }
    public List<Song> Songs { get => songs; set => songs = value; }
    public List<Concert> Concerts { get => concerts; set => concerts = value; }

    public Song LastSong => this.Songs?.OrderByDescending(song => song.ReleaseDate).FirstOrDefault();
    public override DateTime Date { get => base.Date; set => base.Date = value; }

    IEnumerator IEnumerable.GetEnumerator() => new MusicianEnumerator(this);

    public Musician(string name, string surname, DateTime birthday, string pseudo, int rating)
       : base(name, surname, birthday)
    {
        this.pseudo = pseudo;
        this.rating = rating;
        songs = new(); concerts = new();
    }

    public Musician() : this("Name", "Surname", new DateTime(2007, 6, 7), "Unknown", 0) { }

    public void AddSongs(params Song[] newSongs) => songs.AddRange(newSongs);
    public void AddConcerts(params Concert[] newConcerts) => concerts.AddRange(newConcerts);

    public IEnumerable<object> GetAllItems()
    {
        foreach (var s in songs) yield return s;
        foreach (var c in concerts) yield return c;
    }

    public IEnumerable<Song> GetSongsFromLastYears(int n)
    {
        foreach (var s in songs) if (s.ReleaseDate >= DateTime.Now.AddYears(-n)) yield return s;
    }

    public IEnumerable<Concert> GetConcertsByCity(string city)
    {
        foreach (var c in concerts) if (string.Equals(c.City, city, StringComparison.OrdinalIgnoreCase)) yield return c;
    }

    public IEnumerable<Song> GetSongsByGenre(Genre genre)
    {
        foreach (var s in songs) if (s.SongGenre == genre) yield return s;
    }

    public override string ToString()
    {
        var songsStr = songs.Count > 0 ? string.Join(", ", songs.Select(s => s.ToString())) : "No songs";
        var concertsStr = concerts.Count > 0 ? string.Join(", ", concerts.Select(c => c.ToString())) : "No concerts";
        return $"{base.ToString()}, Stage Name: {Pseudo}, Rating: {Rating}, Songs: [{songsStr}], Concerts: [{concertsStr}]";
    }

    public override string ToShortString() => $"{base.ToShortString()}, Stage: {Pseudo}, Last Song: {LastSong?.Name}";

    public override bool Equals(object obj)
    {
        if (ReferenceEquals(this, obj)) return true;
        if (obj is not Musician other) return false;

        bool songsEqual = songs.SequenceEqual(other.songs);
        bool concertsEqual = concerts.SequenceEqual(other.concerts);

        return base.Equals(other) && Pseudo == other.Pseudo && Rating == other.Rating && songsEqual && concertsEqual;
    }

    public static bool operator ==(Musician left, Musician right)
    {
        if (ReferenceEquals(left, right)) return true;
        if (left is null || right is null) return false;
        return left.Equals(right);
    }

    public static bool operator !=(Musician left, Musician right) => !(left == right);

    public override int GetHashCode()
    {
        int hash = base.GetHashCode();
        hash = HashCode.Combine(hash, Pseudo, Rating);
        if (Songs != null)
        {
            foreach (var song in Songs)
                hash = HashCode.Combine(hash, song);
        }
        return hash;
    }

    public override object DeepCopy()
    {
        var personCopy = (Person)base.DeepCopy();
        var songsCopy = songs.Select(s => (Song)s.DeepCopy()).ToList();
        var concertsCopy = concerts.Select(c => new Concert(c.Title, c.City, c.Date)).ToList();
        var copy = new Musician(personCopy.Name, personCopy.Surname, personCopy.Birthday, Pseudo, Rating)
        {
            Songs = songsCopy,
            Concerts = concertsCopy
        };
        return copy;
    }

    protected void OnPropertyChanged(string propertyName)
    {
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
    }
}

public class ListEntry
{
    public string CollectionName { get; set; }
    public Action ChangeType { get; set; }
    public string PropertyName { get; set; }
    public string KeyString { get; set; }

    public ListEntry(string collectionName, Action changeType, string propertyName, string keyString)
    {
        CollectionName = collectionName;
        ChangeType = changeType;
        PropertyName = propertyName;
        KeyString = keyString;
    }

    public override string ToString()
    {
        string propPart = string.IsNullOrEmpty(PropertyName) ? "" : $", Property: {PropertyName}";
        return $"Collection: {CollectionName}, Action: {ChangeType}, Key: {KeyString}{propPart}";
    }
}

public class Listener<TKey>
{
    private List<ListEntry> entries = new List<ListEntry>();

    public void MusiciansChanged(object source, MusiciansChangedEventArgs<TKey> args)
    {
        entries.Add(new ListEntry(
            args.CollectionName,
            args.ChangeType,
            args.PropertyName,
            args.Key?.ToString() ?? "null"
        ));
    }

    public override string ToString()
    {
        if (entries.Count == 0)
            return "Events list is empty";

        return string.Join("\n", entries.Select(e => e.ToString()));
    }

    public void Clear() => entries.Clear();
}

public class Concert
{
    public string Title { get; set; }
    public string City { get; set; }
    public DateTime Date { get; set; }

    public Concert(string title, string city, DateTime date)
    {
        this.Title = title;
        this.City = city;
        this.Date = date;
    }

    public Concert() : this("Untitled Concert", "Unknown City", DateTime.Now) { }

    public override string ToString()
    {
        return $"Concert: {Title}, City: {City}, Date: {Date.ToShortDateString()}";
    }
}

public class Program
{
    public static void Main()
    {
        var col1 = new MusicianCollection<string>(mus => mus.Pseudo)
        {
            CollectionName = "Rock Legends"
        };
        var col2 = new MusicianCollection<string>(mus => mus.Pseudo)
        {
            CollectionName = "Pop Stars"
        };

        var listener = new Listener<string>();
        col1.MusiciansChanged += listener.MusiciansChanged;
        col2.MusiciansChanged += listener.MusiciansChanged;

        var m1 = new Musician("Freddie", "Mercury", new DateTime(1946, 9, 5), "Freddie Mercury", 5);
        var m2 = new Musician("Brian", "May", new DateTime(1947, 7, 19), "Brian May", 5);
        var m3 = new Musician("Elton", "John", new DateTime(1947, 3, 25), "Elton John", 5);

        m1.AddSongs(new Song("Bohemian Rhapsody", Genre.Rock, new DateTime(1975, 10, 31)));
        m2.AddSongs(new Song("We Will Rock You", Genre.Rock, new DateTime(1977, 10, 7)));
        m3.AddSongs(new Song("Rocket Man", Genre.Pop, new DateTime(1972, 4, 17)));

        col1.AddMusicians(m1, m2);
        col2.AddMusicians(m3);

        m1.Rating = 4;
        m2.Pseudo = "Dr. Brian May";
        col1.Remove(m2);
        m2.Rating = 2;

        Console.WriteLine("Listener:");
        Console.WriteLine(listener.ToString());

        Console.WriteLine("\nRock Legends:");
        Console.WriteLine($"Last song release date: {col1.LastSongDate}");

        int year = 1946;
        Console.WriteLine($"\nBirthYear {year}:");
        foreach (var kvp in col1.BirthYear(year))
            Console.WriteLine($"  {kvp.Value.Pseudo}");

        Console.WriteLine("\nGrouped by BirthYear");
        foreach (var group in col1.GroupByBirthYear)
        {
            Console.WriteLine($"  Year {group.Key}:");
            foreach (var kvp in group)
                Console.WriteLine($"    {kvp.Value.Pseudo}");
        }
    }
}
