namespace ofzza.aoc.year2023.utils.hotsprings;

public class HotSprings {

  /// <summary>
  /// Holds damaged definitions of a row of springs
  /// </summary>
  private string Springs { init; get; }
  /// <summary>
  /// Holds a row of springs checksum
  /// </summary>
  private int[] Checksum { init; get; }

  // PRecalculated values cache
  private Dictionary<int, Dictionary<string, Dictionary<int, long>>> Cache = new Dictionary<int, Dictionary<string, System.Collections.Generic.Dictionary<int, long>>>();

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="springs">Damaged definitions of a row of springs</param>
  /// <param name="checksum">Row of springs checksum</param>
  public HotSprings (string springs, int[] checksum) {
    // Store properties
    this.Checksum = checksum;
    // Store reduced springs
    this.Springs = string.Join('.', springs.Split('.').Where(s => s.Length > 0));
  }

  /// <summary>
  /// Generates all permutations of springs matching the provided checksum
  /// </summary>
  /// <param name="permutations">Weather or not to keep track of permutations</param>
  /// <returns>All permutations of springs matching the provided checksum</returns>
  public (long Count, List<string>? Permutations) GeneratePossiblePermutations (bool permutations, bool walkthrough) {
    // Reset cache
    this.Cache.Clear();
    // Generate permutations    
    return this.generatePermutations(permutations ? new List<string>() : null, walkthrough, "", new int[this.Springs.Length]);
  }

  public (long Count, List<string>? Permutations) generatePermutations (List<string>? permutations, bool walkthrough, string current, int[] checksum, int checksumLength = 0, int currentGroupLength = 0) {
    // Toggle cache
    var caching = true;
    
    // Try reading from cache
    if (caching && permutations == null) {
      var cached = this.ReadFromCache(current.Length, checksum, checksumLength, currentGroupLength);
      if (cached != null) {
        if (walkthrough) {
          System.Console.ForegroundColor = ConsoleColor.DarkGreen;
          System.Console.WriteLine("> CACHE Hit!");
        }
        return ((long)cached!, permutations);
      }
    }

    // Early exit checks
    {
      // Check if group in progress is too large
      if (currentGroupLength > 0 && (this.Checksum.Length <= checksumLength || this.Checksum[checksumLength] < currentGroupLength)) {
        return (0, permutations);
      }
      // Check if all groups full, but can't have remaining group items in front
      if (checksumLength == this.Checksum.Length) {
        for (var i=current.Length; i<this.Springs.Length; i++) if (this.Springs[i] == '#') {
          return (0, permutations);
        }
      }
      // Check if enough characters remaining to fill known existing groups
      if (Math.Ceiling((double)(this.Springs.Length - current.Length) / 2) < (this.Checksum.Length - (checksumLength + (currentGroupLength > 0 ? 1 : 0)))) {
        return (0, permutations);
      }
    }
    
    // If fully composed permutation, store and return
    if (current.Length == this.Springs.Length) {
      if (checksumLength == this.Checksum.Length) {
        permutations?.Add(current);
        if (walkthrough) {
          System.Console.ForegroundColor = ConsoleColor.Green;
          System.Console.WriteLine("> Found a solution!");
        }
        return (1, permutations);
      } else {
        return (0, permutations);
      }
    }

    if (walkthrough) {
      System.Console.ForegroundColor = ConsoleColor.White;
      System.Console.Write(current);
      System.Console.ForegroundColor = ConsoleColor.DarkGray;
      System.Console.Write(this.Springs.Substring(current.Length));
      System.Console.Write("   |   ");
      System.Console.ForegroundColor = ConsoleColor.White;
      System.Console.Write(current.Length + " | ");
      System.Console.ForegroundColor = ConsoleColor.White;
      System.Console.Write(string.Join(',', checksum.Take(checksumLength).ToArray()));
      System.Console.ForegroundColor = ConsoleColor.DarkGray;
      System.Console.Write((checksumLength > 0 ? "," : "") + string.Join(',', this.Checksum.Skip(checksumLength).Take(this.Checksum.Length - checksumLength).ToArray()));
      System.Console.ForegroundColor = ConsoleColor.White;
      System.Console.Write(" | " +  currentGroupLength);
      System.Console.WriteLine();
      Thread.Sleep(100);
    }

    // Append current permutations
    var next = this.Springs[current.Length];
    // Use known character
    if (next != '?') {
      // Fast forward if multiple '#' in a row
      if (next == '#') for (var i=current.Length + 1; i<this.Springs.Length; i++) if (this.Springs[i] == '#') { current += '#'; currentGroupLength++; } else break;
      // Fast forward if multiple '#' in a row
      if (next == '.') for (var i=current.Length + 1; i<this.Springs.Length; i++) if (this.Springs[i] == '.') { current += '.'; } else break;
      // Process '#'
      var count = this.generatePermutationsNext(next, permutations, walkthrough, current, checksum, checksumLength, currentGroupLength).Count;
      if (caching && permutations == null && current.Length > 0) this.WriteToCache(current.Length, checksum, checksumLength, currentGroupLength, count);
      return (count, permutations);
    }
    // Sum options for possible characters
    else {
      // Interpret '?' as '.'
      var countA = this.generatePermutationsNext('#', permutations, walkthrough, current, checksum, checksumLength, currentGroupLength).Count;
      // Interpret '?' as '#'
      var countB = this.generatePermutationsNext('.', permutations, walkthrough, current, checksum, checksumLength, currentGroupLength).Count;
      // Return sum of partial counts
      var count = countA + countB;
      if (caching && permutations == null && current.Length > 0) this.WriteToCache(current.Length, checksum, checksumLength, currentGroupLength, count);
      return (count, permutations);
    }    
    
  }

  public (long Count, List<string>? Permutations) generatePermutationsNext (char next, List<string>? permutations, bool walkthrough, string current, int[] checksum, int checksumLength = 0, int currentGroupLength = 0) {
    // Get last character
    char? last = current.Length > 0 ? current[current.Length - 1] : null;
    // If last character
    if (current.Length + 1 == this.Springs.Length && (currentGroupLength > 0 || next == '#')) {
      // Store to checksum and verify
      checksum[checksumLength] = currentGroupLength + (next == '#' ? 1 : 0);
      if (checksumLength >= this.Checksum.Length || checksum[checksumLength] != this.Checksum[checksumLength]) return (0, permutations);
      // Continue with next character and updated checksums
      return this.generatePermutations(permutations, walkthrough, $"""{current}{next}""", checksum, checksumLength + 1, 0);
    }
    // If group just finished ...
    else if (last == '#' && next == '.') {
      // Store to checksum and verify
      checksum[checksumLength] = currentGroupLength + (next == '#' ? 1 : 0);
      if (checksumLength >= this.Checksum.Length || checksum[checksumLength] != this.Checksum[checksumLength]) return (0, permutations);
      // Continue with next character and updated checksums
      return this.generatePermutations(permutations, walkthrough, $"""{current}{next}""", checksum, checksumLength + 1, 0);
    }
    // Else ...
    else {
      // Continue with next character
      return this.generatePermutations(permutations, walkthrough, $"""{current}{next}""", checksum, checksumLength, currentGroupLength + (next == '#' ? 1 : 0));
    }
  }

  private long? ReadFromCache (int currentLength, int[] checksum, int checksumLength, int currentGroupLength) {
    // Check L1 cache
    if (this.Cache.ContainsKey(currentLength)) {
      var l2Cache = this.Cache[currentLength];
      var checksumKey = string.Join("", checksum.Take(checksumLength));
      // Check L2 Cache
      if (l2Cache.ContainsKey(checksumKey)) {
        var l3Cache = l2Cache[checksumKey];
        // Check L3 cache
        if (l3Cache.ContainsKey(currentGroupLength)) {
          var value = l3Cache[currentGroupLength];
          return value;
        }
      }
    }
    return null;
  }

  private void WriteToCache (int currentLength, int[] checksum, int checksumLength, int currentGroupLength, long value) {
    // Initialize L2 cache
    if (!this.Cache.ContainsKey(currentLength)) this.Cache.Add(currentLength, new());
    var l2Cache = this.Cache[currentLength];
    // Initialize L3 cache
    var checksumKey = string.Join("", checksum.Take(checksumLength));
    if (!l2Cache.ContainsKey(checksumKey)) l2Cache.Add(checksumKey, new());
    var l3Cache = l2Cache[checksumKey];
    // Write to L3 cache
    if (!l3Cache.ContainsKey(currentGroupLength)) {
      l3Cache.Add(currentGroupLength, value);
    }
    // If already written, check if same
    else if (l3Cache[currentGroupLength] != value) throw new Exception("Caching different value under same key! Should never happen!");
    // else if (l3Cache[currentGroupLength] == value) throw new Exception("Caching same value and key multiple times! Should never happen!");    
  }

}
