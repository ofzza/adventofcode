namespace ofzza.aoc.utils.primes;

using System.Numerics;
using ofzza.aoc.utils.matrix;

/// <summary>
/// Utility class containing typical numeric sequence manipulation functions
/// </summary>
/// <typeparam name="T">Sequence members' numeric type</typeparam>
public class NumericSequence<T> where T: IBinaryInteger<T> {

  /// <summary>
  /// Holds the numeric sequence being processed
  /// </summary>
  public required T[] Sequence { init; get; }

  /// <summary>
  /// Extrapolates the next member of the sequence using the differences table method
  /// </summary>
  /// <param name="direction">Direction to extrapolate into (default "Right", meaning next value of the sequence; alternative "Left", meaning preceding value to the sequence</param>
  /// <returns>Next (extrapolated) member of the sequence</returns>
  public T ExtrapolateUsingDifferencesTableMethod (Direction direction = Direction.Right) {
    // Initialize differences table
    var table = new T[this.Sequence.Length * this.Sequence.Length];
    var index = new MatrixIndexer(new long[] { this.Sequence.Length, this.Sequence.Length });
    // Write first row into the differences table
    for (var i=0; i<this.Sequence.Length; i++) table[index.CoordinatesToIndex(new long[] { 0, i })] =this.Sequence[i];
    // Calculate difference rows
    var level = 0;
    var zeroed = true;
    do {
      zeroed = true;
      for (var i=1; i<this.Sequence.Length - level; i++) {
        var a = table[index.CoordinatesToIndex(new long[] { level, i - 1 })];
        var b = table[index.CoordinatesToIndex(new long[] { level, i })];
        var diff = b - a;
        if (diff != null && diff != T.Zero) zeroed = false;
        table[index.CoordinatesToIndex(new long[] { level + 1, i - 1 })] = diff!;
      }
      level ++;
    } while (!zeroed);
    // Extrapolate value
    var extrapolation = new T[level];
    for (var i=level; i>0; i--) {
      var val = table[index.CoordinatesToIndex(new long[] { i - 1, direction == Direction.Right ? this.Sequence.Length - i : 0 })];      
      var diff = (i > level - 1 ? T.Zero : extrapolation[i]);
      extrapolation[i - 1] = direction == Direction.Right ? val + diff : val - diff;
    }
    // Return extrapolation
    return extrapolation[0];
  }

}
