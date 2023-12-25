namespace ofzza.aoc.year2023.day24;

using ofzza.aoc.utils;
using ofzza.aoc.utils.vector;

public partial class Day24: ISolution<(double[] Min, double[] Max, string Input), long> {
  private static Vector[] parse ((double[] Min, double[] Max, string Input) input) {
    return input.Input.Split('\n').Select(l => {
      var parsed = l.Split('@');
      return new Vector() {
        Origin = parsed[0].Trim().Split(',').Select(n => double.Parse(n)).ToArray(),
        Magnitude = parsed[1].Trim().Split(',').Select(n => double.Parse(n)).ToArray()
      };
    }).ToArray();
  }
}
