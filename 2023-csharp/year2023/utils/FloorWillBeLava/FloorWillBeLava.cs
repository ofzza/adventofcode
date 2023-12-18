namespace ofzza.aoc.year2023.utils.floorwillbelava;

using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

public class FloorWillBeLava {

  /// <summary>
  ///  Holds map input
  /// </summary>
  private char[] Input { init; get; }
  private MatrixIndexer Index { init; get; }

  /// <summary>
  /// Holds locations of all items on the map and connections to other items in each direction
  /// </summary>
  private (char Item, int[] Coords, (int? Top, int? Bottom, int? Left, int? Right) Connections)[] Connections { init; get; }

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="input">Map of mirrors and splitters</param>
  public FloorWillBeLava (char[][] input) {
    // Store input
    this.Input = string.Join("", input.Select(l => string.Join("", l))).ToArray();
    this.Index = new MatrixIndexer(new long[] { input[0].Length, input.Length });
    
    // Initialize connections
    this.Connections = new (char Item, int[] Coords, (int? Top, int? Bottom, int? Left, int? Right) Connections)[this.Index.Length];
    for (var i=0; i<this.Index.Length; i++) this.Connections[i] = (this.Input[i], this.Index.IndexToCoordinates(i).Select(c => (int)c).ToArray(), (null, null, null, null));
    // Store vertical connections
    var row = new int?[this.Index.Dimensions[0]];
    for (var i=0; i<this.Index.Dimensions[0]; i++) row[i] = null;
    for (var y=0; y<this.Index.Dimensions[1]; y++) {
      for (var x=0; x<this.Index.Dimensions[0]; x++) {
        var i = this.Index.CoordinatesToIndex(new long[] { x, y });
        if (this.Input[i] != '.') {
          // Get item on location
          var item = this.Input[i];
          var index = (int)i;
          var coords = new int[] { x, y };
          // Connect to previous item in same column
          if (row[x] != null) {
            var top = this.Connections[(int)row[x]!];
            this.Connections[(int)row[x]!] = (top.Item, top.Coords, (top.Connections.Top, index, top.Connections.Left, top.Connections.Right));
            var current = this.Connections[i];
            this.Connections[i] = (current.Item, current.Coords, ((int)row[x]!, current.Connections.Bottom, current.Connections.Left, current.Connections.Right));
          }
          // Connect first item in row, to first cell (if first cell empty)
          else {
            var fi = this.Index.CoordinatesToIndex(new long[] { x, 0 });
            var first = this.Connections[fi];
            this.Connections[fi] = (first.Item, first.Coords, (first.Connections.Top, index, first.Connections.Left, first.Connections.Right));
          }
          // Set as latest item
          row[x] = index;
        }
      }
    }
    // Connect back from last row
    for (var x=0; x<this.Index.Dimensions[0]; x++) {
      var li = this.Index.CoordinatesToIndex(new long[] { x, this.Index.Dimensions[1] - 1 });
      var last = this.Connections[li];
      if (this.Input[li] == '.') this.Connections[li] = (last.Item, last.Coords, (row[x], last.Connections.Bottom, last.Connections.Left, last.Connections.Right));
    }
    // Store horizontal connections
    var column = new int?[this.Index.Dimensions[1]];
    for (var i=0; i<this.Index.Dimensions[1]; i++) column[i] = null;
    for (var x=0; x<this.Index.Dimensions[0]; x++) {
      for (var y=0; y<this.Index.Dimensions[1]; y++) {
        var i = this.Index.CoordinatesToIndex(new long[] { x, y });
        if (this.Input[i] != '.') {
          // Get item on location
          var item = this.Input[i];
          var index = (int)i;
          var coords = new int[] { x, y };
          // Connect to previous item in same column
          if (column[y] != null) {
            var top = this.Connections[(int)column[y]!];
            this.Connections[(int)column[y]!] = (top.Item, top.Coords, (top.Connections.Top, top.Connections.Bottom, top.Connections.Left, index));
            var current = this.Connections[i];
            this.Connections[i] = (current.Item, current.Coords, (current.Connections.Top, current.Connections.Bottom, (int)column[y]!, current.Connections.Right));
          }
          // Connect first item in column, to first cell (if first cell empty)
          else {
            var fi = this.Index.CoordinatesToIndex(new long[] { 0, y });
            var first = this.Connections[fi];
            this.Connections[fi] = (first.Item, first.Coords, (first.Connections.Top, first.Connections.Bottom, first.Connections.Left, index));
          }
          // Set as latest item
          column[y] = index;
        }
      }
    }
    // Connect back from last column
    for (var y=0; y<this.Index.Dimensions[1]; y++) {
      var li = this.Index.CoordinatesToIndex(new long[] { this.Index.Dimensions[0] - 1, y });
      var last = this.Connections[li];
      if (this.Input[li] == '.') this.Connections[li] = (last.Item, last.Coords, (last.Connections.Top, last.Connections.Bottom, column[y], last.Connections.Right));
    }
  }

  /// <summary>
  /// Shoot a beam of light and track its interactions
  /// </summary>
  /// <param name="index">Starting point index</param>
  /// <param name="direction">Starting direction</param>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  /// <returns>Number of energized points</returns>
  public int CalculateEnergizationFromAStartingPoint (int index, Direction direction, Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    // Initialize and track energized items
    var count = 0;
    var energized = new bool[this.Index.Length];
    for (var i=0; i<this.Index.Length; i++) energized[i] = false;

    // Initialize beams
    var beams = new List<(int Previous, int Current, Direction Direction)>() {(index, index, direction)};

    // Initialize beam history
    var history = new List<(int Location, Direction Direction)>() {};
    // Cast beams
    for (var i=0; i<beams.Count; i++) {
      // Log
      log.WriteLine(level);
      log.WriteLine($"""Beam {i}:""", level);

      // Find next item for the beam to hit
      var target = ((int Location, Direction Direction))(beams[i].Current, beams[i].Direction);
      var last = beams[i].Previous;
      // Cast beam until it loops
      do {
        // Log
        var targetCoords = this.Index.IndexToCoordinates(target.Location);
        log.WriteLine($"""- {targetCoords[0]}x{targetCoords[1]} {target.Direction.ToString()}: {this.Input[target.Location]}""", level);

        if (history.Any(p => target.Location == p.Location && target.Direction == p.Direction)) {
          log.WriteLine($"""- Looping detected!""", level);
          break;
        }

        // Energize the space between last an current item
        var lastCoords = this.Index.IndexToCoordinates((long)last!);
        count += this.Energize(energized, last, target.Location);
        // Log energized
        log.WriteLine($"""- Energized line between: {lastCoords[0]}x{lastCoords[1]} - {targetCoords[0]}x{targetCoords[1]} = {count}""", level);

        // Store current location/direction to history
        history.Add((target.Location, target.Direction));
        last = target.Location;
        // Interact with next item
        var item = this.Connections[target.Location];
        // Interact with a splitter
        if (item.Item == '|') {
          // Pass through
          if (target.Direction == Direction.Top) {
            if (item.Connections.Top == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Top));
              break;
            } else {
              target = ((int)item.Connections.Top!, Direction.Top);
            }
          }
          else if (target.Direction == Direction.Bottom) {
            if (item.Connections.Bottom == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Bottom));
              break;
            } else {
              target = ((int)item.Connections.Bottom!, Direction.Bottom);
            }
          }
          // Bifurcate
          else if (target.Direction == Direction.Left || target.Direction == Direction.Right) {
            // Split a secondary beam to the bottom
            if (item.Connections.Bottom != null) {
              beams.Add((target.Location, (int)item.Connections.Bottom!, Direction.Bottom));
              log.WriteLine($"""- Split off into a new beam""", level);
            } else {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Bottom));
            }
            // Reflect to top
            if (item.Connections.Top == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Top));
              break;
            } else {
              target = ((int)item.Connections.Top!, Direction.Top);
            }
          }
        }
        // Interact with a splitter
        else if (item.Item == '-') {
          // Bifurcate
          if (target.Direction == Direction.Top || target.Direction == Direction.Bottom) {
            // Split a secondary beam to the right
            if (item.Connections.Right != null) {
              beams.Add((target.Location, (int)item.Connections.Right!, Direction.Right));
              log.WriteLine($"""- Split off into a new beam""", level);
            } else {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Right));
            }
            // Reflect to left
            if (item.Connections.Left == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Left));
              break;
            } else {
              target = ((int)item.Connections.Left!, Direction.Left);
            }
          }
          // Pass through
          else if (target.Direction == Direction.Left) {
            if (item.Connections.Left == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Left));
              break;
            } else {
              target = ((int)item.Connections.Left!, Direction.Left);
            }
          }
          else if (target.Direction == Direction.Right) {
            if (item.Connections.Right == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Right));
              break;
            } else {
              target = ((int)item.Connections.Right!, Direction.Right);
            }
          }
        }
        // Interact with a mirror
        else if (item.Item == '/') {
          // Reflect
          if (target.Direction == Direction.Top) {
            if (item.Connections.Right == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Right));
              break;
            } else {
              target = ((int)item.Connections.Right!, Direction.Right);
            }
          }
          else if (target.Direction == Direction.Bottom) {
            if (item.Connections.Left == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Left));
              break;
            } else {
              target = ((int)item.Connections.Left!, Direction.Left);
            }
          }
          else if (target.Direction == Direction.Left) {
            if (item.Connections.Bottom == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Bottom));
              break;
            } else {
              target = ((int)item.Connections.Bottom!, Direction.Bottom);
            }
          }
          else if (target.Direction == Direction.Right) {
            if (item.Connections.Top == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Top));
              break;
            } else {
              target = ((int)item.Connections.Top!, Direction.Top);
            }
          }
        }
        // Interact with a mirror
        else if (item.Item == '\\') {
          // Reflect
          if (target.Direction == Direction.Top) {
            if (item.Connections.Left == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Left));
              break;
            } else {
              target = ((int)item.Connections.Left!, Direction.Left);
            }
          }
          else if (target.Direction == Direction.Bottom) {
            if (item.Connections.Right == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Right));
              break;
            } else {
              target = ((int)item.Connections.Right!, Direction.Right);
            }
          }
          else if (target.Direction == Direction.Left) {
            if (item.Connections.Top == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Top));
              break;
            } else {
              target = ((int)item.Connections.Top!, Direction.Top);
            }
          }
          else if (target.Direction == Direction.Right) {
            if (item.Connections.Bottom == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Bottom));
              break;
            } else {
              target = ((int)item.Connections.Bottom!, Direction.Bottom);
            }
          }
        }
        // Interact with empty space
        else {
          // Continue
          if (target.Direction == Direction.Top) {
            if (item.Connections.Top == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Top));
              break;
            } else {
              target = ((int)item.Connections.Top!, Direction.Top);
            }
          }
          if (target.Direction == Direction.Bottom) {
            if (item.Connections.Bottom == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Bottom));
              break;
            } else {
              target = ((int)item.Connections.Bottom!, Direction.Bottom);
            }
          }
          if (target.Direction == Direction.Left) {
            if (item.Connections.Left == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Left));
              break;
            } else {
              target = ((int)item.Connections.Left!, Direction.Left);
            }
          }
          if (target.Direction == Direction.Right) {
            if (item.Connections.Right == null) {
              count += this.Energize(energized, target.Location, this.FindEdge(target.Location, Direction.Right));
              break;
            } else {
              target = ((int)item.Connections.Right!, Direction.Right);
            }
          }
        }
      
      } while (true);
      
      // Log beam outside the are or looping
      if (history.Any(p => target.Location == p.Location && target.Direction == p.Direction)) {
        log.WriteLine($"""- Done!""", level);
      }

    }

    // Output energized area
    log.WriteLine(level);
    var buffer = new ConsoleBuffer(this.Index.Dimensions);
    buffer.FillBuffer('.');
    for (var j=0; j<this.Index.Length; j++) if (  this.Input[j] != '.') buffer.WriteToBuffer(this.Input[j], j); else if (energized[j]) buffer.WriteToBuffer('#', j);
    buffer.WriteToLog(log, level);

    // Return count of energized elements
    return count;
  }

  /// <summary>
  /// Shoot a beam of light from every possible entry point, track its interactions and find the maximum
  /// </summary>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  /// <returns>Maximum number of energized points</returns>
  public int FindOptimalEnergization (Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    // Reset maximum
    var max = 0;
    // Search for initial positions with vertical direction
    for (var x=0; x<this.Index.Dimensions[0]; x++) {
      // Search starting from top
      var top = this.CalculateEnergizationFromAStartingPoint((int)this.Index.CoordinatesToIndex(new long[] { x, 0 }), Direction.Bottom, log, level);
      if (top > max) max = top;
      // Search starting from bottom
      // var bottom = this.CalculateEnergizationFromAStartingPoint((int)this.Index.CoordinatesToIndex(new long[] { x, this.Index.Dimensions[1] - 1 }), Direction.Top, log, level);
      // if (bottom > max) max = bottom;
    }
    // Search for initial positions with horizontal direction
    for (var y=0; y<this.Index.Dimensions[1]; y++) {
      // Search from left
      // var left = this.CalculateEnergizationFromAStartingPoint((int)this.Index.CoordinatesToIndex(new long[] { 0, y }), Direction.Right, log, level);
      // if (left > max) max = left;
      // Search from right
      // var right = this.CalculateEnergizationFromAStartingPoint((int)this.Index.CoordinatesToIndex(new long[] { this.Index.Dimensions[0] - 1, y }), Direction.Left, log, level);
      // if (right > max) max = right;
    }
    // Return maximum
    return max;
  }

  private int FindEdge (int location, Direction direction) {
    var coords = this.Index.IndexToCoordinates(location);
    switch (direction) {
      case Direction.Top: return (int)this.Index.CoordinatesToIndex(new long[] { (long)coords[0], 0 });
      case Direction.Bottom: return (int)this.Index.CoordinatesToIndex(new long[] { (long)coords[0], this.Index.Dimensions[1] - 1 });
      case Direction.Left: return (int)this.Index.CoordinatesToIndex(new long[] { 0, (long)coords[1] });
      case Direction.Right: return (int)this.Index.CoordinatesToIndex(new long[] { (long)this.Index.Dimensions[0] - 1, (long)coords[1] });
    }
    throw new Exception("This should never happen!");
  }

  private int Energize (bool[] energized, int start, int end) {
    // Initialize
    var count = 0;
    var startCoords = this.Index.IndexToCoordinates((long)start!);
    var endCoords = this.Index.IndexToCoordinates((long)end!);
    // Energize
    for (
      var y=startCoords[1] <= endCoords[1] ? startCoords[1] : endCoords[1];
      y<=(startCoords[1] >= endCoords[1] ? startCoords[1] : endCoords[1]);
      y++)
    {
      for (
        var x=startCoords[0] <= endCoords[0] ? startCoords[0] : endCoords[0];
        x<=(startCoords[0] >= endCoords[0] ? startCoords[0] : endCoords[0]);
        x++)
      {
        var energizedIndex = this.Index.CoordinatesToIndex(new long[] { x, y });
        if (!energized[energizedIndex]) {
          energized[energizedIndex] = true;
          count++;
        }
      }
    }
    // Return count of energized coords
    return count;
  }

}
