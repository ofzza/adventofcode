namespace ofzza.aoc.year2023.utils.stepcounter;

using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

public class StepCounter {

  /// <summary>
  /// Holds map tiles
  /// </summary>
  private char[] Tiles { init; get; }
  private MatrixIndexer Index { init; get; }

  /// <summary>
  /// Starting position
  /// </summary>
  public long[] StartCoordinates { init; get; }

  public StepCounter (char[][] input) {
    // Store input and index it
    this.Tiles = string.Join("", input.Select(l => string.Join("", l))).ToArray();
    this.Index = new MatrixIndexer(new long[] { input[0].Length, input.Length }) {
      InfinitePlain = true
    };
    // Find starting position
    this.StartCoordinates = new long[] {};
    for (var i=0; i<this.Tiles.Length; i++) {
      if (this.Tiles[i] == 'S') {
        this.StartCoordinates = this.Index.IndexToCoordinates(i);
        break;
      }
    }
  }

  /// <summary>
  /// Calculates distances from a position to all other available positions
  /// </summary>
  /// <param name="startingCoordinates">Starting coordinates to calculate distances from</param>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  /// <param name="infinitePlain">If running on an infinite plain</param>
  /// <returns>Distance of each tile from the starting distance</returns>
  public (long Count, long[][] Tiles) CalculateTilesAccessibleWithinFixedNumberOfSteps (long[] startingCoordinates, int stepsCount, Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, bool infinitePlain = false) {
    var result = this.CalculateMinimalDistanceToEachTile(startingCoordinates, stepsCount, log, level, infinitePlain);
    return (result.ExactDistanceCoordinatesCount, result.ExactDistanceCoordinates);
  }
  
  /// <summary>
  /// Calculates distances from a position to all other available positions
  /// </summary>
  /// <param name="startingCoordinates">Starting coordinates to calculate distances from</param>
  /// <param name="maxStepsCount">Maximum number of steps, should stop the search after this</param>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  /// <param name="infinitePlain">If running on an infinite plain</param>
  /// <returns>Distance of each tile from the starting distance</returns>
  private (long ExactDistanceCoordinatesCount, long[][] ExactDistanceCoordinates) CalculateMinimalDistanceToEachTile (long[] startingCoordinates, int maxStepsCount, Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, bool infinitePlain = false) {
    // Get index
    var index = !infinitePlain ? this.Index : new MatrixIndexer(this.Index.Dimensions) { InfinitePlain = true };
    // Initialize exact distance tiles
    var startingCoordinatesKey = $"""{startingCoordinates[0]},{startingCoordinates[1]}""";
    long exactDistanceCoordinatesCount = 0;
    var exactDistanceCoordinates = new Dictionary<string, long[]>();
    if (maxStepsCount % 2 == 0) {
      exactDistanceCoordinatesCount++;
      if (!infinitePlain) exactDistanceCoordinates.Add(startingCoordinatesKey, startingCoordinates);
    }
    // Initialize tiles
    var previous = new Dictionary<string, long[]>();
    var current = new Dictionary<string, long[]>() { { startingCoordinatesKey, startingCoordinates } };
    var next = new Dictionary<string, long[]>();
    // Set distances for next step
    for (var i=0; i<maxStepsCount && current.Count>0; i++) {
      var distance = i + 1;
      var reachable = distance % 2 == maxStepsCount % 2;
      // Find steps available in next step and store distances
      foreach (var tile in current.Values) {
        // var neighbors = index.GetNeighboringCoordinates(tile, false);
        var neighbors = new long[][] { // Faster(ish), more direct, 2D only version
          new long[] { tile[0] + 1, tile[1] },
          new long[] { tile[0] - 1, tile[1] },
          new long[] { tile[0], tile[1] + 1 },
          new long[] { tile[0], tile[1] - 1 }
        };
        foreach (var neighbor in neighbors) {
          var neighborKey = $"""{neighbor[0]},{neighbor[1]}""";
          if (this.Tiles[index.CoordinatesToIndex(neighbor)] != '#' && !previous.ContainsKey(neighborKey)) {
            // If already processed, move on
            if (next.ContainsKey(neighborKey)) {
              continue;
            }            
            // If not already processed, schedule as next point to search
            else {
              next.Add(neighborKey, neighbor);
            }
            // Check if reachable in exact distance
            if (reachable) {
              exactDistanceCoordinatesCount++;              
              if (!infinitePlain) exactDistanceCoordinates.Add(neighborKey, neighbor);
            };
          }
        }
      }
      // Log
      if (log.CheckLogLevel(ConsoleLoggingLevel.All)) {
        log.WriteLine($"""> Step #{i + 1}""", ConsoleLoggingLevel.All);
        this.LogDistances(exactDistanceCoordinates.Values.ToArray(), log, ConsoleLoggingLevel.All);
        log.WriteLine(ConsoleLoggingLevel.All);
      }
      // Log progress
      if (i % 100 == 0) log.Progress(i, maxStepsCount, level);
      // Ready next step
      previous = current;
      current = next;
      next = new Dictionary<string, long[]>();
      
    }
    // Return distances
    return (exactDistanceCoordinatesCount, exactDistanceCoordinates.Values.ToArray());
  }

  /// <summary>
  /// Outputs maze visual representation to Console
  /// </summary>
  /// <param name="exactDistanceCoordinates">Coordinates to log as matches</param>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  public void LogDistances (long[][] exactDistanceCoordinates, Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    var buffer = new ConsoleBuffer(this.Index);
    buffer.FillBuffer('.');
    for (var i=0; i<this.Tiles.Length; i++) {
      buffer.WriteToBuffer(this.Tiles[i], i);
    }
    for (var i=0; i<exactDistanceCoordinates.Length; i++) {
      buffer.WriteToBuffer('+', this.Index.CoordinatesToIndex(exactDistanceCoordinates[i]));
    }
    buffer.WriteToLog(log, level);
  }

}
