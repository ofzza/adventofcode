namespace ofzza.aoc.year2023.utils.cosmicexpansion;

using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

public class CosmicExpansion {

  /// <summary>
  /// Holds coordinates of galaxy tiles
  /// </summary>
  public required long[][] Galaxies { init; get; }
 
  /// <summary>
  /// Matrix indexer initialized to map cosmic expansion to tiles
  /// </summary>
  private MatrixIndexer Index { init; get; }
  
  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="dimensions">Dimensions of the space</param>
  public CosmicExpansion (long[] dimensions) {
    this.Index = new MatrixIndexer(dimensions);
  }
  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="index">Matrix index, indexing the space</param>
  public CosmicExpansion (MatrixIndexer index) {
    this.Index = index;
  }

  /// <summary>
  /// Expands the space on every empty row and column
  /// </summary>
  /// <param name="emptyRowExpansion">How many extra rows each empty row should expand into</param>
  /// <param name="emptyColumnExpansion">How many extra rows each empty column should expand into</param>
  /// <returns>Expanded space</returns>
  public CosmicExpansion ExpandEmptyRowsAndColumns (long emptyRowExpansion = 1, long emptyColumnExpansion = 1) {
    // Detect empty rows and columns
    var emptyRows = new bool[this.Index.Dimensions[1]];
    for (var y=0; y<this.Index.Dimensions[1]; y++) emptyRows[y] = true;
    var emptyCols = new bool[this.Index.Dimensions[0]];
    for (var x=0; x<this.Index.Dimensions[0]; x++) emptyCols[x] = true;
    foreach (var coords in this.Galaxies) {
      emptyRows[coords[1]] = false;
      emptyCols[coords[0]] = false;
    }
    // Calculate offsets based on empty rows and columns
    var rowsOffset = new long[this.Index.Dimensions[1]];
    long rowsOffsetSum = 0;
    for (var i=0; i<emptyRows.Length; i++) {
      if (emptyRows[i] == true) { rowsOffsetSum += emptyRowExpansion - 1;  }
      rowsOffset[i] = rowsOffsetSum;
    }
    var columnsOffset = new long[this.Index.Dimensions[0]];
    long columnsOffsetSum = 0;
    for (var i=0; i<emptyCols.Length; i++) {
      if (emptyCols[i] == true) { columnsOffsetSum += emptyColumnExpansion - 1; }
      columnsOffset[i] = columnsOffsetSum;
    }
    // Clone current cosmos
    var expanded = new CosmicExpansion(new long[] { this.Index.Dimensions[0] + columnsOffsetSum, this.Index.Dimensions[1] + rowsOffsetSum }) {
      Galaxies = (long[][])this.Galaxies.Clone()
    };
    // Adjust expanded coordinates to take into account empty rows and columns
    for (var i=0; i<expanded.Galaxies.Length; i++) {
      expanded.Galaxies[i][0] += columnsOffset[expanded.Galaxies[i][0]];
      expanded.Galaxies[i][1] += rowsOffset[expanded.Galaxies[i][1]];
    }
    // Return cloned, expanded cosmos
    return expanded;
  }

  /// <summary>
  /// Outputs cosmic expansion visual representation to Console
  /// </summary>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  public void Log (Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    // Check log level
    if (!log.CheckLogLevel(level)) return;    
    
    // Ready console buffer
    var buffer = new ConsoleBuffer(this.Index);
    buffer.FillBuffer('.');
    // Write galaxies to buffer
    foreach (var coords in this.Galaxies) buffer.WriteToBuffer('#', coords);
    // Flush buffer to log
    buffer.WriteToLog(log, level);
  }

}
