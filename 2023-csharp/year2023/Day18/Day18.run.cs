namespace ofzza.aoc.year2023.day18;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.lavaductlagoon;

public partial class Day18: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // Initialize lagoon
    var lagoon = new LavaductLagoon(input, info.ExecutionIndex == 1 ? false : true);
    // Dig and fill a path
    var path = lagoon.DigPath();
    var pathSize = lagoon.GetPathSize(path);
    // Log path
    log.WriteLine($"""- Found path: {pathSize}""");
    if (info.ExecutionIndex == 1) {
      lagoon.Log(path, new (long[] Start, long[] End)[] {}, log);
      log.WriteLine();
    }
    // Fill area within the path
    var area = lagoon.FillArea(path);
    var areaSize = lagoon.GetAreaSize(area);
    log.WriteLine($"""- Found area within the path: {areaSize}""");
    if (info.ExecutionIndex == 1) {
      lagoon.Log(path, area, log);
      log.WriteLine();
    }
    /// Return number of dug tiles (Remove one for duplicate starting point)
    return pathSize + areaSize;
  }
}
