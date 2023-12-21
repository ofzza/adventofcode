namespace ofzza.aoc.year2023.day18;

using ofzza.aoc.utils;

public partial class Day18: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 18;
  
  private static string test = string.Join('\n', new string[] {
    "R 6 (#70c710)",
    "D 5 (#0dc571)",
    "L 2 (#5713f0)",
    "D 2 (#d2c081)",
    "R 2 (#59c680)",
    "D 2 (#411b91)",
    "L 5 (#8ceee2)",
    "U 2 (#caa173)",
    "L 1 (#1b58a2)",
    "U 2 (#caa171)",
    "R 2 (#7807d2)",
    "U 3 (#a77fa3)",
    "L 2 (#015232)",
    "U 2 (#7a21e3)",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day18.test,
      Expect = 62
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day18/input.txt").Trim(),
      Expect = 47045
    },
    // Part #2, Test
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day18.test,
      Expect = 952408144115
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day18/input.txt").Trim(),
      Expect = 147839570293376
    }
  };
}
