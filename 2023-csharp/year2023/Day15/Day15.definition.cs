
namespace ofzza.aoc.year2023.day15;

using ofzza.aoc.utils;

public partial class Day15: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 15;
  
  private static string testA = string.Join('\n', new string[] {
    "HASH",
  });
  private static string testB = string.Join('\n', new string[] {
    "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day15.testA,
      Expect = 52
    },
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day15.testB,
      Expect = 1320
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day15/input.txt").Trim(),
      Expect = 510801
    },
    // Part #2, Test
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day15.testB,
      Expect = 145
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day15/input.txt").Trim(),
      Expect = 212763
    }
  };
}
