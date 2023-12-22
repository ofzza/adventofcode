namespace ofzza.aoc.utils.matrix;

using System;

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
  public long[] Dimensions { init; get; }
  /// <summary>
  /// If infinite plain mode is used.
  /// When enabled, all coordinates are mapped back to the indexed area as if the edges of the area were connected on a torus.
  /// </summary>
  public bool InfinitePlain { init; get; }
  /// <summary>
  /// Gets total length of all cells in the matrix
  /// </summary>
  public long Length { get; private set; }

/// <summary>
/// Constructor
/// </summary>
/// <param name="dimensions">Dimensions of a N-dimensional matrix</param>
/// <exception cref="Exception">Dimensionality not suppoerted</exception>
  public MatrixIndexer (long[] dimensions) {
    // Verify and set dimensionality
    if (dimensions.Length < 1) throw new Exception("Must provide at least a single dimension!");
    this.Dimensions = dimensions;

    // Initialize index
    this.Initialize();
  }

  private long[] DimensionOffsets { get; set; } = new long[0];
  private long[][] RelativeNeighborCoordsWithDiagonals { get; set; } = new long[0][];
  private long[][] RelativeNeighborCoordsWithoutDiagonals { get; set; } = new long[0][];

  private void Initialize () {
    // Calculate length and offsets per dimension
    this.DimensionOffsets = new long[this.Dimensions.Length];
    this.DimensionOffsets[0] = 1;
    long length = this.Dimensions[0];
    long offset = 1;
    for (var i=1; i<this.Dimensions.Length; i++) {
      length *= this.Dimensions[i];
      offset *= this.Dimensions[i - 1];
      this.DimensionOffsets[i] = offset;
    }
    this.Length = length;

    // Calculate relative neighboring coordinates
    var coords = new List<List<long>>() { new() {} };
    for (var i=0; i<this.Dimensions.Length; i++) {
      // Extend all coordinates with an extra dimension
      foreach (var coord in coords) coord.Add(0);
      // For each coordinate, add neighbord in recent dimension
      var next = new List<List<long>>();
      foreach (var coord in coords) {
        next.Add(coord);
        var c1p = new List<long>(coord);
        c1p[c1p.Count - 1] = 1;
        next.Add(c1p);
        var c1m = new List<long>(coord);
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
  public long[] IndexToCoordinates (long index) {
    if (!this.CheckIfValidIndex(index)) throw new Exception("Invalid index provided!");
    index = index >= 0 ? index % this.Length : ((-1 * ((-1 * index) % this.Length)) + this.Length) % this.Length;
    var coords = new long[this.Dimensions.Length];
    var remainder = index;
    for (var i=this.Dimensions.Length - 1; i>=0; i--) {
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
  public long CoordinatesToIndex (long[] coords) {
    if (!this.CheckIfValidCoordinates(coords)) throw new Exception("Invalid coordinates provided!");
    coords = new long[] {
      coords[0] >= 0 ? coords[0] % this.Dimensions[0] : ((-1 * ((-1 * coords[0]) % this.Dimensions[0])) + this.Dimensions[0]) % this.Dimensions[0],
      coords[1] >= 0 ? coords[1] % this.Dimensions[1] : ((-1 * ((-1 * coords[1]) % this.Dimensions[1])) + this.Dimensions[1]) % this.Dimensions[1]
    };
    long index = 0;
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
  public long[] GetNeighboringIndices (long index, bool diagonal) => this.GetNeighboringIndices(this.IndexToCoordinates(index), diagonal);
  /// <summary>
  /// Returns coordinates of all neighboring cells
  /// </summary>
  /// <param name="index">Index to find neighbors for</param>
  /// <param name="diagonal">If diagonally adjacent cells could as neighbors</param>
  /// <returns>Coordinates of all neighbor cells</returns>
  public long[][] GetNeighboringCoordinates (long index, bool diagonal) => this.GetNeighboringCoordinates(this.IndexToCoordinates(index), diagonal);
  /// <summary>
  /// Returns indices of all neighboring cells
  /// </summary>
  /// <param name="coords">Coordinates to find neighbors for</param>
  /// <param name="diagonal">If diagonally adjacent cells could as neighbors</param>
  /// <returns>Indices of all neighbor cells</returns>
  public long[] GetNeighboringIndices (long[] coords, bool diagonal) => this.GetNeighboringCoordinates(coords, diagonal).Select(c => this.CoordinatesToIndex(c)).ToArray();
  /// <summary>
  /// Returns coordinates of all neighboring cells
  /// </summary>
  /// <param name="coords">Coordinates to find neighbors for</param>
  /// <param name="diagonal">If diagonally adjacent cells could as neighbors</param>
  /// <returns>Coordinates of all neighbor cells</returns>
  public long[][] GetNeighboringCoordinates (long[] coords, bool diagonal) {
    if (!this.CheckIfValidCoordinates(coords)) throw new Exception("Invalid coordinates provided!");
    var neighbors = new List<List<long>>();
    foreach (var relative in (diagonal ? this.RelativeNeighborCoordsWithDiagonals : this.RelativeNeighborCoordsWithoutDiagonals)!) {
      var valid = true;
      var neighbor = new List<long>();
      for (var i=0; i<this.Dimensions.Length; i++) {
        var x = coords[i] + relative[i];
        neighbor.Add(x);
        if (!this.InfinitePlain && !((x >= 0) && (x < this.Dimensions[i]))) {
          valid = false;
          break;
        }
      }
      if (valid) neighbors.Add(neighbor);
    }
    return neighbors.Select(c => c.ToArray()).ToArray();
  }

  /// <summary>
  /// Checks if index is a valid index inside the scope of the matrix
  /// </summary>
  /// <param name="index">Index to verify</param>
  /// <returns>If provided index is a valid index inside the scope of the matrix</returns>
  public bool CheckIfValidIndex (long index) {
    if (this.InfinitePlain) return true;
    return index >= 0 && index < this.Length;
  }
  /// <summary>
  /// Checks if coordinates are valid and inside the scope of the matrix
  /// </summary>
  /// <param name="index">Coordinates to verify</param>
  /// <returns>If provided coordinates are valid and inside the scope of the matrix</returns>
  public bool CheckIfValidCoordinates (long[] coords) {
    if (this.InfinitePlain) return true;
    if (coords.Length != this.Dimensions.Length) return false;
    for (var i=0; i<coords.Length; i++) {
      if (coords[i] < 0 || coords[i] >= this.Dimensions[i]) return false;
    }
    return true;
  }
}
