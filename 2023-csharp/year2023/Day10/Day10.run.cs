namespace ofzza.aoc.year2023.day10;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.pipemaze;

public partial class Day10: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    var maze = new PipeMaze(input);          
    // First
    if (info.ExecutionIndex == 1) {        
      // Output maze
      maze.Log(log, ConsoleLoggingLevel.Verbose);
      // Find path
      var path = maze.FindClosedLoopPath();
      log.WriteLine(ConsoleLoggingLevel.Verbose);
      maze.Log(log, ConsoleLoggingLevel.Verbose, path);
      log.WriteLine(ConsoleLoggingLevel.Verbose);
      log.WriteLine($"""- Path:""", ConsoleLoggingLevel.Verbose);
      log.WriteLine($"""  - Length: {path.Length}""", ConsoleLoggingLevel.Verbose);
      log.WriteLine($"""  - Coordinates: {string.Join(", ", path.Select(t => $"""{t.Coordinates[1]}x{t.Coordinates[0]} {t.Direction.ToString()}"""))}""", ConsoleLoggingLevel.All);
      // Output farthest distance
      return (int)Math.Floor((double)path.Length / (double)2);
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Output maze
      maze.Log(log, ConsoleLoggingLevel.Verbose);
      // Find path
      var path = maze.FindClosedLoopPath();
      log.WriteLine(ConsoleLoggingLevel.Verbose);
      maze.Log(log, ConsoleLoggingLevel.Verbose, path);
      log.WriteLine(ConsoleLoggingLevel.Verbose);
      log.WriteLine($"""- Path:""", ConsoleLoggingLevel.Verbose);
      log.WriteLine($"""  - Length: {path.Length}""", ConsoleLoggingLevel.Verbose);
      log.WriteLine($"""  - Coordinates: {string.Join(", ", path.Select(t => $"""{t.Coordinates[1]}x{t.Coordinates[0]} {t.Direction.ToString()}"""))}""", ConsoleLoggingLevel.All);
      // Fill inside/outside path
      var areas = maze.FindAreas(path);
      log.WriteLine(ConsoleLoggingLevel.Verbose);
      maze.Log(log, ConsoleLoggingLevel.Verbose, path, areas.InsideCoordinates, areas.OutsideCoordinates);
      log.WriteLine(ConsoleLoggingLevel.Verbose);
      log.WriteLine($"""- Areas:""", ConsoleLoggingLevel.Verbose);
      log.WriteLine($"""  - Inside: {areas.InsideCoordinates.Length}""", ConsoleLoggingLevel.Verbose);
      log.WriteLine($"""  - Outside: {areas.OutsideCoordinates.Length}""", ConsoleLoggingLevel.Verbose);
      // Output inside area
      return areas.InsideCoordinates.Length;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
