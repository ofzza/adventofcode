namespace ofzza.aoc.year2023.day22;

using ofzza.aoc.utils;

public partial class Day22: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 22;
  
  private static string test = string.Join('\n', new string[] {
    "1,0,1~1,2,1",
    "0,0,2~2,0,2",
    "0,2,3~2,2,3",
    "0,0,4~0,2,4",
    "2,0,5~2,2,5",
    "0,1,6~2,1,6",
    "1,1,8~1,1,9",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day22.test,
      Expect = 5
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day22/input.txt").Trim(),
      Expect = 416
    },
    // Part #2, Test
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day22.test,
      Expect = 7
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day22/input.txt").Trim(),
      Expect = 60963
    }
  };
}
