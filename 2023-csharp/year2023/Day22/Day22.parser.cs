namespace ofzza.aoc.year2023.day22;

using ofzza.aoc.utils;

public partial class Day22: ISolution<string, long> {
  private static (long[] Start, long[] End)[] parse (string input) {
    return input.Split('\n').Select(l => {
      var parsed = l.Split('~');
      return (
        parsed[0].Split(',').Select(n => long.Parse(n)).ToArray(),
        parsed[1].Split(',').Select(n => long.Parse(n)).ToArray()
      );
    }).ToArray();
  }
}
