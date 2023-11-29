namespace ofzza.aoc.utils.matrix;

using System;
using System.Diagnostics;
using Microsoft.VisualBasic;

/// <summary>
/// Helps manipulating a N-dimensional matrix, by:
/// - Converting coordinates to index
/// - Converting index to coordinates
/// - Finding neighboring coordinates/indexes
/// </summary>
public class MatrixIndexer {
  
  /// <summary>
  /// Dimensions of a N-dimensional matrix
  /// </summary>
  public int[] Dimensions { init; get; }
  /// <summary>
  /// Gets total length of all cells in the matrix
  /// </summary>
  public int Length { get; private set; }

/// <summary>
/// Constructor
/// </summary>
/// <param name="dimensions">Dimensions of a N-dimensional matrix</param>
/// <exception cref="Exception">Dimensionality not suppoerted</exception>
  public MatrixIndexer (int[] dimensions) {
    // Verify and set dimensionality
    if (dimensions.Length < 1) throw new Exception("Must provide at least a single dimension!");
    this.Dimensions = dimensions;

    // Initialize index
    this.Initialize();
  }

  private int[] DimensionOffsets { get; set; } = new int[0];
  private int[][] RelativeNeighborCoordsWithDiagonals { get; set; } = new int[0][];
  private int[][] RelativeNeighborCoordsWithoutDiagonals { get; set; } = new int[0][];

  private void Initialize () {
    // Calculate length and offsets per dimension
    this.DimensionOffsets = new int[this.Dimensions.Length];
    this.DimensionOffsets[this.Dimensions.Length - 1] = 1;
    var length = this.Dimensions[0];
    var offset = 1;
    for (var i=1; i<this.Dimensions.Length; i++) {
      length *= this.Dimensions[i];
      offset *= this.Dimensions[this.Dimensions.Length - i];
      this.DimensionOffsets[this.Dimensions.Length - 1 - i] = offset;
    }
    this.Length = length;

    // Calculate relative neighboring coordinates
    var coords = new List<List<int>>() { new() {} };
    for (var i=0; i<this.Dimensions.Length; i++) {
      // Extend all coordinates with an extra dimension
      foreach (var coord in coords) coord.Add(0);
      // For each coordinate, add neighbord in recent dimension
      var next = new List<List<int>>();
      foreach (var coord in coords) {
        next.Add(coord);
        var c1p = new List<int>(coord);
        c1p[c1p.Count - 1] = 1;
        next.Add(c1p);
        var c1m = new List<int>(coord);
        c1m[c1m.Count - 1] = -1;
        next.Add(c1m);
      }
      coords = next;
    }
    this.RelativeNeighborCoordsWithDiagonals = coords.GetRange(1, coords.Count - 1).Select(c => c.ToArray()).ToArray();
    this.RelativeNeighborCoordsWithoutDiagonals = coords.GetRange(1, coords.Count - 1).Where(c => c.Where(x => x != 0).Count() == 1).Select(c => c.ToArray()).ToArray();
  }

  /// <summary>
  /// Converts a linear index into a N-dimensional coordinate
  /// </summary>
  /// <param name="index">Linear index</param>
  /// <returns>N-dimensional coordinate</returns>
  public int[] IndexToCoordinates (int index) {
    var coords = new int[this.Dimensions.Length];
    var remainder = index;
    for (var i=0; i<this.Dimensions.Length; i++) {
      var coord = remainder / this.DimensionOffsets[i];
      remainder = remainder % this.DimensionOffsets[i];
      coords[i] = coord;
    }
    return coords;
  }
  
  /// <summary>
  /// Converts a N-dimensional coordinate into a linear index
  /// </summary>
  /// <param name="coords">N-dimensional coordinate</param>
  /// <returns>Linear index</returns>
  public int CoordinatesToIndex (int[] coords) {
    int index = 0;
    for (var i=0; i<this.Dimensions.Length; i++) {
      index += this.DimensionOffsets[i] * coords[i];
    }
    return index;
  }

  /// <summary>
  /// Returns indices of all neighboring cells
  /// </summary>
  /// <param name="index">Index to find neighbors for</param>
  /// <param name="diagonal">If diagonally adjacent cells could as neighbors</param>
  /// <returns>Indices of all neighbor cells</returns>
  public int[] GetNeighboringIndices (int index, bool diagonal) => this.GetNeighboringIndices(this.IndexToCoordinates(index), diagonal);
  /// <summary>
  /// Returns coordinates of all neighboring cells
  /// </summary>
  /// <param name="index">Index to find neighbors for</param>
  /// <param name="diagonal">If diagonally adjacent cells could as neighbors</param>
  /// <returns>Coordinates of all neighbor cells</returns>
  public int[][] GetNeighboringCoordinates (int index, bool diagonal) => this.GetNeighboringCoordinates(this.IndexToCoordinates(index), diagonal);
  /// <summary>
  /// Returns indices of all neighboring cells
  /// </summary>
  /// <param name="coords">Coordinates to find neighbors for</param>
  /// <param name="diagonal">If diagonally adjacent cells could as neighbors</param>
  /// <returns>Indices of all neighbor cells</returns>
  public int[] GetNeighboringIndices (int[] coords, bool diagonal) => this.GetNeighboringCoordinates(coords, diagonal).Select(c => this.CoordinatesToIndex(c)).ToArray();
  /// <summary>
  /// Returns coordinates of all neighboring cells
  /// </summary>
  /// <param name="coords">Coordinates to find neighbors for</param>
  /// <param name="diagonal">If diagonally adjacent cells could as neighbors</param>
  /// <returns>Coordinates of all neighbor cells</returns>
  public int[][] GetNeighboringCoordinates (int[] coords, bool diagonal) {
    var neighbors = new List<List<int>>();
    foreach (var relative in (diagonal ? this.RelativeNeighborCoordsWithDiagonals : this.RelativeNeighborCoordsWithoutDiagonals)!) {
      var valid = true;
      var neighbor = new List<int>();
      for (var i=0; i<this.Dimensions.Length; i++) {
        var x = coords[i] + relative[i];
        neighbor.Add(x);
        if (!((x >= 0) && (x < this.Dimensions[i]))) {
          valid = false;
          break;
        }
      }
      if (valid) neighbors.Add(neighbor);
    }
    return neighbors.Select(c => c.ToArray()).ToArray();
  }
}
