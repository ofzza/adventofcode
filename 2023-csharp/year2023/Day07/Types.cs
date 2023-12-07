namespace ofzza.aoc.year2023.day07;

public class Input {
  public required Round[] Rounds { init; get; }
}

public class Round {
  public required Hand Hand { init; get; }
  public required ProcessedHand ProcessedHand { init; get; }
  public required int Bet { init; get; }
}

public class Hand {
  public required char[] Cards { init; get; }
}

public class ProcessedHand: Hand {
  public required Type Type { init; get; }
  public required int Value { init; get; }
}

public enum Type {
  HighCard = 1,
  OnePair = 2,
  TwoPair = 3,
  ThreeOfAKind = 4,
  FullHouse = 5,
  FourOfAKind = 6,
  FiveOfAKind = 7,
}

/// <summary>
/// Comparer of Camel cards rounds, used for sorting rounds based on hand value
/// </summary>
public class RoundHandValueComparer : IComparer<Round> {
    public int Compare(Round? x, Round? y) {
        if (x == null && y == null) return 0; 
        if (x == null) return -1; 
        if (y == null) return 1; 
        return  x.ProcessedHand.Value.CompareTo(y.ProcessedHand.Value);
    }
}
