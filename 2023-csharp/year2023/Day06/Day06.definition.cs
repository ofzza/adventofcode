namespace ofzza.aoc.year2023.day06;

using ofzza.aoc.utils;

public partial class Day06: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 6;
  
  private static string test = string.Join('\n', new string[] {
    "Time:      7  15   30",
    "Distance:  9  40  200",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day06.test,
      Expect = 288
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day06/input.txt").Trim(),
      Expect = 861300
    },
    // Part #2, Test
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day06.test,
      Expect = 71503
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day06/input.txt").Trim(),
      Expect = 28101347
    }
  };
}
