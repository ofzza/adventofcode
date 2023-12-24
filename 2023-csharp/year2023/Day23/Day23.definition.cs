namespace ofzza.aoc.year2023.day23;

using ofzza.aoc.utils;

public partial class Day23: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 23;
  
  private static string test = string.Join('\n', new string[] {
    "#.#####################",
    "#.......#########...###",
    "#######.#########.#.###",
    "###.....#.>.>.###.#.###",
    "###v#####.#v#.###.#.###",
    "###.>...#.#.#.....#...#",
    "###v###.#.#.#########.#",
    "###...#.#.#.......#...#",
    "#####.#.#.#######.#.###",
    "#.....#.#.#.......#...#",
    "#.#####.#.#.#########v#",
    "#.#...#...#...###...>.#",
    "#.#.#v#######v###.###v#",
    "#...#.>.#...>.>.#.###.#",
    "#####v#.#.###v#.#.###.#",
    "#.....#...#...#.#.#...#",
    "#.#########.###.#.#.###",
    "#...###...#...#...#.###",
    "###.###.#.###v#####v###",
    "#...#...#.#.>.>.#.>.###",
    "#.###.###.#.###.#.#v###",
    "#.....###...###...#...#",
    "#####################.#",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day23.test,
      Expect = 94
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day23/input.txt").Trim(),
      Expect = 2174
    },
    // Part #2, Test
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day23.test,
      Expect = 154
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day23/input.txt").Trim(),
      Expect = null
    }
  };
}
