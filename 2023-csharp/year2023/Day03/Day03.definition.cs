namespace ofzza.aoc.year2023.day03;

using ofzza.aoc.utils;

public partial class Day03: ISolution<string[], long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 3;
  
  private static string[] test = new string[] {
    "467..114..",
    "...*......",
    "..35..633.",
    "......#...",
    "617*......",
    ".....+.58.",
    "..592.....",
    "......755.",
    "...$.*....",
    ".664.598..",
  };
  public List<SolutionExecution<string[], long>> Executions { get; } = new List<SolutionExecution<string[], long>>() {
    // Part #1, Test
    new SolutionExecution<string[], long>(1, Tag.Test) {
      InputValue = Day03.test,
      Expect = 4361
    },
    // Part #1, Solution
    new SolutionExecution<string[], long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day03/input.txt").Trim().Split("\n"),
      Expect = 527369
    },
    // Part #2, Test
    new SolutionExecution<string[], long>(2, Tag.Test) {
      InputValue = Day03.test,
      Expect = 467835
    },
    // Part #2, Solution
    new SolutionExecution<string[], long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day03/input.txt").Trim().Split("\n"),
      Expect = 73074886
    }
  };
}
