namespace ofzza.aoc.year2023.day07;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day07: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // First
    if (info.ExecutionIndex == 1) {        
      // Parse input
      var parsed = parse(info.InputValue!);
      // Sort rounds by hand type and value
      var sortedHands = parsed.Rounds.ToList();
      sortedHands.Sort(new RoundHandValueComparer());
      // Calculate bets
      var sum = 0;
      for (var i=0; i<sortedHands.Count; i++) {
        sum += (i + 1) * sortedHands[i].Bet;
      }
      return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Parse input (taking jokers into account)
      var parsed = parse(info.InputValue!, true);
      // Sort rounds by hand type and value
      var sortedHands = parsed.Rounds.ToList();
      sortedHands.Sort(new RoundHandValueComparer());
      // Calculate bets
      var sum = 0;
      for (var i=0; i<sortedHands.Count; i++) {
        sum += (i + 1) * sortedHands[i].Bet;
      }
      return sum;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
