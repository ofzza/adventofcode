namespace ofzza.aoc.year2023.day02;

public class Input {
  public required List<CubeGameRoundResult> Limits { init; get; }
  public required List<CubeGame> Games { init; get; }
}
public class CubeGame {
  public required int Index { init; get; }
  public required List<CubeGameRound> Rounds { init; get; }
}
public class CubeGameRound {
  public required List<CubeGameRoundResult> Results { init; get; }
}
public class CubeGameRoundResult {
  public required CubeColor Color { init; get; }
  public required int Count { init; get; }
}
public enum CubeColor {
  Red = 0,
  Green = 1,
  Blue = 2
}
