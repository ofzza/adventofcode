namespace ofzza.aoc.year2023.day13;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day13: ISolution<string, long> {
  private static char[][][] parse (string input) {
    return input.Split("\n\n").Select(m => m.Split('\n').Select(l => l.ToCharArray()).ToArray()).ToArray();
  }
}
