namespace ofzza.aoc.year2023.day07;

using System.Linq;
using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.camelcards;

public partial class Day07: ISolution<string, long> {
  private static Input parse (string input, bool jokers = false) {
    return new Input() {
      Rounds = input.Split('\n').Select(l => {
        var parsed = l.Split(' ');
        var cards = parsed[0].ToArray();
        var hand = new Hand() { Cards = cards! };
        var bet = int.Parse(parsed[1]);
        return new Round() {
          Hand = hand,
          ProcessedHand = CamelCards.process(hand, jokers),
          Bet = bet
        };
      }).ToArray()
    };
  }
}
