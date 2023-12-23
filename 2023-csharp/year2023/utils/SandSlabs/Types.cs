namespace ofzza.aoc.year2023.utils.sandslabs;

public class Slab {
  public long[] Min { init; get; }
  public long[] Max { init; get; }

  public List<int> SlabsStackedOnTop = new List<int>();
  public List<int> SlabsStackedOnTopOf = new List<int>();

  public Slab (long[] start, long[] end) {
    // Store coordinates
    this.Min = new long[] {
      start[0] < end[0] ? start[0] : end[0],
      start[1] < end[1] ? start[1] : end[1],
      start[2] < end[2] ? start[2] : end[2]
    };
    this.Max = new long[] {
      start[0] > end[0] ? start[0] : end[0],
      start[1] > end[1] ? start[1] : end[1],
      start[2] > end[2] ? start[2] : end[2]
    };
  }
}

/// <summary>
/// Comparer of Slabs
/// </summary>
public class SlabBottomHeightComparer : IComparer<Slab> {
    public int Compare(Slab? a, Slab? b) {
      return a!.Min[2].CompareTo(b!.Min[2]);
    }
}
