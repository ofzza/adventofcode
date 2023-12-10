namespace ofzza.aoc.year2023.day10;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day10: ISolution<string, long> {
  private static char[][] parse (string input) {
    return input.Split('\n').Select(l => l.ToCharArray()).ToArray();
  }
}
