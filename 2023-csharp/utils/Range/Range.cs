namespace ofzza.aoc.utils.range;

using System.Numerics;

/// <summary>
/// Data structure describing a range of indices
/// </summary>
/// <typeparam name="T">Range indices' numeric type</typeparam>
public partial class Range<T> where T: IBinaryInteger<T> {
  /// <summary>
  /// Start index od the range
  /// </summary>
  public required T Start { init; get; }
  /// <summary>
  /// End index od the range
  /// </summary>
  public required T End { init; get; }
}
