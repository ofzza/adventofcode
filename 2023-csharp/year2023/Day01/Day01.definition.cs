namespace ofzza.aoc.year2023.day01;

using ofzza.aoc.utils;

public partial class Day01: ISolution<string[], int> {
    public int SolutionYear { get; } = 2023;

    public int SolutionDay { get; } = 1;
    public List<SolutionExecution<string[], int>> Executions { get; } = new List<SolutionExecution<string[], int>>() {
    // Part #1, Test
    new SolutionExecution<string[], int>(1, Tag.Test) {
      InputValue = new string[] { "1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet" },
      Expect = 142
    },
    // Part #1, Solution
    new SolutionExecution<string[], int>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day01/input.txt").Trim().Split("\n"),
      Expect = 54951
    },
    // Part #2, Test
    new SolutionExecution<string[], int>(2, Tag.Test) {
      InputValue = new string[] { "two1nine", "eightwothree", "abcone2threexyz", "xtwone3four", "4nineeightseven2", "zoneight234", "7pqrstsixteen" },
      Expect = 281
    },
    // Part #2, Solution
    new SolutionExecution<string[], int>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day01/input.txt").Trim().Split("\n"),
      Expect = 55218
    }
  };
}
