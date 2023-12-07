namespace ofzza.aoc.year2023.day07;

using ofzza.aoc.utils;

public partial class Day07: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 7;
  
  private static string test = string.Join('\n', new string[] {
    "32T3K 765",
    "T55J5 684",
    "KK677 28",
    "KTJJT 220",
    "QQQJA 483",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day07.test,
      Expect = 6440
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day07/input.txt").Trim(),
      Expect = 249483956
    },
    // Part #2, Test
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day07.test,
      Expect = 5905
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day07/input.txt").Trim(),
      Expect = 252137472
    }
  };
}
