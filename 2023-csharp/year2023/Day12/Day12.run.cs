namespace ofzza.aoc.year2023.day12;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.hotsprings;

public partial class Day12: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // First
    if (info.ExecutionIndex == 1) {        
      // Generate permutations
      long sum = 0;
      for (var i=0; i<input.Length; i++) {
        var line = input[i];
        // Log
        log.WriteLine($"""- Pattern: '{line.Springs}'""");
        log.WriteLine($"""  Checksum: {string.Join(',', line.Checksum)}""");
        // Parse and generate permutations
        var springs = new HotSprings(line.Springs, line.Checksum);
        var permutations = springs.GeneratePossiblePermutations(log.CheckLogLevel(ConsoleLoggingLevel.All), false);
        // Add permutations
        sum += permutations.Count;
        // Log
        if (log.CheckLogLevel(ConsoleLoggingLevel.All)) {
          foreach (var permutation in permutations.Permutations!) log.WriteLine($"""  - '{permutation}'""", ConsoleLoggingLevel.All);
        }
        log.WriteLine($"""  Count = {permutations.Count}""");
        log.Progress(i, input.Length);
      }
      // Return permutation 
      return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Expand input
      var expanded = input.Select(l => {
        var springs = new List<string>();
        var checksum = new List<int>();
        for (var i=0; i<5; i++) {
          springs.Add(l.Springs);
          checksum.AddRange(l.Checksum);
        }
        return ((string Springs, int[] Checksum))(string.Join('?', springs), checksum.ToArray());
      }).ToArray();
      // Generate permutations
      long sum = 0;
      for (var i=0; i<expanded.Length; i++) {
        var line = expanded[i];
        // Log
        log.WriteLine($"""- Pattern: '{line.Springs}'""");
        log.WriteLine($"""  Checksum: {string.Join(',', line.Checksum)}""");
        // Parse and generate permutations
        var springs = new HotSprings(line.Springs, line.Checksum);
        var permutations = springs.GeneratePossiblePermutations(false, log.CheckLogLevel(ConsoleLoggingLevel.All));
        // Add permutations
        sum += permutations.Count;
        // Log
        log.WriteLine($"""  Count = {permutations.Count}""");
        log.Progress(i, expanded.Length);
      }
      // Return permutation 
      return sum;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
