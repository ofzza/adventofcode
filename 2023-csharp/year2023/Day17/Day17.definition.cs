namespace ofzza.aoc.year2023.day17;

using ofzza.aoc.utils;

public partial class Day17: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 17;
  
  private static string testA = string.Join('\n', new string[] {
    "2413432311323",
    "3215453535623",
    "3255245654254",
    "3446585845452",
    "4546657867536",
    "1438598798454",
    "4457876987766",
    "3637877979653",
    "4654967986887",
    "4564679986453",
    "1224686865563",
    "2546548887735",
    "4322674655533",
  });
  private static string testB = string.Join('\n', new string[] {
    "111111111111",
    "999999999991",
    "999999999991",
    "999999999991",
    "999999999991",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day17.testA,
      Expect = 102
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day17/input.txt").Trim(),
      Expect = 1128
    },
    // Part #2, Test
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day17.testA,
      Expect = 94
    },
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day17.testB,
      Expect = 71
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day17/input.txt").Trim(),
      Expect = null
    }
  };
}
