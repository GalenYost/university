using System.Collections;
using System.Diagnostics;
using System.Text;

public enum Genre
{
    Pop,
    Rock,
    Electronic,
    Classical,
}

public interface IDateAndCopy
{
    object DeepCopy();
    DateTime Date { get; set; }
}

public class TestCollections<TKey, TValue>
    where TKey : class
    where TValue : TKey
{
    private List<Person> persons = new List<Person>();
    private List<string> strings = new List<string>();
    private Dictionary<Person, Musician> personToMusician = new Dictionary<Person, Musician>();
    private Dictionary<string, Musician> stringToMusician = new Dictionary<string, Musician>();

    public static Musician GenerateMusician(int index)
    {
        var m = new Musician
        {
            Name = $"Name{index}",
            Surname = $"Surname{index}",
            Birthday = new DateTime(2025, 1, 1),
            Rating = 2,
        };

        m.Songs.Add(new Song($"Song{index}_A", Genre.Pop, new DateTime(2025, 1, 1)));
        m.Songs.Add(new Song($"Song{index}_B", Genre.Rock, new DateTime(2025, 1, 1)));
        m.Concerts.Add(new Concert($"Concert{index}", "City", new DateTime(2025, 1, 1)));

        return m;
    }

    public TestCollections(int count)
    {
        for (int i = 0; i < count; i++)
        {
            var musician = GenerateMusician(i);
            var person = musician.Person;
            persons.Add(person);
            strings.Add(musician.Surname);

            personToMusician[person] = musician;
            stringToMusician[musician.Surname] = musician;
        }
    }

    public string MeasureSearchTimes()
    {
        if (persons.Count == 0)
            return "Collections are empty.";

        int mid = persons.Count / 2;

        var targetPerson = persons[mid];
        var targetMusician = personToMusician[targetPerson];
        var targetString = strings[mid];

        var sw = new Stopwatch();
        var results = new List<string>();

        bool found;

        sw.Restart();
        found = persons.Contains(targetPerson);
        sw.Stop();
        results.Add($"List<Person>.Contains: {sw.ElapsedTicks} ticks | found={found}");

        sw.Restart();
        found = strings.Contains(targetString);
        sw.Stop();
        results.Add($"List<string>.Contains: {sw.ElapsedTicks} ticks | found={found}");

        sw.Restart();
        found = personToMusician.ContainsKey(targetPerson);
        sw.Stop();
        results.Add(
            $"Dictionary<Person,Musician>.ContainsKey: {sw.ElapsedTicks} ticks | found={found}"
        );

        sw.Restart();
        found = personToMusician.ContainsValue(targetMusician);
        sw.Stop();
        results.Add(
            $"Dictionary<Person,Musician>.ContainsValue: {sw.ElapsedTicks} ticks | found={found}"
        );

        sw.Restart();
        found = stringToMusician.ContainsKey(targetString);
        sw.Stop();
        results.Add(
            $"Dictionary<string,Musician>.ContainsKey: {sw.ElapsedTicks} ticks | found={found}"
        );

        sw.Restart();
        found = stringToMusician.ContainsValue(targetMusician);
        sw.Stop();
        results.Add(
            $"Dictionary<string,Musician>.ContainsValue: {sw.ElapsedTicks} ticks | found={found}"
        );

        return string.Join("\n", results);
    }
}

public class Person : IDateAndCopy, IComparable, IComparer<Person>
{
    private string name;
    private string surname;
    private DateTime birth;

    public string Name
    {
        get => name;
        set => name = value;
    }
    public string Surname
    {
        get => surname;
        set => surname = value;
    }
    public DateTime Birthday
    {
        get => birth;
        set => birth = value;
    }
    public int BirthYear
    {
        get => birth.Year;
        set => birth = new DateTime(value, birth.Month, birth.Day);
    }

    public virtual DateTime Date
    {
        get => Birthday;
        set => Birthday = value;
    }

    public Person(string first_name, string surname, DateTime birth)
    {
        this.Name = first_name;
        this.Surname = surname;
        this.Birthday = birth;
    }

    public Person()
        : this("Name", "Surname", new DateTime(2007, 6, 7)) { }

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
        if (ReferenceEquals(this, obj))
            return true;
        if (obj is not Person other)
            return false;
        return Name == other.Name && Surname == other.Surname && Birthday.Equals(other.Birthday);
    }

    public static bool operator ==(Person left, Person right)
    {
        if (ReferenceEquals(left, right))
            return true;
        if (left is null || right is null)
            return false;
        return left.Equals(right);
    }

    public static bool operator !=(Person left, Person right) => !(left == right);

    public override int GetHashCode() => HashCode.Combine(Name, Surname, Birthday);

    public virtual object DeepCopy() => new Person(Name, Surname, Birthday);

    public int CompareTo(object obj)
    {
        if (obj == null)
            return 1;
        if (obj is not Person other)
            throw new ArgumentException("Object is not a Person");

        return string.Compare(this.Surname, other.Surname, StringComparison.Ordinal);
    }

    public int Compare(Person x, Person y)
    {
        if (x is null && y is null)
            return 0;
        if (x is null)
            return -1;
        if (y is null)
            return 1;

        return DateTime.Compare(x.Birthday, y.Birthday);
    }
}

public class Song : IDateAndCopy
{
    public string Name { get; set; }
    public Genre SongGenre { get; set; }
    public DateTime ReleaseDate { get; set; }

    public DateTime Date
    {
        get => ReleaseDate;
        set => ReleaseDate = value;
    }

    public Song(string name, Genre genre, DateTime release)
    {
        this.Name = name;
        this.SongGenre = genre;
        this.ReleaseDate = release;
    }

    public Song()
        : this("Unnamed", Genre.Pop, DateTime.Now) { }

    public override string ToString() =>
        $"{this.Name} ({this.SongGenre}) - {this.ReleaseDate.ToShortDateString()}";

    public override bool Equals(object obj)
    {
        if (ReferenceEquals(this, obj))
            return true;
        if (obj is not Song other)
            return false;
        return Name == other.Name
            && SongGenre == other.SongGenre
            && ReleaseDate.Equals(other.ReleaseDate);
    }

    public static bool operator ==(Song left, Song right)
    {
        if (ReferenceEquals(left, right))
            return true;
        if (left is null || right is null)
            return false;
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
            if (position < 0 || position >= titles.Count)
                throw new InvalidOperationException();
            return titles[position];
        }
    }

    public bool MoveNext()
    {
        position++;
        return position < titles.Count;
    }

    public void Reset()
    {
        position = -1;
    }
}

public class MusicianRatingComparer : IComparer<Musician>
{
    public int Compare(Musician x, Musician y)
    {
        if (x is null && y is null)
            return 0;
        if (x is null)
            return -1;
        if (y is null)
            return 1;

        return y.Rating.CompareTo(x.Rating);
    }
}

public class Musician : Person, IDateAndCopy, IEnumerable
{
    private string pseudo;
    private double rating;
    private List<Song> songs;
    private List<Concert> concerts;

    public Person Person
    {
        get { return new Person(Name, Surname, Birthday); }
        set
        {
            Name = value.Name;
            Surname = value.Surname;
            Birthday = value.Birthday;
        }
    }

    public string Pseudo
    {
        get => pseudo;
        set => pseudo = value;
    }
    public double Rating
    {
        get => rating;
        set
        {
            if (value <= 0 || value > 5)
                throw new ArgumentOutOfRangeException(
                    nameof(value),
                    "Rating must be between 1 and 5"
                );
            rating = value;
        }
    }
    public List<Song> Songs
    {
        get => songs;
        set => songs = value;
    }
    public List<Concert> Concerts
    {
        get => concerts;
        set => concerts = value;
    }

    public Song LastSong =>
        this.Songs?.OrderByDescending(song => song.ReleaseDate).FirstOrDefault();
    public override DateTime Date
    {
        get => base.Date;
        set => base.Date = value;
    }

    IEnumerator IEnumerable.GetEnumerator() => new MusicianEnumerator(this);

    public Musician(string name, string surname, DateTime birthday, string pseudo, double rating)
        : base(name, surname, birthday)
    {
        this.pseudo = pseudo;
        this.rating = rating;
        songs = new();
        concerts = new();
    }

    public Musician()
        : this("Name", "Surname", new DateTime(2007, 6, 7), "Unknown", 0) { }

    public void AddSongs(params Song[] newSongs) => songs.AddRange(newSongs);

    public void AddConcerts(params Concert[] newConcerts) => concerts.AddRange(newConcerts);

    public IEnumerable<object> GetAllItems()
    {
        foreach (var s in songs)
            yield return s;
        foreach (var c in concerts)
            yield return c;
    }

    public IEnumerable<Song> GetSongsFromLastYears(int n)
    {
        foreach (var s in songs)
            if (s.ReleaseDate >= DateTime.Now.AddYears(-n))
                yield return s;
    }

    public IEnumerable<Concert> GetConcertsByCity(string city)
    {
        foreach (var c in concerts)
            if (string.Equals(c.City, city, StringComparison.OrdinalIgnoreCase))
                yield return c;
    }

    public IEnumerable<Song> GetSongsByGenre(Genre genre)
    {
        foreach (var s in songs)
            if (s.SongGenre == genre)
                yield return s;
    }

    public override string ToString()
    {
        var songsStr =
            songs.Count > 0 ? string.Join(", ", songs.Select(s => s.ToString())) : "No songs";
        var concertsStr =
            concerts.Count > 0
                ? string.Join(", ", concerts.Select(c => c.ToString()))
                : "No concerts";
        return $"{base.ToString()}, Stage Name: {Pseudo}, Rating: {Rating}, Songs: [{songsStr}], Concerts: [{concertsStr}]";
    }

    public override string ToShortString() =>
        $"{base.ToShortString()}, Stage: {Pseudo}, Last Song: {LastSong?.Name}";

    public override bool Equals(object obj)
    {
        if (ReferenceEquals(this, obj))
            return true;
        if (obj is not Musician other)
            return false;

        bool songsEqual = songs.SequenceEqual(other.songs);
        bool concertsEqual = concerts.SequenceEqual(other.concerts);

        return base.Equals(other)
            && Pseudo == other.Pseudo
            && Rating == other.Rating
            && songsEqual
            && concertsEqual;
    }

    public static bool operator ==(Musician left, Musician right)
    {
        if (ReferenceEquals(left, right))
            return true;
        if (left is null || right is null)
            return false;
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
        var copy = new Musician(
            personCopy.Name,
            personCopy.Surname,
            personCopy.Birthday,
            Pseudo,
            Rating
        )
        {
            Songs = songsCopy,
            Concerts = concertsCopy,
        };
        return copy;
    }
}

public class MusicianCollection
{
    private List<Musician> musicians = new List<Musician>();

    // назва колекції
    public string Name { get; set; }

    public event MusicianListHandler MusicianCountChanged;
    public event MusicianListHandler MusicianReferenceChanged;

    private void OnMusicianCountChanged(string changeInfo, Musician m)
    {
        MusicianCountChanged?.Invoke(this, new MusicianListHandlerEventArgs(Name, changeInfo, m));
    }

    private void OnMusicianReferenceChanged(string changeInfo, Musician m)
    {
        MusicianReferenceChanged?.Invoke(this, new MusicianListHandlerEventArgs(Name, changeInfo, m));
    }

    public void AddDefaults()
    {
        var m1 = new Musician
        {
            Name = "John",
            Surname = "Doe",
            Birthday = new DateTime(1980, 5, 12),
            Rating = 4.7,
        };
        m1.Songs.AddRange(
            new[]
            {
                new Song("First Song", Genre.Rock, new DateTime(2021, 5, 1)),
                new Song("Second Song", Genre.Pop, new DateTime(2022, 3, 14)),
            }
        );
        m1.Concerts.AddRange(
            new[]
            {
                new Concert("Summer Tour", "New York", new DateTime(2020, 6, 1)),
                new Concert("Festival", "London", new DateTime(2021, 8, 15)),
            }
        );

        var m2 = new Musician
        {
            Name = "Alice",
            Surname = "Smith",
            Birthday = new DateTime(1990, 3, 22),
            Rating = 4.9,
        };
        m2.Songs.Add(new Song("Single", Genre.Pop, new DateTime(2022, 11, 1)));
        m2.Concerts.Add(new Concert("Concert", "New York", new DateTime(2022, 11, 5)));

        var m3 = new Musician
        {
            Name = "Bob",
            Surname = "Johnson",
            Birthday = new DateTime(1975, 12, 30),
            Rating = 4.5,
        };
        m3.Songs.AddRange(
            new[]
            {
                new Song("Rock Anthem", Genre.Rock, new DateTime(2021, 12, 7)),
                new Song("Ballad", Genre.Electronic, new DateTime(2021, 12, 8)),
            }
        );
        m3.Concerts.AddRange(
            new[]
            {
                new Concert("Summer Tour", "New York", new DateTime(2019, 4, 20)),
                new Concert("Festival", "London", new DateTime(2023, 2, 10)),
            }
        );

        musicians.AddRange(new[] { m1, m2, m3 });
        OnMusicianCountChanged("Added default musician", m1);
    }

    public void AddMusicians(params Musician[] items)
    {
        if (items == null)
            return;
        musicians.AddRange(items);
        foreach (var m in items)
            OnMusicianCountChanged("Added musician", m);
    }

    public bool Remove(int j)
    {
        if (j < 0 || j >= musicians.Count)
            return false;

        var removed = musicians[j];
        musicians.RemoveAt(j);
        OnMusicianCountChanged($"Removed musician at index {j}", removed);
        return true;
    }

    // індексатор
    public Musician this[int index]
    {
        get
        {
            if (index < 0 || index >= musicians.Count)
                throw new ArgumentOutOfRangeException(nameof(index));
            return musicians[index];
        }
        set
        {
            if (index < 0 || index >= musicians.Count)
                throw new ArgumentOutOfRangeException(nameof(index));
            musicians[index] = value;
            OnMusicianReferenceChanged($"Replaced musician at index {index}", value);
        }
    }

    public override string ToString()
    {
        var sb = new StringBuilder();
        foreach (var m in musicians)
        {
            sb.AppendLine(m.ToString());
            sb.AppendLine("  Songs:");
            foreach (var song in m.Songs)
                sb.AppendLine($"    - {song}");

            sb.AppendLine("  Concerts:");
            foreach (var c in m.Concerts)
                sb.AppendLine($"    - {c:d}");
            sb.AppendLine();
        }
        return sb.ToString();
    }

    public virtual string ToShortString()
    {
        var sb = new StringBuilder();
        foreach (var m in musicians)
        {
            sb.Append($"{m.Name} {m.Surname}, born {m.Birthday:d}, rating={m.Rating}; ");
            int songCount = m.Songs.Count;
            string lastSong = songCount > 0 ? m.Songs[^1].Name : "(no songs)";
            int concertCount = m.Concerts.Count;

            sb.Append(
                $"last song: {lastSong}, total songs: {songCount}, total concerts: {concertCount}"
            );
            sb.AppendLine();
        }
        return sb.ToString();
    }

    public void SortBySurname()
    {
        musicians.Sort();
    }

    public void SortByBirthDate()
    {
        var comparer = new Person();
        musicians.Sort((x, y) => comparer.Compare(x, y));
    }

    public void SortByRating()
    {
        musicians.Sort(new MusicianRatingComparer());
    }
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

    public Concert()
        : this("Untitled Concert", "Unknown City", DateTime.Now) { }

    public override string ToString()
    {
        return $"Concert: {Title}, City: {City}, Date: {Date.ToShortDateString()}";
    }
}

public delegate void MusicianListHandler(object source, MusicianListHandlerEventArgs args);

public class MusicianListHandlerEventArgs : EventArgs
{
    public string CollectionName { get; set; }
    public string ChangeInfo { get; set; }
    public Musician ChangedMusician { get; set; }

    public MusicianListHandlerEventArgs(string collectionName, string changeInfo, Musician musician)
    {
        CollectionName = collectionName;
        ChangeInfo = changeInfo;
        ChangedMusician = musician;
    }

    public override string ToString()
    {
        return $"Collection: {CollectionName}, Change: {ChangeInfo}, Musician: {ChangedMusician?.ToShortString()}";
    }
}

public class Listener
{
    private List<ListEntry> changes = new List<ListEntry>();

    public void AddEntry(object source, MusicianListHandlerEventArgs args)
    {
        changes.Add(new ListEntry(args.CollectionName, args.ChangeInfo, args.ChangedMusician));
    }

    public override string ToString()
    {
        var sb = new StringBuilder();
        foreach (var entry in changes)
            sb.AppendLine(entry.ToString());
        return sb.ToString();
    }
}

public class ListEntry
{
    public string CollectionName { get; set; }
    public string ChangeInfo { get; set; }
    public int Index { get; set; }
    public Musician ChangedMusician { get; set; }

    public ListEntry(string collectionName, string changeInfo, Musician musician)
    {
        CollectionName = collectionName;
        ChangeInfo = changeInfo;
        ChangedMusician = musician;
    }

    public override string ToString()
    {
        return $"[{CollectionName}] {ChangeInfo}: {ChangedMusician?.ToShortString()}";
    }
}

public class Program
{
    public static void Main()
    {
        var collection = new MusicianCollection();
        collection.AddMusicians(
            new Musician
            {
                Name = "Anna",
                Surname = "Kowalska",
                Birthday = new DateTime(1991, 4, 20),
                Rating = 4.3,
            },
            new Musician
            {
                Name = "Boris",
                Surname = "Ivanov",
                Birthday = new DateTime(1987, 2, 10),
                Rating = 4.7,
            },
            new Musician
            {
                Name = "Zoya",
                Surname = "Abramova",
                Birthday = new DateTime(1995, 12, 5),
                Rating = 4.1,
            }
        );

        Console.WriteLine("Original collection:");
        Console.WriteLine(collection.ToShortString());

        collection.SortBySurname();
        Console.WriteLine("\nSorted by surname:");
        Console.WriteLine(collection.ToShortString());

        collection.SortByBirthDate();
        Console.WriteLine("\nSorted by birth date:");
        Console.WriteLine(collection.ToShortString());

        collection.SortByRating();
        Console.WriteLine("\nSorted by rating:");
        Console.WriteLine(collection.ToShortString());

        var tests = new TestCollections<Person, Musician>(1000);
        Console.WriteLine("\nTestCollections search timings:");
        Console.WriteLine(tests.MeasureSearchTimes());
    }
}
