namespace ofzza.aoc.year2023.day21;

using ofzza.aoc.utils;

public partial class Day21: ISolution<(int Count, string Tiles), long> {
  private static char[][] parse ((int Count, string Tiles) input) {
    return input.Tiles.Split('\n').Select(l => l.ToCharArray()).ToArray();
  }
}
