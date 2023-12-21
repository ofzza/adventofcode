namespace ofzza.aoc.year2023.day16;

using ofzza.aoc.utils;

public partial class Day16: ISolution<string, long> {
  private static char[][] parse (string input) {
    return input.Split('\n').Select(l => l.ToCharArray()).ToArray();
  }
}
