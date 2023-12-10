namespace ofzza.aoc.year2023.utils.camelcards;

/// <summary>
/// A hand of Camel cards
/// </summary>
public class Hand {
  public required char[] Cards { init; get; }
}

/// <summary>
/// A processed (evaluated) hand of camel cards
/// </summary>
public class ProcessedHand: Hand {
  public required Type Type { init; get; }
  public required int Value { init; get; }
}

/// <summary>
/// Hand types, card combination
/// </summary>
public enum Type {
  HighCard = 1,
  OnePair = 2,
  TwoPair = 3,
  ThreeOfAKind = 4,
  FullHouse = 5,
  FourOfAKind = 6,
  FiveOfAKind = 7,
}
