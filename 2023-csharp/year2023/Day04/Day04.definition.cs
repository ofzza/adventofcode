namespace ofzza.aoc.year2023.day04;

using ofzza.aoc.utils;

public partial class Day04: ISolution<string[], int> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 4;
  
  private static string[] test = new string[] {
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
    "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
    "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
    "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
    "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
    "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
  };
  public List<SolutionExecution<string[], int>> Executions { get; } = new List<SolutionExecution<string[], int>>() {
    // Part #1, Test
    new SolutionExecution<string[], int>(1, Tag.Test) {
      InputValue = Day04.test,
      Expect = 13
    },
    // Part #1, Solution
    new SolutionExecution<string[], int>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day04/input.txt").Trim().Split("\n"),
      Expect = 33950
    },
    // Part #2, Test
    new SolutionExecution<string[], int>(2, Tag.Test) {
      InputValue = Day04.test,
      Expect = 30
    },
    // Part #2, Solution
    new SolutionExecution<string[], int>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day04/input.txt").Trim().Split("\n"),
      Expect = 14814534
    }
  };
}
