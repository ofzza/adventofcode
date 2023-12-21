namespace ofzza.aoc.year2023.day16;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.floorwillbelava;

public partial class Day16: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);    
    // Initialize floor
    var floor = new FloorWillBeLava(input);
    // First
    if (info.ExecutionIndex == 1) {      
      // Calculate beam trajectory and return number of energized elements
      return floor.CalculateEnergizationFromAStartingPoint(0, Direction.Right, log);
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      return floor.FindOptimalEnergization(log); // 7920 too low 
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
