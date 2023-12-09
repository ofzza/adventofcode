namespace ofzza.aoc.year2023.day09;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day09: ISolution<string, long> {
  private static long[][] parse (string input) {
    return input.Split('\n').Select(l => l.Split(' ').Select(n => long.Parse(n)).ToArray()).ToArray();
  }
}
