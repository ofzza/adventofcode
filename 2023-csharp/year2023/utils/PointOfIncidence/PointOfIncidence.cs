namespace ofzza.aoc.year2023.utils.pointsofincidence;

using ofzza.aoc.utils.matrix;

public class PointOfIncidence {

  /// <summary>
  /// Indexes into the provided field
  /// </summary>
  private MatrixIndexer TilesIndex { init; get; }

  /// <summary>
  /// Holds all the field tiles
  /// </summary>
  private char[] Tiles { init; get; }

  /// <summary>
  /// Processed view identity in each direction, for each tile
  /// </summary>
  private ((string Top, string Bottom)[] Horizontal, (string Left, string Right)[] Vertical) Views { init; get; }

  private (MatrixIndexer Horizontal, MatrixIndexer Vertical) ViewsIndex { init; get; }

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="input">Field data</param>
  public PointOfIncidence (char[][] input) {
    // Store field and create index
    this.Tiles = string.Join("", input.Select(l => string.Join("", l))).ToCharArray();
    this.TilesIndex = new MatrixIndexer(new long[] { input[0].Length, input.Length });
    // Initialize views
    this.Views = (
      new (string Top, string Bottom)[this.TilesIndex.Dimensions[0] * (this.TilesIndex.Dimensions[1] - 1)],
      new (string Left, string Right)[(this.TilesIndex.Dimensions[0] - 1) * this.TilesIndex.Dimensions[1]]
    );    
    this.ViewsIndex = (
      new MatrixIndexer(new long[] { this.TilesIndex.Dimensions[0], this.TilesIndex.Dimensions[1] - 1 }),
      new MatrixIndexer(new long[] { this.TilesIndex.Dimensions[0] - 1, this.TilesIndex.Dimensions[1] })
    );

  }
  
  /// <summary>
  /// Generates view identities for each tile, row and column
  /// </summary>
  public void GenerateViews () {
    // Initialize reduced view
    for (var i=0; i<this.Views.Horizontal.Length; i++) this.Views.Horizontal[i] = ("", "");
    for (var i=0; i<this.Views.Vertical.Length; i++) this.Views.Vertical[i] = ("", "");
    // Process tiles's vertical
    for (var y=0; y<this.TilesIndex.Dimensions[1]; y++) {
      // Update and store view to left
      var xLSum = "";
      for (var x=0; x<this.ViewsIndex.Vertical.Dimensions[0]; x++) {
        xLSum = this.Tiles[this.TilesIndex.CoordinatesToIndex(new long[] { x, y })] + xLSum;
        this.Views.Vertical[this.ViewsIndex.Vertical.CoordinatesToIndex(new long[] { x, y })].Left = xLSum;
      }
      // Update and store view to right
      var xRSum = "";
      for (var x=this.ViewsIndex.Vertical.Dimensions[0] - 1; x>=0; x--) {
        xRSum = this.Tiles[this.TilesIndex.CoordinatesToIndex(new long[] { x + 1, y })] + xRSum;
        this.Views.Vertical[this.ViewsIndex.Vertical.CoordinatesToIndex(new long[] { x, y })].Right = xRSum;
      }
    }
     // Process tiles's horizontal
    for (var x=0; x<this.TilesIndex.Dimensions[0]; x++) {
      // Update and store view to top
      var yTSum = "";
      for (var y=0; y<this.ViewsIndex.Horizontal.Dimensions[1]; y++) {
        yTSum = this.Tiles[this.TilesIndex.CoordinatesToIndex(new long[] { x, y })] + yTSum;
        this.Views.Horizontal[this.ViewsIndex.Horizontal.CoordinatesToIndex(new long[] { x, y })].Top = yTSum;
      }
      // Update and store view to bottom
      var yBSum = "";
      for (var y=this.ViewsIndex.Horizontal.Dimensions[1] - 1; y>=0; y--) {
        yBSum = this.Tiles[this.TilesIndex.CoordinatesToIndex(new long[] { x, y + 1 })] + yBSum;
        this.Views.Horizontal[this.ViewsIndex.Horizontal.CoordinatesToIndex(new long[] { x, y })].Bottom = yBSum;
      }
    }
 }

  /// <summary>
  /// Searches for a horizontal mirroring plane
  /// </summary>
  /// <returns>Index of the horizontal mirroring plane, or null if plane not found</returns>
  public int? FindHorizontalMirroringPlain () {
    for (var y=0; y<this.ViewsIndex.Horizontal.Dimensions[1]; y++) {
      var matched = true;
      for (var x=0; x<this.ViewsIndex.Horizontal.Dimensions[0]; x++) {
        var plain = this.Views.Horizontal[this.ViewsIndex.Horizontal.CoordinatesToIndex(new long[] { x, y })];
        var length = plain.Top.Length < plain.Bottom.Length ? plain.Top.Length : plain.Bottom.Length;
        if (plain.Top.Substring(0, length) != plain.Bottom.Substring(0, length)) {
          matched = false;
          break;
        }
      }
      if (matched) return y;
    }

    return null;
  }  
  /// <summary>
  /// Searches for a vertical mirroring plane
  /// </summary>
  /// <returns>Index of the vertical mirroring plane, or null if plane not found</returns>
  public int? FindVerticalMirroringPlain () {
    for (var x=0; x<this.ViewsIndex.Vertical.Dimensions[0]; x++) {
      var matched = true;
      for (var y=0; y<this.ViewsIndex.Vertical.Dimensions[1]; y++) {
        var plain = this.Views.Vertical[this.ViewsIndex.Vertical.CoordinatesToIndex(new long[] { x, y })];
        var length = plain.Left.Length < plain.Right.Length ? plain.Left.Length : plain.Right.Length;
        if (plain.Left.Substring(0, length) != plain.Right.Substring(0, length)) {
          matched = false;
          break;
        }
      }
      if (matched) return x;
    }

    return null;
  }

  /// <summary>
  /// Searches for a horizontal mirroring plane with a single column not being mirrored
  /// </summary>
  /// <returns>Index of the horizontal mirroring plane, or null if plane not found</returns>
  public int? FindHorizontalSmudgedMirroringPlain () {
    for (var y=0; y<this.ViewsIndex.Horizontal.Dimensions[1]; y++) {
      var mismatched = 0;
      for (var x=0; x<this.ViewsIndex.Horizontal.Dimensions[0]; x++) {
        var plain = this.Views.Horizontal[this.ViewsIndex.Horizontal.CoordinatesToIndex(new long[] { x, y })];
        var length = plain.Top.Length < plain.Bottom.Length ? plain.Top.Length : plain.Bottom.Length;
        if (plain.Top.Substring(0, length) != plain.Bottom.Substring(0, length)) {
          mismatched++;
          if (mismatched > 1) break;
        }
      }
      if (mismatched == 1) return y;
    }

    return null;
  }  
  /// <summary>
  /// Searches for a vertical mirroring plane with a single column not being mirrored
  /// </summary>
  /// <returns>Index of the vertical mirroring plane, or null if plane not found</returns>
  public int? FindVerticalSmudgedMirroringPlain () {
    for (var x=0; x<this.ViewsIndex.Vertical.Dimensions[0]; x++) {
      var mismatched = 0;
      for (var y=0; y<this.ViewsIndex.Vertical.Dimensions[1]; y++) {
        var plain = this.Views.Vertical[this.ViewsIndex.Vertical.CoordinatesToIndex(new long[] { x, y })];
        var length = plain.Left.Length < plain.Right.Length ? plain.Left.Length : plain.Right.Length;
        if (plain.Left.Substring(0, length) != plain.Right.Substring(0, length)) {
          mismatched++;
          if (mismatched > 1) break;
        }
      }
      if (mismatched == 1) return x;
    }

    return null;
  }

}
