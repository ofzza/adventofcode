namespace ofzza.aoc.year2023.day05;

using ofzza.aoc.utils.range;

public class Input {
  public required long[] SeedIds { init; get; }
  public required Range<long>[] SeedRanges { init; get; }
  public required KeyMapping[] Mappings { init; get; }
}

public class KeyMapping {
  public required string SourceKey { init; get; }
  public required string DestinationKey { init; get; }
  public required IdMapping[] Mappings { init; get; }
}

public class IdMapping {
  public required Range<long> SourceRange { init; get; }
  public required Range<long> DestinationRange { init; get; }
}
