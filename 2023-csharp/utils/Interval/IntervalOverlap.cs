namespace ofzza.aoc.utils.interval;

using System.Numerics;

/// <summary>
/// Result of calculating overlap of two intervals
/// </summary>
/// <typeparam name="T">Interval indices' numeric type</typeparam>
public class IntervalOverlap<T> where T: IBinaryInteger<T> {
  /// <summary>
  /// Interval which is a resulting overlap of two other intervals
  /// </summary>
  public Interval<T>? Overlap { init; get; }
  /// <summary>
  /// Array of remaining intervals after taking an overlap of two other intervals
  /// </summary>
  public (Interval<T>[] FirstRemainder, Interval<T>[] SecondRemainder) Remainders { init; get; }
}

public partial class Interval<T> where T: IBinaryInteger<T> {
  /// <summary>
  /// Calculates overlap of two intervals
  /// </summary>
  /// <param name="a">First interval to calculate overlap for</param>
  /// <param name="b">Second interval to calculate overlap for</param>
  /// <returns>Interval overlap object</returns>
  public static IntervalOverlap<T> GetOverlap (Interval<T> a, Interval<T> b) {
    // Validate intervals
    var first = a.Start <= a.End ? a : new Interval<T>() { Start = a.End, End = a.Start };
    var second = b.Start <= b.End ? b : new Interval<T>() { Start = b.End, End = b.Start };
    // Check no overlap
    if (first.End < second.Start || first.Start > second.End) {
      return new IntervalOverlap<T>() {
        Overlap = null,
        Remainders = (new [] { first }, new[] { second })
      };
    }
    // Check if full subset
    if (first.Start >= second.Start && first.End <= second.End) {
      var remainders = new List<Interval<T>>();
      if (first.Start > second.Start) {
        remainders.Add(new Interval<T>() { Start = second.Start, End = first.Start - T.One });
      }
      if (first.End < second.End) {
        remainders.Add(new Interval<T>() { Start = first.End + T.One, End = second.End });
      }
      return new IntervalOverlap<T>() {
        Overlap = new Interval<T>() { Start = first.Start, End = first.End },
        Remainders = (new Interval<T>[] {}, remainders.ToArray())
      };
    }
    if (second.Start >= first.Start && second.End <= first.End) {
      var remainders = new List<Interval<T>>();
      if (second.End < first.End) {
        remainders.Add(new Interval<T>() { Start = second.End + T.One, End = first.End });
      }
      if (second.Start > first.Start) {
        remainders.Add(new Interval<T>() { Start = first.Start, End = second.Start - T.One });
      }
      return new IntervalOverlap<T>() {
        Overlap = new Interval<T>() { Start = second.Start, End = second.End },
        Remainders = (remainders.ToArray(), new Interval<T>[] {})
      };
    }
    // Check if partial overlap
    if (first.Start < second.Start && first.End >= second.Start && first.End < second.End) {
      return new IntervalOverlap<T>() {
        Overlap = new Interval<T>() { Start = second.Start, End = first.End },
        Remainders = (new [] { new Interval<T>() { Start = first.Start, End = second.Start - T.One }}, new [] { new Interval<T>() { Start = first.End + T.One, End = second.End } })
      };
    }
    if (second.Start < first.Start && second.End >= first.Start && second.End < first.End) {
      return new IntervalOverlap<T>() {
        Overlap = new Interval<T>() { Start = first.Start, End = second.End },
        Remainders = (new [] { new Interval<T>() { Start = second.End + T.One, End = first.End } }, new [] { new Interval<T>() { Start = second.Start, End = first.Start - T.One } })
      };
    }
    // If no overlap variation found, throw error
    throw new Exception("Overlap detection failed! This should never ever happen!");
  }
}
