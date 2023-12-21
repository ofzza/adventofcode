namespace ofzza.aoc.year2023.day20;

using ofzza.aoc.utils;

public partial class Day20: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 20;
  
  private static string testA = string.Join('\n', new string[] {
    "broadcaster -> a, b, c",
    "%a -> b",
    "%b -> c",
    "%c -> inv",
    "&inv -> a",
  });
  private static string testB = string.Join('\n', new string[] {
    "broadcaster -> a",
    "%a -> inv, con",
    "&inv -> b",
    "%b -> con",
    "&con -> output",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day20.testA,
      Expect = 32000000
    },
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day20.testB,
      Expect = 11687500
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day20/input.txt").Trim(),
      Expect = 1020211150
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day20/input.txt").Trim(),
      Expect = 238815727638557
    }
  };
}
