namespace ofzza.aoc.year2023.utils.parabolicreflectordish;

using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

public class ParabolicReflectorDish {

  /// <summary>
  /// Indexes into the provided field
  /// </summary>
  private MatrixIndexer Index { init; get; }

  /// <summary>
  /// Holds all the field tiles
  /// </summary>
  private char[] Tiles { get; set; }

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="input">Field data</param>
  public ParabolicReflectorDish (char[][] input) {
    // Store field and create index
    this.Tiles = string.Join("", input.Select(l => string.Join("", l))).ToCharArray();;
    this.Index = new MatrixIndexer(new long[] { input[0].Length, input.Length });
  }

  public void Tilt (Direction direction) {
    // Initialize tilted state
    var tilted = (char[])this.Tiles.Clone();
    // Initialize space calculation    
    var offsets = this.Tiles.Select(c => 0).ToArray();

    // Calculate for each boulders offset to boundary for vertical movement
    if (direction == Direction.Top || direction == Direction.Bottom) {
      for (var x=0; x<this.Index.Dimensions[0]; x++) {
        var offset = 0;
        for (
          var y = direction == Direction.Top ? 0 : this.Index.Dimensions[1] - 1;
          direction == Direction.Top ? y<this.Index.Dimensions[1] : y>= 0;
          y = direction == Direction.Top ? y + 1 : y - 1
        ) {
          var i = this.Index.CoordinatesToIndex(new long[] { x, y });
          switch (this.Tiles[i]) {
            case '.': offsets[i] = 0; offset++; break;
            case '#': offsets[i] = 0; offset = 0; break;
            case 'O': offsets[i] = offset; break;
          }        
        }
      }
    }
    // Calculate for each boulders offset to boundary for horizontal movement
    else {
      for (var y=0; y<this.Index.Dimensions[1]; y++) {
        var offset = 0;
        for (
          var x = direction == Direction.Left ? 0 : this.Index.Dimensions[0] - 1;
          direction == Direction.Left ? x<this.Index.Dimensions[0] : x>= 0;
          x = direction == Direction.Left ? x + 1 : x - 1
        ) {
          var i = this.Index.CoordinatesToIndex(new long[] { x, y });
          switch (this.Tiles[i]) {
            case '.': offsets[i] = 0; offset++; break;
            case '#': offsets[i] = 0; offset = 0; break;
            case 'O': offsets[i] = offset; break;
          }        
        }
      }
    }

    // Move boulders in vertical direction
    if (direction == Direction.Top || direction == Direction.Bottom) {
      for (
          var y = direction == Direction.Top ? 0 : this.Index.Dimensions[1] - 1;
          direction == Direction.Top ? y<this.Index.Dimensions[1] : y>= 0;
          y = direction == Direction.Top ? y + 1 : y - 1
        ) {
        for (var x=0; x<this.Index.Dimensions[0]; x++) {
          var i = this.Index.CoordinatesToIndex(new long[] { x, y });        
          if (offsets[i] > 0) {
            tilted[this.Index.CoordinatesToIndex(new long[] { x, direction == Direction.Top ? y - offsets[i] : y + offsets[i]})] = this.Tiles[i];
            tilted[i] = '.';
          }
        }
      }
    }
    // Move boulders in horizontal direction
    else {
      for (
        var x = direction == Direction.Left ? 0 : this.Index.Dimensions[0] - 1;
        direction == Direction.Left ? x<this.Index.Dimensions[0] : x>= 0;
        x = direction == Direction.Left ? x + 1 : x - 1
      ) {
        for (var y=0; y<this.Index.Dimensions[1]; y++) {
          var i = this.Index.CoordinatesToIndex(new long[] { x, y });        
          if (offsets[i] > 0) {
            tilted[this.Index.CoordinatesToIndex(new long[] { direction == Direction.Left ? x - offsets[i] : x + offsets[i], y })] = this.Tiles[i];
            tilted[i] = '.';
          }
        }
      }
    }

    // Store next state
    this.Tiles = tilted;
  }

  public void CycleDish (long count, Console? log = null, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    var detected = false;
    var previous = new string[count];
    for (var i=0; i<count; i++) {
      // Log
      if (log != null) {
        log.WriteLine($"""> Cycle {i + 1} / {count}""", level);
      }
      // Store state before cycle
      previous[i] = string.Join("", this.Tiles);
      // Cycle the dish
      this.Tilt(Direction.Top);
      this.Tilt(Direction.Left);
      this.Tilt(Direction.Bottom);
      this.Tilt(Direction.Right);
      // Log
      if (log != null) this.Log(log, level);
      // Compare state and fast-forward if possible
      if (!detected) {
        var after = string.Join("", this.Tiles);
        for (var j=i; j>0; j--) {
          if (after == previous[j]) {
            detected = true;
            var cycleStartIndex = j;
            var cycleEndIndex = i + 1;
            var cycleLength = cycleEndIndex - cycleStartIndex;
            i += (int)Math.Floor((double)(count - cycleEndIndex) / (double)cycleLength) * cycleLength;
            if (log != null) log.WriteLine($"""> Found a cycle of length {cycleLength} from {cycleStartIndex} to {cycleEndIndex} > Jumping to {i}!""");
          }
        }
      }
    }
  }

  public long CalculateWeight () {
    long sum = 0;
    for (var y=0; y<this.Index.Dimensions[1]; y++) {
      for (var x=0; x<this.Index.Dimensions[0]; x++) {
        var i = this.Index.CoordinatesToIndex(new long[] { x, y });        
        if (this.Tiles[i] == 'O') sum += this.Index.Dimensions[1] - y;
      }
    }
    return sum;
  }

  /// <summary>
  /// Outputs dish visual representation to Console
  /// </summary>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  public void Log (Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    // Check log level
    if (!log.CheckLogLevel(level)) return;

    // Ready console buffer
    var buffer = new ConsoleBuffer(this.Index);

    // Draw to console buffer
    buffer.FillBuffer('.');

    // Draw to buffer
    for (var y=0; y<this.Index.Dimensions[1]; y++) {
      for (var x=0; x<this.Index.Dimensions[0]; x++) {
        var i = this.Index.CoordinatesToIndex(new long[] { x, y });
        buffer.WriteToBuffer(this.Tiles[i], new long[] { x, y });
      }
    }

    // Flush buffer to log
    buffer.WriteToLog(log, level);
  }
  
}
