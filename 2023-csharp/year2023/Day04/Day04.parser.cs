namespace ofzza.aoc.year2023.day04;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day04: ISolution<string[], int> {
  private static Card[] parse (string[] input) {
    return input.Select(line => {
      var columns = line.Split(':')[1].Split('|');
      var selected = columns[0].Trim().Split(' ').Where(n => n.Length > 0).Select(n => int.Parse(n));
      var winning = columns[1].Trim().Split(' ').Where(n => n.Length > 0).Select(n => int.Parse(n));
      return new Card() {
        selected = selected.ToArray(),
        winning = winning.ToArray()
      };
    }).ToArray();
  }
}
