namespace ofzza.aoc.utils.range;

using System.Numerics;

/// <summary>
/// Result of calculating overlap of two ranges
/// </summary>
/// <typeparam name="T">Range indices' numeric type</typeparam>
public class RangeOverlap<T> where T: IBinaryInteger<T> {
  /// <summary>
  /// Range which is a resulting overlap of two other ranges
  /// </summary>
  public Range<T>? Overlap { init; get; }
  /// <summary>
  /// Array of remaining ranges after taking an overlap of two other ranges
  /// </summary>
  public (Range<T>[], Range<T>[]) Remainders { init; get; }
}

public partial class Range<T> where T: IBinaryInteger<T> {
  /// <summary>
  /// Calculates overlap of two ranges
  /// </summary>
  /// <param name="a">First range to calculate overlap for</param>
  /// <param name="b">Second range to calculate overlap for</param>
  /// <returns>Range overlap object</returns>
  public static RangeOverlap<T> GetOverlap (Range<T> a, Range<T> b) {
    // Validate ranges
    var first = a.Start <= a.End ? a : new Range<T>() { Start = a.End, End = a.Start };
    var second = b.Start <= b.End ? b : new Range<T>() { Start = b.End, End = b.Start };
    // Check no overlap
    if (first.End < second.Start || first.Start > second.End) {
      return new RangeOverlap<T>() {
        Overlap = null,
        Remainders = (new [] { first }, new[] { second })
      };
    }
    // Check if full subset
    if (first.Start >= second.Start && first.End <= second.End) {
      var remainders = new List<Range<T>>();
      if (first.Start > second.Start) {
        remainders.Add(new Range<T>() { Start = second.Start, End = first.Start - T.One });
      }
      if (first.End < second.End) {
        remainders.Add(new Range<T>() { Start = first.End + T.One, End = second.End });
      }
      return new RangeOverlap<T>() {
        Overlap = new Range<T>() { Start = first.Start, End = first.End },
        Remainders = (new Range<T>[] {}, remainders.ToArray())
      };
    }
    if (second.Start >= first.Start && second.End <= first.End) {
      var remainders = new List<Range<T>>();
      if (second.End < first.End) {
        remainders.Add(new Range<T>() { Start = second.End + T.One, End = first.End });
      }
      if (second.Start > first.Start) {
        remainders.Add(new Range<T>() { Start = first.Start, End = second.Start - T.One });
      }
      return new RangeOverlap<T>() {
        Overlap = new Range<T>() { Start = second.Start, End = second.End },
        Remainders = (remainders.ToArray(), new Range<T>[] {})
      };
    }
    // Check if partial overlap
    if (first.Start < second.Start && first.End >= second.Start && first.End < second.End) {
      return new RangeOverlap<T>() {
        Overlap = new Range<T>() { Start = second.Start, End = first.End },
        Remainders = (new [] { new Range<T>() { Start = first.Start, End = second.Start - T.One }}, new [] { new Range<T>() { Start = first.End + T.One, End = second.End } })
      };
    }
    if (second.Start < first.Start && second.End >= first.Start && second.End < first.End) {
      return new RangeOverlap<T>() {
        Overlap = new Range<T>() { Start = first.Start, End = second.End },
        Remainders = (new [] { new Range<T>() { Start = second.End + T.One, End = first.End } }, new [] { new Range<T>() { Start = second.Start, End = first.Start - T.One } })
      };
    }
    // If no overlap variation found, throw error
    throw new Exception("Overlap detection failed! This should never ever happen!");
  }
}
