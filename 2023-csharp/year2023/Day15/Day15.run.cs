namespace ofzza.aoc.year2023.day15;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.lenslibrary;

public partial class Day15: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);    
    // First
    if (info.ExecutionIndex == 1) {
      long sum = 0;
      foreach (var item in input) {
        sum += LensLibrary.CalculateHash(item);
      }
      return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Initialize
      var library = new LensLibrary(input);
      // Perform the initialization sequence
      return library.RunInitializationSequence();
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
