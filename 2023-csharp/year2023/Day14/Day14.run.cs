namespace ofzza.aoc.year2023.day14;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.parabolicreflectordish;

public partial class Day14: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);    
    var dish = new ParabolicReflectorDish(input);
    // First
    if (info.ExecutionIndex == 1) {
      // Draw the dish
      dish.Log(log);
      log.WriteLine();
      // Tilt the dish
      dish.Tilt(Direction.Top);
      // (Re)Draw the dish
      dish.Log(log);
      log.WriteLine();
      // Output
      return dish.CalculateWeight();
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Draw the dish
      dish.Log(log);
      log.WriteLine();
      // Cycle the dish
      dish.CycleDish(1000000000, log);
      // (Re)Draw the dish
      dish.Log(log);
      log.WriteLine();
      // Output
      return dish.CalculateWeight();
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
