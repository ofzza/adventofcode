namespace ofzza.aoc.year2023.day22;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.sandslabs;

public partial class Day22: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // Initialize sand slabs
    var slabs = new SandSlabs(input);
    // First
    if (info.ExecutionIndex == 1) {
      // Stack brocks
      var stacked = slabs.Stack(log, ConsoleLoggingLevel.All);
      // Find critical slabs
      var criticals = slabs.FindCriticalSlabs(stacked);
      // Log
      log.WriteLine();
      log.WriteLine($"""Found {criticals.Length} critical slabs: {string.Join(", ", criticals.Select(i => ConsoleBuffer.IndexToLetter(i)))}""");
      // Return count of non-critical slabs
      return stacked.Length - criticals.Length;
    }
    // Second
    else if (info.ExecutionIndex == 2) { // 1103 too low, 1218 too low, 1103
      // Stack brocks
      log.WriteLine("Stacking slabs ...");
      var stacked = slabs.Stack(log, ConsoleLoggingLevel.All);
      // Return count of non-critical slabs
      var criticals = slabs.FindCriticalSlabs(stacked);
      // Log
      log.WriteLine();
      log.WriteLine($"""Found {criticals.Length} critical slabs: {string.Join(", ", criticals.Select(i => ConsoleBuffer.IndexToLetter(i)))}""");
      // Find criticality of most critical slab
      log.WriteLine();
      log.WriteLine("Evaluating criticality of each slab ...");
      long sum = 0;
      for (var i=0; i<criticals.Length; i++) {
        var critical = criticals[i];

        // Evaluate slab
        var eval = slabs.EvaluateCriticalSlab(stacked, critical, log, ConsoleLoggingLevel.All);
        sum += eval.Length - 1;
        // Log
        log.WriteLine($"""  ... disintegrating slab {ConsoleBuffer.IndexToLetter(critical)} would disintegrate: {eval.Length} slabs""");
        log.WriteLine($"""  ... disintegrating slab {ConsoleBuffer.IndexToLetter(critical)} would disintegrate: {string.Join(", ", eval.Select(i => ConsoleBuffer.IndexToLetter(i)))}""", ConsoleLoggingLevel.All);
        log.WriteLine(ConsoleLoggingLevel.All);
        log.Progress(i, criticals.Length);
      }
      return sum;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
