namespace ofzza.aoc.year2023.utils.clumsycrucible;

using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

public class ClumsyCrucible {

  /// <summary>
  /// Holds heat loss information for each tile
  /// </summary>
  private int[] HeatLossPerTile { init; get; }
  private MatrixIndexer Index { init; get; }

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="input">Heat loss information for each tile</param>
  public ClumsyCrucible (char[][] input) {
    // Store hear loss information per tile
    this.HeatLossPerTile = string.Join("", input.Select(l => string.Join("", l))).Select(c => int.Parse(c.ToString())).ToArray();
    this.Index = new MatrixIndexer(new long[] { input[0].Length, input.Length });
  }

  public int FindPathOfLeastHeathLoss (long[] start, long[] end, int minLength, int maxLength, Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    // Get start and end indices
    var startIndex = (int)this.Index.CoordinatesToIndex(start);
    var endIndex = (int)this.Index.CoordinatesToIndex(end);

    // Initialize position(s)' tracking
    var closestToEnd = long.MaxValue;
    var minTotalHeathLoss = int.MaxValue;
    var positions = new (int Index, int Length, Direction? Direction, int TotalHeatLoss, int[] Path)[] { (startIndex, 0, null, 0, new int[] { startIndex }) };
    var nextPositions = new List<(int Index, int Length, Direction? Direction, int TotalHeatLoss, int[] Path)>();
    var history = new int[this.Index.Length][];
    for (var i=0; i<history.Length; i++) {
      history[i] = new int[4 * (maxLength + 1)];
      for (var j=0; j<(4 * (maxLength + 1)); j++) history[i][j] = int.MaxValue;
    }

    // Loop until all positions exhausted
    while (positions.Length > 0) {
      // Clear next positions
      nextPositions.Clear();

      // Find next locations based on currently scheduled positions
      for (var i=0; i<positions.Length; i++) {
        var position = positions[i];
        var coords = this.Index.IndexToCoordinates(position.Index);

        // Prevent looping
        this.storePosition(position, maxLength, history);

        // Check if reached end index
        if (position.Index == endIndex && position.Length >= minLength) {
          if (position.TotalHeatLoss < minTotalHeathLoss) {
            minTotalHeathLoss = position.TotalHeatLoss;

            // Output the minimum
            log.WriteLine($"""- New minimum path found: {position.TotalHeatLoss}""", level);
            // Output new minimum path
            var buffer = new ConsoleBuffer(this.Index.Dimensions);
            for (var j=0; j<this.Index.Length; j++) {
              if (position.Path.Contains(j)) {
                buffer.WriteToBuffer('.', j);
              } else {
                buffer.WriteToBuffer(this.HeatLossPerTile[j].ToString()[0], j);
              }
            }
            buffer.WriteToLog(log, ConsoleLoggingLevel.All);
            log.WriteLine(ConsoleLoggingLevel.All);
          }
        }

        // Check if closest to end
        var distance = Math.Abs(coords[0] - end[0]) + Math.Abs(coords[1] - end[1]);
        if (distance < closestToEnd) {
          closestToEnd = distance;
          log.WriteLine($"""- Closest location to end point: {coords[0]}x{coords[1]}""", level);
        }

        // Find next positions based on last position, direction and length
        {
          // Try moving up
          if (position.Direction != Direction.Bottom && (position.Direction == null || (position.Direction != Direction.Top ? position.Length >= minLength : position.Length < maxLength))) {
            if (coords[1] - 1 >= 0) {
              var index = (int)this.Index.CoordinatesToIndex(new long[] { coords[0], coords[1] - 1 });
              var nextLength = position.Direction == Direction.Top ? position.Length + 1 : 1;
              var heatLoss = position.TotalHeatLoss + this.HeatLossPerTile[index];
              if (this.allowPosition(position, index, minTotalHeathLoss, maxLength, history)) {
                var next = (
                  index,
                  nextLength,
                  Direction.Top,
                  heatLoss,
                  log.CheckLogLevel(ConsoleLoggingLevel.All) ? position.Path.Concat(new int[] { index }).ToArray() : position.Path
                );
                nextPositions.Add(next);
              }
            }
          }
          // Try moving down
          if (position.Direction != Direction.Top && (position.Direction == null || (position.Direction != Direction.Bottom ? position.Length >= minLength : position.Length < maxLength))) {
            if (coords[1] + 1 < this.Index.Dimensions[1]) {
              var index = (int)this.Index.CoordinatesToIndex(new long[] { coords[0], coords[1] + 1 });
              var nextLength = position.Direction == Direction.Bottom ? position.Length + 1 : 1;
              var heatLoss = position.TotalHeatLoss + this.HeatLossPerTile[index];
              if (this.allowPosition(position, index, minTotalHeathLoss, maxLength, history)) {
                var next = (
                  index,
                  nextLength,
                  Direction.Bottom,
                  heatLoss,
                  log.CheckLogLevel(ConsoleLoggingLevel.All) ? position.Path.Concat(new int[] { index }).ToArray() : position.Path
                );
                nextPositions.Add(next);
              }
            }
          }
          // Try moving left
          if (position.Direction != Direction.Right && (position.Direction == null || (position.Direction != Direction.Left ? position.Length >= minLength : position.Length < maxLength))) {
            if (coords[0] - 1 >= 0) {
              var index = (int)this.Index.CoordinatesToIndex(new long[] { coords[0] - 1, coords[1] });
              var nextLength = position.Direction == Direction.Left ? position.Length + 1 : 1;
              var heatLoss = position.TotalHeatLoss + this.HeatLossPerTile[index];
              if (this.allowPosition(position, index, minTotalHeathLoss, maxLength, history)) {
                var next = (
                  index,
                  nextLength,
                  Direction.Left,
                  heatLoss,
                  log.CheckLogLevel(ConsoleLoggingLevel.All) ? position.Path.Concat(new int[] { index }).ToArray() : position.Path
                );
                nextPositions.Add(next);
              }
            }
          }
          // Try moving right
          if (position.Direction != Direction.Left && (position.Direction == null || (position.Direction != Direction.Right ? position.Length >= minLength : position.Length < maxLength))) {
            if (coords[0] + 1 < this.Index.Dimensions[0]) {
              var index = (int)this.Index.CoordinatesToIndex(new long[] { coords[0] + 1, coords[1] });
              var nextLength = position.Direction == Direction.Right ? position.Length + 1 : 1;
              var heatLoss = position.TotalHeatLoss + this.HeatLossPerTile[index];
              if (this.allowPosition(position, index, minTotalHeathLoss, maxLength, history)) {
                var next = (
                  index,
                  nextLength,
                  Direction.Right,
                  heatLoss,
                  log.CheckLogLevel(ConsoleLoggingLevel.All) ? position.Path.Concat(new int[] { index }).ToArray() : position.Path
                );
                nextPositions.Add(next);
              }
            }
          }
        }
      }

      // Process next positions
      var dict = new Dictionary<string, (int Index, int Length, Direction? Direction, int TotalHeatLoss, int[] Path)>();
      foreach(var next in nextPositions) {
        var key = $"""{next.Index}:{next.Direction}:{next.Length}""";
        if (!dict.ContainsKey(key) || dict[key].TotalHeatLoss > next.TotalHeatLoss) {
          dict[key] = next;
        }
      }
      positions = dict.Values.ToArray();
    }

    // Return min heath loss
    return minTotalHeathLoss;
  }

  public void storePosition ((int Index, int Length, Direction? Direction, int TotalHeatLoss, int[] Path)position, int maxLength, int[][] history) {
        // Prevent looping
      if (position.Direction != null) {
        // Get a unique direction and length index
        var directionAndLengthIndex = 0;
        switch (position.Direction) {
          case Direction.Top:    directionAndLengthIndex = 0 * maxLength; break;
          case Direction.Bottom: directionAndLengthIndex = 1 * maxLength; break;
          case Direction.Left:   directionAndLengthIndex = 2 * maxLength; break;
          case Direction.Right:  directionAndLengthIndex = 3 * maxLength; break;
        }
        // Store length if lower
        if (history[position.Index][directionAndLengthIndex + position.Length] > position.TotalHeatLoss) {
          history[position.Index][directionAndLengthIndex + position.Length] = position.TotalHeatLoss;
        }
      }
  }

  public bool allowPosition ((int Index, int Length, Direction? Direction, int TotalHeatLoss, int[] Path)position, int index, int minTotalHeathLoss, int maxLength, int[][] history) {
      // Prevent looping
      if (position.Direction != null) {
        // If over already found minimum, skip
        if (position.TotalHeatLoss > minTotalHeathLoss) return false;
        // Check if already in path
        if (position.Path.Contains(index)) return false;
        // Get a unique direction and length index
        var directionAndLengthIndex = 0;
        switch (position.Direction) {
          case Direction.Top:    directionAndLengthIndex = 0 * maxLength; break;
          case Direction.Bottom: directionAndLengthIndex = 1 * maxLength; break;
          case Direction.Left:   directionAndLengthIndex = 2 * maxLength; break;
          case Direction.Right:  directionAndLengthIndex = 3 * maxLength; break;
        }
        // If index already was reached from same direction with same length with smaller total, skip
        if (history[position.Index][directionAndLengthIndex + position.Length] < position.TotalHeatLoss) {
          return false;
        }
      }
      // Default to allow
      return true;
  }

}
