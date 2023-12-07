namespace ofzza.aoc.year2023.day05;

using ofzza.aoc.utils.interval;

public class Input {
  public required long[] SeedIds { init; get; }
  public required Interval<long>[] SeedIntervals { init; get; }
  public required KeyMapping[] Mappings { init; get; }
}

public class KeyMapping {
  public required string SourceKey { init; get; }
  public required string DestinationKey { init; get; }
  public required IdMapping[] Mappings { init; get; }
}

public class IdMapping {
  public required Interval<long> SourceInterval { init; get; }
  public required Interval<long> DestinationInterval { init; get; }
}
