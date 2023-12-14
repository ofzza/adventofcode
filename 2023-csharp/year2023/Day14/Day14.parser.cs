namespace ofzza.aoc.year2023.day14;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day14: ISolution<string, long> {
  private static char[][] parse (string input) {
    return input.Split('\n').Select(l => l.ToCharArray()).ToArray();
  }
}
