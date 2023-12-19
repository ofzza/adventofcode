namespace ofzza.aoc.year2023.day17;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.clumsycrucible;

public partial class Day17: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // Initialize crubible
    var crucible = new ClumsyCrucible(input);
    // First
    if (info.ExecutionIndex == 1) {      
      // Find path of minimum heath loss
      return crucible.FindPathOfLeastHeathLoss(new long[] { 0, 0 }, new long[] { input[0].Length - 1, input.Length - 1 }, 0, 3, log);
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Find path of minimum heath loss
      return crucible.FindPathOfLeastHeathLoss(new long[] { 0, 0 }, new long[] { input[0].Length - 1, input.Length - 1 }, 4, 10, log); // 1270 too high
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
