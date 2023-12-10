namespace ofzza.aoc.year2023.utils.pipemaze;

using System.Collections;
using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

/// <summary>
/// Implements functionality required for navigating a 2D pipe maze
/// </summary>
public class PipeMaze {

  /// <summary>
  /// Holds all tiles making up the maze
  /// </summary>
  public DiagonalDirection[] Tiles { init; get; }
  /// <summary>
  /// Matrix indexer initialized to map maze coordinates to tiles
  /// </summary>
  public MatrixIndexer Index { init; get; }
  /// <summary>
  /// Holds the starting tile index
  /// </summary>
  private int StartIndex { init; get; }

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="input">Maze tiles as characters</param>
  public PipeMaze (char[][] input) {
    // Generate a matrix navigation index
    this.Index = new MatrixIndexer(new int[] { input.Length, input[0].Length });
    // Initialize tiles
    this.Tiles = new DiagonalDirection[input.Length * input[0].Length];
    // Decode and store tiles
    var i=0;
    for (var y=0; y<input.Length; y++) {
      for (var x=0; x<input[0].Length; x++) {
        // Store tiles and look for start
        switch (input[y][x]) {
          case '.': this.Tiles[i++] = DiagonalDirection.None; break;
          case '|': this.Tiles[i++] = DiagonalDirection.TopBottom; break;
          case '-': this.Tiles[i++] = DiagonalDirection.LeftRight; break;
          case 'J': this.Tiles[i++] = DiagonalDirection.TopLeft; break;
          case 'L': this.Tiles[i++] = DiagonalDirection.TopRight; break;
          case '7': this.Tiles[i++] = DiagonalDirection.BottomLeft; break;
          case 'F': this.Tiles[i++] = DiagonalDirection.BottomRight; break;
          case 'S': this.StartIndex = i; this.Tiles[i++] = DiagonalDirection.None; break;
        }
      }
    }
    // Find starting tile pipe type
    var startCoords = this.Index.IndexToCoordinates(this.StartIndex);
    var topCoords = new int[] { startCoords[0] - 1, startCoords[1] };
    var topConnected = this.Index.CheckIfValidCoordinates(topCoords) && ((int)this.Tiles[this.Index.CoordinatesToIndex(topCoords)] & 0b0010) != 0;
    var bottomCoords = new int[] { startCoords[0] + 1, startCoords[1] };
    var bottomConnected = this.Index.CheckIfValidCoordinates(bottomCoords) && ((int)this.Tiles[this.Index.CoordinatesToIndex(bottomCoords)] & 0b1000) != 0;
    var leftCoords = new int[] { startCoords[0], startCoords[1] - 1 };
    var leftConnected = this.Index.CheckIfValidCoordinates(leftCoords) && ((int)this.Tiles[this.Index.CoordinatesToIndex(leftCoords)] & 0b0001) != 0;
    var rightCoords = new int[] { startCoords[0], startCoords[1] + 1 };
    var rightConnected = this.Index.CheckIfValidCoordinates(rightCoords) && ((int)this.Tiles[this.Index.CoordinatesToIndex(rightCoords)] & 0b0100) != 0;
    if (topConnected && bottomConnected) { this.Tiles[this.StartIndex] = DiagonalDirection.TopBottom; }
    else if (leftConnected && rightConnected) { this.Tiles[this.StartIndex] = DiagonalDirection.LeftRight; }
    else if (topConnected && leftConnected) { this.Tiles[this.StartIndex] = DiagonalDirection.TopLeft; }
    else if (topConnected && rightConnected) { this.Tiles[this.StartIndex] = DiagonalDirection.TopRight; }
    else if (bottomConnected && leftConnected) { this.Tiles[this.StartIndex] = DiagonalDirection.BottomLeft; }
    else if (bottomConnected && rightConnected) { this.Tiles[this.StartIndex] = DiagonalDirection.BottomRight; }
  }

  /// <summary>
  /// Searches for a closed loop path from the starting tile, back to the starting tile
  /// </summary>
  /// <returns>A closed loop path from the starting tile, back to the starting tile</returns>
  public (int[] Coordinates, Direction Direction)[] FindClosedLoopPath () {
    // Initialize starting index and starting direction
    var index = this.StartIndex;
    var coords = this.Index.IndexToCoordinates(index);
    var tile = this.Tiles[index];
    var direction =
      ((int)tile & (int)Direction.Top) != 0 ? Direction.Top :
      ((int)tile & (int)Direction.Bottom) != 0 ? Direction.Bottom :
      ((int)tile & (int)Direction.Left) != 0 ? Direction.Left :
      ((int)tile & (int)Direction.Right) != 0 ? Direction.Right :
      Direction.None;
    // Walk the path until back to start
    var path = new List<(int[] Coordinates, Direction Direction)>() {};
    do {
      // Store next time into path
      path.Add((coords, direction));
      // Find next tile
      coords =
        direction == Direction.Top ? new int[] { coords[0] - 1, coords[1] } : 
        direction == Direction.Bottom ? new int[] { coords[0] + 1, coords[1] } : 
        direction == Direction.Left ? new int[] { coords[0], coords[1] - 1 } : 
        direction == Direction.Right ? new int[] { coords[0], coords[1] + 1 } :
        new int[] { coords[0], coords[1] };
      index = this.Index.CoordinatesToIndex(coords);
      tile = this.Tiles[index];
      // Find next direction
      direction = (Direction)((int)tile ^ (int)this.GetOppositeDirection(direction));
    } while (index != this.StartIndex);
    // Return path
    return path.ToArray();
  }

  private Direction GetOppositeDirection (Direction direction) {
    switch (direction) {
      case Direction.Top: return Direction.Bottom;
      case Direction.Bottom: return Direction.Top;
      case Direction.Left: return Direction.Right;
      case Direction.Right: return Direction.Left;
    }
    throw new Exception("Provided direction can't be reversed!");
  }

  /// <summary>
  /// Finds coordinates of areas inside and outside the path
  /// </summary>
  /// <param name="path">Path within which the area is considered to be an inside area</param>
  /// <returns>Coordinates of inside and outside area tiles</returns>
  public (int[][] InsideCoordinates, int[][] OutsideCoordinates) FindAreas ((int[] Coordinates, Direction Direction)[] path) {
    // Initialize areas
    var inside = new List<int[]>();
    var outside = new List<int[]>();
    // Find coordinates in areas
    for (var y=0; y<this.Index.Dimensions[0]; y++) {
      var isInside = false;
      var pathIntersection = new List<int>();
      foreach (var c in path) if (c.Coordinates[0] == y) pathIntersection.Add(this.Index.CoordinatesToIndex(c.Coordinates));
      for (var x=0; x<this.Index.Dimensions[1]; x++) {
        // Check if coordinate on path
        var i = this.Index.CoordinatesToIndex(new int[] { y, x });
        var tile = this.Tiles[i];
        var coordinateOnPath = pathIntersection.Any(index => index == i);
        var top = ((int)tile & (int)Direction.Top) != 0;
        // If on path, toggle inside/outside if required
        if (coordinateOnPath && top) {
          isInside = !isInside;
        }
        // Store coordinates not on the path
        else if (!coordinateOnPath) {
          // Store as inside coordinates
          if (isInside) {
            inside.Add(new int[] { y, x });
          }
          // Store outside coordinates
          else {
            outside.Add(new int[] { y, x });
          }
        }
      }
    }
    // Return areas
    return (inside.ToArray(), outside.ToArray());
  }

  /// <summary>
  /// Outputs maze visual representation to Console
  /// </summary>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  /// <param name="path">If path is provided provided, only the path tiles will be drawn</param>
  /// <param name="inside">If area is provided, tiles within the area will be colored as inner</param>
  /// <param name="outside">If area is provided, tiles within the area will be colored as outer</param>
  public void Log (Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, (int[] Coordinates, Direction Direction)[]? path = null, int[][]? inside = null, int[][]? outside = null) {
    // Check log level
    if (!log.CheckLogLevel(level)) return;
    // Write to log
    for (var y=0; y<this.Index.Dimensions[0]; y++) {
      var line = "";
      var pathIntersection = new List<int>();
      if (path != null) foreach (var c in path) if (c.Coordinates[0] == y) pathIntersection.Add(this.Index.CoordinatesToIndex(c.Coordinates));
      for (var x=0; x<this.Index.Dimensions[1]; x++) {
        var i = this.Index.CoordinatesToIndex(new int[] { y, x });
        if (path == null || pathIntersection!.Any(index => index == i)) { //
          switch (this.Tiles[i]) {
            case DiagonalDirection.None:        line += i == this.StartIndex ? '█' : '·'; break;
            case DiagonalDirection.TopBottom:   line += i == this.StartIndex ? '║' : '│'; break;
            case DiagonalDirection.LeftRight:   line += i == this.StartIndex ? '═' : '─'; break;
            case DiagonalDirection.TopLeft:     line += i == this.StartIndex ? '╝' : '┘'; break;
            case DiagonalDirection.TopRight:    line += i == this.StartIndex ? '╚' : '└'; break;
            case DiagonalDirection.BottomLeft:  line += i == this.StartIndex ? '╗' : '┐'; break;
            case DiagonalDirection.BottomRight: line += i == this.StartIndex ? '╔' : '┌'; break;
          }
        } else if (inside != null && inside.Any(coord => this.Index.CoordinatesToIndex(coord) == i)) {
          line += 'i';
        } else if (outside != null && outside.Any(coord => this.Index.CoordinatesToIndex(coord) == i)) {
          line += 'o';
        } else { line += '.'; }
      }
      log.WriteLine(line, level);
    }
  }
}
