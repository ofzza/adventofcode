namespace ofzza.aoc.year2023.day23;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.longwalk;

public partial class Day23: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // Initialize walk
    var labyrinth = new LongWalk(input);
    // First
    if (info.ExecutionIndex == 1) {
      // Handle every time end is reached
      var longest = 0;
      labyrinth.OnReachedEndIndex += path => {
        // Store longest path          
        if (path.Length > longest) longest = path.Length;
        // Log path
        labyrinth.Log(path, log);
        log.WriteLine($"""- Found path of length: {path.Length - 1} (Longest yet = {longest - 1})""");
        log.WriteLine();
      };
      // Walk the labyrinth
      labyrinth.Walk(log, ConsoleLoggingLevel.Verbose, false);
      // Return longest path length
      return longest - 1;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Handle every time end is reached
      var longest = 0;
      labyrinth.OnReachedEndIndex += path => {
        // Store longest path          
        if (path.Length > longest) longest = path.Length;
        // Log path
        labyrinth.Log(path, log);
        log.WriteLine($"""- Found path of length: {path.Length - 1} (Longest yet = {longest - 1})""");
        log.WriteLine();
      };
      // Walk the labyrinth
      labyrinth.Walk(log, ConsoleLoggingLevel.Verbose, true);
      // Return longest path length
      return longest - 1;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
