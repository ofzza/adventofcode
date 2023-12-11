namespace ofzza.aoc.year2023.utils.pipemaze;

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
  private long StartIndex { init; get; }

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="input">Maze tiles as characters</param>
  public PipeMaze (char[][] input) {
    // Generate a matrix navigation index
    this.Index = new MatrixIndexer(new long[] { input[0].Length, input.Length });
    // Initialize tiles
    this.Tiles = new DiagonalDirection[input[0].Length * input.Length];
    // Decode and store tiles
    for (var y=0; y<input.Length; y++) {
      for (var x=0; x<input[0].Length; x++) {
        // Store tiles and look for start
        var i = this.Index.CoordinatesToIndex(new long[] { x, y});
        switch (input[y][x]) {
          case '.': this.Tiles[i] = DiagonalDirection.None; break;
          case '|': this.Tiles[i] = DiagonalDirection.TopBottom; break;
          case '-': this.Tiles[i] = DiagonalDirection.LeftRight; break;
          case 'J': this.Tiles[i] = DiagonalDirection.TopLeft; break;
          case 'L': this.Tiles[i] = DiagonalDirection.TopRight; break;
          case '7': this.Tiles[i] = DiagonalDirection.BottomLeft; break;
          case 'F': this.Tiles[i] = DiagonalDirection.BottomRight; break;
          case 'S': this.Tiles[i] = DiagonalDirection.None; this.StartIndex = i; break;
        }
      }
    }
    // Find starting tile pipe type
    var startCoords = this.Index.IndexToCoordinates(this.StartIndex);
    var topCoords = new long[] { startCoords[0], startCoords[1] - 1 };
    var topConnected = this.Index.CheckIfValidCoordinates(topCoords) && ((int)this.Tiles[this.Index.CoordinatesToIndex(topCoords)] & 0b0010) != 0;
    var bottomCoords = new long[] { startCoords[0], startCoords[1] + 1 };
    var bottomConnected = this.Index.CheckIfValidCoordinates(bottomCoords) && ((int)this.Tiles[this.Index.CoordinatesToIndex(bottomCoords)] & 0b1000) != 0;
    var leftCoords = new long[] { startCoords[0] - 1, startCoords[1] };
    var leftConnected = this.Index.CheckIfValidCoordinates(leftCoords) && ((int)this.Tiles[this.Index.CoordinatesToIndex(leftCoords)] & 0b0001) != 0;
    var rightCoords = new long[] { startCoords[0] + 1, startCoords[1] };
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
  public (long[] Coordinates, Direction Direction)[] FindClosedLoopPath () {
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
    var path = new List<(long[] Coordinates, Direction Direction)>() {};
    do {
      // Store next time into path
      path.Add((coords, direction));
      // Find next tile
      coords =
        direction == Direction.Top ? new long[] { coords[0], coords[1] - 1 } : 
        direction == Direction.Bottom ? new long[] { coords[0], coords[1] + 1 } : 
        direction == Direction.Left ? new long[] { coords[0] - 1, coords[1] } : 
        direction == Direction.Right ? new long[] { coords[0] + 1, coords[1] } :
        new long[] { coords[0], coords[1] };
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
  public (long[][] InsideCoordinates, long[][] OutsideCoordinates) FindAreas ((long[] Coordinates, Direction Direction)[] path) {
    // Initialize areas
    var inside = new List<long[]>();
    var outside = new List<long[]>();
    // Find coordinates in areas
    for (var y=0; y<this.Index.Dimensions[1]; y++) {
      var isInside = false;
      var pathIntersectionWithRow = new List<long>();
      foreach (var c in path) if (c.Coordinates[1] == y) pathIntersectionWithRow.Add(this.Index.CoordinatesToIndex(c.Coordinates));
      for (var x=0; x<this.Index.Dimensions[0]; x++) {
        // Check if coordinate on path
        var i = this.Index.CoordinatesToIndex(new long[] { x, y });
        var tile = this.Tiles[i];
        var coordinateOnPath = pathIntersectionWithRow.Any(index => index == i);
        var top = ((int)tile & (int)Direction.Top) != 0;
        // If on path, toggle inside/outside if required
        if (coordinateOnPath && top) {
          isInside = !isInside;
        }
        // Store coordinates not on the path
        else if (!coordinateOnPath) {
          // Store as inside coordinates
          if (isInside) {
            inside.Add(new long[] { y, x });
          }
          // Store outside coordinates
          else {
            outside.Add(new long[] { y, x });
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
  public void Log (Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, (long[] Coordinates, Direction Direction)[]? path = null, long[][]? inside = null, long[][]? outside = null) {
    // Check log level
    if (!log.CheckLogLevel(level)) return;
    
    // Ready console buffer
    var buffer = new ConsoleBuffer(this.Index);
    // Fill buffer with all the individual tiles
    if (path == null) {
      for (var y=0; y<this.Index.Dimensions[1]; y++) {
        for (var x=0; x<this.Index.Dimensions[0]; x++) {
          var index = this.Index.CoordinatesToIndex(new long[] { x, y });
          buffer.WriteToBuffer(this.LogTile(index), index);
        }
      }
    }
    // Add path and areas to the buffer
    else {
      buffer.FillBuffer('.');
      foreach (var tile in path) {
        var index = this.Index.CoordinatesToIndex(tile.Coordinates);
        buffer.WriteToBuffer(this.LogTile(index), index);
      }
      if (inside != null) {
        foreach (var tile in inside) buffer.WriteToBuffer('i', this.Index.CoordinatesToIndex(new long[] { tile[1], tile[0] }));
      }
      if (outside != null) {
        foreach (var tile in outside) buffer.WriteToBuffer('o', this.Index.CoordinatesToIndex(new long[] { tile[1], tile[0] }));
      }
    }
    // Flush buffer to log
    buffer.WriteToLog(log, level);
  }

  /// <summary>
  /// Finds a character representation for a tile
  /// </summary>
  /// <param name="i">Index of a tile to represent</param>
  /// <returns>A character representation for a tile</returns>
  /// <exception cref="Exception"></exception>
  private char LogTile (long i) {
    switch (this.Tiles[i]) {
      case DiagonalDirection.None:        return i == this.StartIndex ? '█' : '·';
      case DiagonalDirection.TopBottom:   return i == this.StartIndex ? '║' : '│';
      case DiagonalDirection.LeftRight:   return i == this.StartIndex ? '═' : '─';
      case DiagonalDirection.TopLeft:     return i == this.StartIndex ? '╝' : '┘';
      case DiagonalDirection.TopRight:    return i == this.StartIndex ? '╚' : '└';
      case DiagonalDirection.BottomLeft:  return i == this.StartIndex ? '╗' : '┐';
      case DiagonalDirection.BottomRight: return i == this.StartIndex ? '╔' : '┌';
    }
    throw new Exception("Tile can't be represented! This should never happen!");
  }
}
