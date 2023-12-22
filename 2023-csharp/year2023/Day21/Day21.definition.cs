namespace ofzza.aoc.year2023.day21;

using ofzza.aoc.utils;

public partial class Day21: ISolution<(int Count, string Tiles), long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 21;
  
  private static string test = string.Join('\n', new string[] {
    "...........",
    ".....###.#.",
    ".###.##..#.",
    "..#.#...#..",
    "....#.#....",
    ".##..S####.",
    ".##..#...#.",
    ".......##..",
    ".##.#.####.",
    ".##..##.##.",
    "...........",
  });
  public List<SolutionExecution<(int Count, string Tiles), long>> Executions { get; } = new List<SolutionExecution<(int Count, string Tiles), long>>() {
    // Part #1, Test
    new SolutionExecution<(int Count, string Tiles), long>(1, Tag.Test) {
      InputValue = (6, Day21.test),
      Expect = 16
    },
    // Part #1, Solution
    new SolutionExecution<(int Count, string Tiles), long>(1, Tag.Solution) {
      InputValue = (64, File.ReadAllText("./inputs/Day21/input.txt").Trim()),
      Expect = 3795
    },
    // Part #2, Test
    new SolutionExecution<(int Count, string Tiles), long>(2, Tag.Test) {
      InputValue = (6, Day21.test),
      Expect = 16
    },
    new SolutionExecution<(int Count, string Tiles), long>(2, Tag.Test) {
      InputValue = (10, Day21.test),
      Expect = 50
    },
    new SolutionExecution<(int Count, string Tiles), long>(2, Tag.Test) {
      InputValue = (50, Day21.test),
      Expect = 1594
    },
    new SolutionExecution<(int Count, string Tiles), long>(2, Tag.Test) {
      InputValue = (100, Day21.test),
      Expect = 6536
    },
    new SolutionExecution<(int Count, string Tiles), long>(2, Tag.Test) {
      InputValue = (500, Day21.test),
      Expect = 167004
    },
    new SolutionExecution<(int Count, string Tiles), long>(2, Tag.Test) {
      InputValue = (1000, Day21.test),
      Expect = 668697
    },
    new SolutionExecution<(int Count, string Tiles), long>(2, Tag.Test) {
      InputValue = (5000, Day21.test),
      Expect = 16733044
    },
    // Part #2, Solution
    new SolutionExecution<(int Count, string Tiles), long>(2, Tag.Solution) {
      InputValue = (26501365, File.ReadAllText("./inputs/Day21/input.txt").Trim()),
      Expect = null
    }
  };
}
