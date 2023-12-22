namespace ofzza.aoc.year2023.day09;

using ofzza.aoc.utils;
using ofzza.aoc.utils.primes;

public partial class Day09: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // First
    if (info.ExecutionIndex == 1) {        
      // Process all histories and extrapolate next value
      long sum = 0;
      for (var i=0; i<input.Length; i++) {
        var history = new NumericSequence<long>() { Sequence = input[i] };
        var next = history.ExtrapolateUsingDifferencesTableMethod();
        sum += next;

        log.WriteLine($"""- {string.Join(", ", history.Sequence)}: {next}""");
        log.Progress(i, input.Length);
      }
      return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Process all histories and extrapolate preceding value
      long sum = 0;
      for (var i=0; i<input.Length; i++) {
        var history = new NumericSequence<long>() { Sequence = input[i] };
        var next = history.ExtrapolateUsingDifferencesTableMethod(Direction.Left);
        sum += next;

        log.WriteLine($"""- {string.Join(", ", history.Sequence)}: {next}""");
        log.Progress(i, input.Length);
      }
      return sum;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
