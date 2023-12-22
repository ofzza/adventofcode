namespace ofzza.aoc.year2023.day21;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.stepcounter;

public partial class Day21: ISolution<(int Count, string Tiles), long> {
  public long Run(SolutionExecutionRunInfo<(int Count, string Tiles)> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // Initialize step counter
    var counter = new StepCounter(input);
    // First
    if (info.ExecutionIndex == 1) {
      // Calculate distances from starting index
      var result = counter.CalculateTilesAccessibleWithinFixedNumberOfSteps(counter.StartCoordinates, info.InputValue!.Count, log);
      // Log distances
      counter.LogDistances(result.Tiles, log);
      // Return count of tiles of requested distance
      return result.Count;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Calculate distances from starting index
      var result = counter.CalculateTilesAccessibleWithinFixedNumberOfSteps(counter.StartCoordinates, info.InputValue!.Count, log, ConsoleLoggingLevel.Verbose, true);
      // Return count of tiles of requested distance
      return result.Count;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
