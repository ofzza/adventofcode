namespace ofzza.aoc.year2023.day13;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.pointsofincidence;

public partial class Day13: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);    
    // First
    if (info.ExecutionIndex == 1) {        
      // Process each field
      var sum = 0;
      for (var i=0; i<input.Length; i++) {
        var field = input[i];
        // Process field
        var poi = new PointOfIncidence(field);
        // Find mirroring line
        poi.GenerateViews();
        var h = poi.FindHorizontalMirroringPlain();
        var v = h != null ? 0 : poi.FindVerticalMirroringPlain();
        if (h == null && v == null) throw new Exception("No mirroring plain found!");
        sum += h != null ? 100 * ((int)h! + 1) : ((int)v! + 1);
        // Log
        log.Progress(i, input.Length);
      }
      // Output result
      return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Process each field
      var sum = 0;
      for (var i=0; i<input.Length; i++) {
        var field = input[i];
        // Process field
        var poi = new PointOfIncidence(field);
        // Find mirroring line
        poi.GenerateViews();
        var h = poi.FindHorizontalSmudgedMirroringPlain();
        var v = h != null ? 0 : poi.FindVerticalSmudgedMirroringPlain();
        if (h == null && v == null) throw new Exception("No mirroring plain found!");
        sum += h != null ? 100 * ((int)h! + 1) : ((int)v! + 1);
        // Log
        log.Progress(i, input.Length);
      }
      // Output result
      return sum;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
