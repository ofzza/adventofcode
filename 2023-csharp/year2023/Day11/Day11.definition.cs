namespace ofzza.aoc.year2023.day11;

using ofzza.aoc.utils;

public partial class Day11: ISolution<(long Expansion, string Space), long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 11;
  
  private static string test = string.Join('\n', new string[] {
    "...#......",
    ".......#..",
    "#.........",
    "..........",
    "......#...",
    ".#........",
    ".........#",
    "..........",
    ".......#..",
    "#...#.....",
  });
  public List<SolutionExecution<(long Expansion, string Space), long>> Executions { get; } = new List<SolutionExecution<(long Expansion, string Space), long>>() {
    // Part #1, Test
    new SolutionExecution<(long Expansion, string Space), long>(1, Tag.Test) {
      InputValue = (2, Day11.test),
      Expect = 374
    },
    // Part #1, Solution
    new SolutionExecution<(long Expansion, string Space), long>(1, Tag.Solution) {
      InputValue = (2, File.ReadAllText("./inputs/Day11/input.txt").Trim()),
      Expect = 9563821
    },
    // Part #2, Test
    new SolutionExecution<(long Expansion, string Space), long>(2, Tag.Test) {
      InputValue = (10, Day11.test),
      Expect = 1030
    },
    // Part #2, Solution
    new SolutionExecution<(long Expansion, string Space), long>(2, Tag.Solution) {
      InputValue = (1000000, File.ReadAllText("./inputs/Day11/input.txt").Trim()),
      Expect = 827009909817
    }
  };
}
