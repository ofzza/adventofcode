namespace ofzza.aoc.utils.vector;

using System;

/// <summary>
/// Helps manipulating a N-dimensional vectors, by:
/// - Finding intersections
/// </summary>
public class Vector {

  /// <summary>
  /// Finds an intersection (if one exists) between 2 lines defined by vectors
  /// </summary>
  /// <param name="a">Vector defining the first line</param>
  /// <param name="b">Vector defining the second line</param>
  /// <param name="plain">Optional plain definition, if provided, will reduce the intersection calculation to the single plain only</param>
  /// <returns>Intersection if one exist, null if no intersection exists, or if lines are on top of each other</returns>
  /// <exception cref="Exception"></exception>
  public static double[]? FindLineIntersection (Vector a, Vector b, (int PrimaryDimensionIndex, int SecondaryDimensionIndex)? plain = null) {
    // Check if vectors of same rank
    if (a.GetVectorRank() != b.GetVectorRank()) throw new Exception("Only vectors of same rank can be intersected!");

    // If plain provided, find intersection within the plain
    if (plain != null) {
      var p = ((int PrimaryDimensionIndex, int SecondaryDimensionIndex))plain!;

      // Check if vectors exist in requested plain
      if (plain != null) {
        if (p.PrimaryDimensionIndex >= a.GetVectorRank() || p.SecondaryDimensionIndex >= a.GetVectorRank()) throw new Exception("Vectors provided exist outside of the plain given! ");
      }

      // Get 2D lines from vectors
      var a2d = new Vector() {
        Origin = new double[] { a.Origin[p.PrimaryDimensionIndex], a.Origin[p.SecondaryDimensionIndex] },
        Magnitude = new double[] { a.Magnitude[p.PrimaryDimensionIndex], a.Magnitude[p.SecondaryDimensionIndex] }
      };
      var aline = a2d.Get2DLine();
      var b2d = new Vector() {
        Origin = new double[] { b.Origin[p.PrimaryDimensionIndex], b.Origin[p.SecondaryDimensionIndex] },
        Magnitude = new double[] { b.Magnitude[p.PrimaryDimensionIndex], b.Magnitude[p.SecondaryDimensionIndex] }
      };
      var bline = b2d.Get2DLine();

      // Check if lines parallel
      if (aline.k == bline.k) return null;

      // Find 2D intersection
      var xi = ((double)bline.b - (double)aline.b) / ((double)aline.k - (double)bline.k); // x = (b2 - b1) / (k1 - k2)
      var yi = (double)aline.k * (double)xi + (double)aline.b;                            // y = k1 * x + b1
      return new double[] { xi, yi };
    }

    // If plain not provided, find intersection within full spacial volume
    else throw new Exception("Not implemented!");
  }

  public required double[] Origin { init; get; }
  public required double[] Magnitude { init; get; }

  /// <summary>
  /// Gets vectors rank (number of dimensions)
  /// </summary>
  /// <returns>Rank of the provided vector</returns>
  /// <exception cref="Exception"></exception>
  public double GetVectorRank () {
    if (this.Origin.Length != this.Magnitude.Length) throw new Exception ("Origin and Magnitude must be of same rank!");
    return this.Origin.Length;
  }

  /// <summary>
  /// Returns a 2D line represented by this vector using a parametrized representation: (k, b) | f(x) = kx + b)
  /// </summary>
  /// <param name="plain">Optional plain in which the 2D line is to be calculated</param>
  /// <returns>A line represented by this vector</returns>
  /// <exception cref="Exception"></exception>
  public (double k, double b) Get2DLine ((int PrimaryDimensionIndex, int SecondaryDimensionIndex)? plain = null) {
    // Check if 2D vector
    if (plain == null && this.GetVectorRank() != 2) throw new Exception("Only a 2D vector can be used to generate a 2D line! Specify a plan, or provide a 2D vector.");
    // Extract a 2D vector
    var p = ((int PrimaryDimensionIndex, int SecondaryDimensionIndex))(plain != null ? plain! : (0, 1));
    var v = plain == null ? this : new Vector() {
      Origin = new double[] { this.Origin[p.PrimaryDimensionIndex], this.Origin[p.SecondaryDimensionIndex] },
      Magnitude = new double[] { this.Magnitude[p.PrimaryDimensionIndex], this.Magnitude[p.SecondaryDimensionIndex] }
    };
    // Calculate a line
    var x1 = v.Origin[0];
    var y1 = v.Origin[1];
    var x2 = v.Origin[0] + v.Magnitude[0];
    var y2 = v.Origin[1] + v.Magnitude[1];    
    var k = ((double)y2 - (double)y1) / ((double)x2 - (double)x1); // k = dy / dx = (y2 - y1) / (x2 - x1)    
    var b = (double)y1 - ((double)k * (double)x1);                 // b = y - kx
    // Return line definition
    return (k, b);
  }
    
  /// <summary>
  /// Gets how much a vector needs to be scaled to reach a point
  /// </summary>
  /// <param name="p">Point to scale to</param>
  /// <returns>How much a vector needs to be scaled to reach a point</returns>
  /// <exception cref="Exception"></exception>
  public double? GetScaleToPoint (double[] p) {
    // Check if point of same rank as vector
    if (p.Length != this.GetVectorRank()) throw new Exception("Point must be of same rank as the vector it is scaling!");
    // Calculate scale to the point
    var scale = ((double)p[0] - (double)this.Origin[0]) / (double)this.Magnitude[0];
    // Check scale equal across all dimensions
    for (var i=1; i<this.GetVectorRank(); i++) {
      // IF scale doesn't match, point not on vector
      if (scale != ((double)p[0] - (double)this.Origin[0]) / (double)this.Magnitude[0]) return null;
    }
    // Return scale
    return scale;
  }
}
