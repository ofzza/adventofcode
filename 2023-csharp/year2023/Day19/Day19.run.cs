namespace ofzza.aoc.year2023.day19;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.aplenty;

public partial class Day19: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // Initialize workflow processor
    var aplenty = new Aplenty(input);
    // First
    if (info.ExecutionIndex == 1) {
      // Process parts to find accepter and rejected, starting with "in" workflow
      var parts = aplenty.ProcessParts("in");
      // Add values or accepted parts
      var sum = 0;
      foreach (var part in parts.Accepted) sum += part.Value;
      // Find path of minimum heath loss
      return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Calculate number of possible acceptable parts
      return aplenty.GetPossibleAcceptablePartsCount("in");
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
