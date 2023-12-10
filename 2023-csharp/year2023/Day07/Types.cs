namespace ofzza.aoc.year2023.day07;

using ofzza.aoc.year2023.utils.camelcards;

public class Input {
  public required Round[] Rounds { init; get; }
}

public class Round {
  public required Hand Hand { init; get; }
  public required ProcessedHand ProcessedHand { init; get; }
  public required int Bet { init; get; }
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
