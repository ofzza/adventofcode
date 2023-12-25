namespace ofzza.aoc.year2023.day25;

using ofzza.aoc.utils;

public partial class Day25: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 25;
  
  private static string test = string.Join('\n', new string[] {
    "jqt: rhn xhk nvd",
    "rsh: frs pzl lsr",
    "xhk: hfx",
    "cmg: qnr nvd lhk bvb",
    "rhn: xhk bvb hfx",
    "bvb: xhk hfx",
    "pzl: lsr hfx nvd",
    "qnr: nvd",
    "ntq: jqt hfx bvb xhk",
    "nvd: lhk",
    "lsr: lhk",
    "rzs: qnr cmg lsr rsh",
    "frs: qnr lhk lsr",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day25.test,
      Expect = 54
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day25/input.txt").Trim(),
      Expect = 552695
    }
  };
}
