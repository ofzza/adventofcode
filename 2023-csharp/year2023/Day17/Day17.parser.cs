namespace ofzza.aoc.year2023.day17;

using ofzza.aoc.utils;

public partial class Day17: ISolution<string, long> {
  private static char[][] parse (string input) {
    return input.Split('\n').Select(l => l.ToCharArray()).ToArray();
  }
}
