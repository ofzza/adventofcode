namespace ofzza.aoc.year2023.utils.camelcards;

/// <summary>
/// Implements functionality required for playing Camel Cards
/// </summary>
public class CamelCards {
  /// <summary>
  /// (Pre)processes a hand into a easier to work with object
  /// </summary>
  /// <param name="hand">Hand of game of Camel cards to be processes</param>
  /// <param name="jokers">If 'J' cards are jokers</param>
  /// <returns>Easier to work with object, (pre)processes hand object</returns>
  public static ProcessedHand process (Hand hand, bool jokers = false) {
    var type = CamelCards.GetType(hand, jokers);
    var value = CamelCards.GetHandValue(hand, type, jokers);
    return new ProcessedHand() {
      Cards = hand.Cards,
      Type = type,
      Value = value
    };
  }

  /// <summary>
  /// Detects hand type
  /// </summary>
  /// <param name="hand">Hand to detect the type of</param>
  /// <param name="jokers">If 'J' cards are jokers</param>
  /// <returns>Hand type</returns>
  /// <exception cref="Exception"></exception>
  private static Type GetType (Hand hand, bool jokers = false) {
    var buckets = new int[13] { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
    foreach (var card in hand.Cards) buckets[CamelCards.GetCardIntegerValue(card, jokers)]++;
    var jokersCount = buckets[0]; // Keep count of joker valued cards, in case dealing with jokers, and remove them as card type
    if (jokers) { buckets[0] = 0; }
    Array.Sort(buckets);
    var sortedCardCounts = buckets.Where(n => n > 0).ToArray();
    if (sortedCardCounts.Length == 0) sortedCardCounts = new int[] { 0 };
    if (jokers) { // Add jokers to top card type
      sortedCardCounts[sortedCardCounts.Length - 1] += jokersCount;
      if (sortedCardCounts[sortedCardCounts.Length - 1] > 5) sortedCardCounts[sortedCardCounts.Length - 1] = 5;
    }
    var type = string.Join("", sortedCardCounts.Select(n => n.ToString()));
    switch (type) {
      case "5": return Type.FiveOfAKind;
      case "14": return Type.FourOfAKind;
      case "23": return Type.FullHouse;
      case "113": return Type.ThreeOfAKind;
      case "122": return Type.TwoPair;
      case "1112": return Type.OnePair;
      case "11111": return Type.HighCard;
    }
    throw new Exception("Unknown card hand type! This should never, ever happen!");
  }

  /// <summary>
  /// Gets a unique value of the hand
  /// </summary>
  /// <param name="hand">Hand to evaluate</param>
  /// <param name="jokers">If 'J' cards are jokers</param>
  /// <param name="type">Precalculated hand type</param>
  /// <returns>Comparable value of the hand</returns>
  private static int GetHandValue (Hand hand, Type type, bool jokers = false) {
    var hex = string.Join("", hand.Cards.Select(c => CamelCards.GetCardHexValue(c, jokers).ToString()));
    return Convert.ToInt32($"""0x{(int)type}{hex}""", 16);
  }

  /// <summary>
  /// Gets card type as integer
  /// </summary>
  /// <param name="card">Card to convert to integer</param>
  /// <param name="jokers">If 'J' cards are jokers</param>
  /// <returns>Card as integer</returns>
  /// <exception cref="Exception"></exception>
  private static int GetCardIntegerValue (char card, bool jokers = false) {
    switch (card) {
      case '2': return !jokers ? 0 : 1;
      case '3': return !jokers ? 1 : 2;
      case '4': return !jokers ? 2 : 3;
      case '5': return !jokers ? 3 : 4;
      case '6': return !jokers ? 4 : 5;
      case '7': return !jokers ? 5 : 6;
      case '8': return !jokers ? 6 : 7;
      case '9': return !jokers ? 7 : 8;
      case 'T': return !jokers ? 8 : 9;
      case 'J': return !jokers ? 9 : 0;
      case 'Q': return !jokers ? 10 : 10;
      case 'K': return !jokers ? 11 : 11;
      case 'A': return !jokers ? 12 : 12;
    }
    throw new Exception("Unknown card type! This should never, ever happen!");
  }

  /// <summary>
  /// Gets card type as a hex digit
  /// </summary>
  /// <param name="card">Card to convert to a hex digit</param>
  /// <param name="jokers">If 'J' cards are jokers</param>
  /// <returns>Card as hex digit</returns>
  /// <exception cref="Exception"></exception>
  private static char GetCardHexValue (char card, bool jokers = false) {
    switch (card) {
      case '2': return !jokers ? '0' : '1';
      case '3': return !jokers ? '1' : '2';
      case '4': return !jokers ? '2' : '3';
      case '5': return !jokers ? '3' : '4';
      case '6': return !jokers ? '4' : '5';
      case '7': return !jokers ? '5' : '6';
      case '8': return !jokers ? '6' : '7';
      case '9': return !jokers ? '7' : '8';
      case 'T': return !jokers ? '8' : '9';
      case 'J': return !jokers ? '9' : '0';
      case 'Q': return !jokers ? 'A' : 'A';
      case 'K': return !jokers ? 'B' : 'B';
      case 'A': return !jokers ? 'C' : 'C';
    }
    throw new Exception("Unknown card type! This should never, ever happen!");
  }

}
