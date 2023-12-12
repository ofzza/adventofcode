namespace ofzza.aoc.year2023.day11;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.cosmicexpansion;

public partial class Day11: ISolution<(long Expansion, string Space), long> {
  public long Run(SolutionExecutionRunInfo<(long Expansion, string Space)> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue.Space!);    
    // Initialize cosmic expansion
    var cosmos = new CosmicExpansion(input.Size) {
      Galaxies = input.Coordinates
    };
    // Draw the cosmos
    cosmos.Log(log);
    log.WriteLine();
    // Expand the cosmos
    var expanded = cosmos.ExpandEmptyRowsAndColumns(info.InputValue.Expansion, info.InputValue.Expansion);
    // Only try drawing expanded cosmos if first index (too large to draw otherwise)
    if (info.ExecutionIndex == 1) {        
      // Draw the expanded cosmos
      expanded.Log(log);
      log.WriteLine();
    }
    // Calculate distances between expanded galaxies
    long sum = 0;
    for (var i=0; i<(expanded.Galaxies.Length - 1); i++) {
      for (var j=i + 1; j<expanded.Galaxies.Length; j++) {
        var distance = Math.Abs(expanded.Galaxies[i][0] - expanded.Galaxies[j][0]) + Math.Abs(expanded.Galaxies[i][1] - expanded.Galaxies[j][1]);
        sum += distance;
      }
    }
    // Return result
    return sum;
  }
}
