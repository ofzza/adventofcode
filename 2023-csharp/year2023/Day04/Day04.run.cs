namespace ofzza.aoc.year2023.day04;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day04: ISolution<string[], int> {
  public int Run(SolutionExecutionRunInfo<string[]> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var parsed = parse(info.InputValue!);
    // First
    if (info.ExecutionIndex == 1) {
        var sum = 0;
        for (var i=0; i<parsed.Length; i++) {
          var card = parsed[i];
          sum += (int)Math.Pow(2, this.CountMatches(card) - 1);
          log.Progress(i, parsed.Length);
        }
        return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
        var cardCounts = Enumerable.Repeat<int>(1, parsed.Length).ToArray();
        for (var i=0; i<parsed.Length; i++) {
          var card = parsed[i];
          var matchedCount = this.CountMatches(card);
          for (var j=0; j<matchedCount; j++) {
            cardCounts[i + j + 1] += cardCounts[i];
          }
          log.Progress(i, parsed.Length);
        }
        var sum = 0;
        foreach (var cardCount in cardCounts) sum += cardCount;
        return sum;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }

  private int CountMatches (Card card) {
    var hash = new HashSet<int>();          
      foreach (var n in card.winning) hash.Add(n);
      var count = 0;
      foreach (var n in card.selected) if (hash.Contains(n)) count += 1;
      return count;
  }
}
