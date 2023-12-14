namespace ofzza.aoc.year2023.day14;

using ofzza.aoc.utils;

public partial class Day14: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 14;
  
  private static string test = string.Join('\n', new string[] {
    "O....#....",
    "O.OO#....#",
    ".....##...",
    "OO.#O....O",
    ".O.....O#.",
    "O.#..O.#.#",
    "..O..#O..O",
    ".......O..",
    "#....###..",
    "#OO..#....",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day14.test,
      Expect = 136
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day14/input.txt").Trim(),
      Expect = 110407
    },
    // Part #2, Test
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day14.test,
      Expect = 64
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day14/input.txt").Trim(),
      Expect = 87273
    }
  };
}
