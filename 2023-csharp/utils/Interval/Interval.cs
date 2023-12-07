namespace ofzza.aoc.utils.interval;

using System.Numerics;

/// <summary>
/// Data structure describing a interval of indices
/// </summary>
/// <typeparam name="T">Interval indices' numeric type</typeparam>
public partial class Interval<T> where T: IBinaryInteger<T> {
  /// <summary>
  /// Start index od the interval
  /// </summary>
  public required T Start { init; get; }
  /// <summary>
  /// End index od the interval
  /// </summary>
  public required T End { init; get; }
}
