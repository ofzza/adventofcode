namespace ofzza.aoc.year2023.day18;

using ofzza.aoc.utils;

public partial class Day18: ISolution<string, long> {
  private static string[][] parse (string input) {
    return input.Split('\n').Select(l => l.Split(' ')).ToArray();
  }
}
