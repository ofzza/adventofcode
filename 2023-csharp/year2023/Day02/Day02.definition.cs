namespace ofzza.aoc.year2023.day02;

using ofzza.aoc.utils;

public partial class Day02: ISolution<string[], int> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 2;
  
  private static string[] test = new string[] {
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
  };
  public List<SolutionExecution<string[], int>> Executions { get; } = new List<SolutionExecution<string[], int>>() {
    // Part #1, Test
    new SolutionExecution<string[], int>(1, Tag.Test) {
      InputValue = Day02.test,
      Expect = 8
    },
    // Part #1, Solution
    new SolutionExecution<string[], int>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day02/input.txt").Trim().Split("\n"),
      Expect = 2204
    },
    // Part #2, Test
    new SolutionExecution<string[], int>(2, Tag.Test) {
      InputValue = Day02.test,
      Expect = 2286
    },
    // Part #2, Solution
    new SolutionExecution<string[], int>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day02/input.txt").Trim().Split("\n"),
      Expect = 71036
    }
  };
}
