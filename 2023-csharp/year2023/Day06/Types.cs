namespace ofzza.aoc.year2023.day06;

public class Input {
  public required Race[] Races { init; get; }
}

public class Race {
  public required long TotalTime { init; get; }
  public required long MaxDistance { init; get; }
}
