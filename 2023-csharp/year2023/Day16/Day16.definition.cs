namespace ofzza.aoc.year2023.day16;

using ofzza.aoc.utils;

public partial class Day16: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 16;
  
  private static string test = string.Join('\n', new string[] {
    $""".|...\....""",
    $"""|.-.\.....""",
    $""".....|-...""",
    $"""........|.""",
    $"""..........""",
    $""".........\""",
    $"""..../.\\..""",
    $""".-.-/..|..""",
    $""".|....-|.\""",
    $"""..//.|....""",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day16.test,
      Expect = 46
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day16/input.txt").Trim(),
      Expect = 7496
    },
    // Part #2, Test
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day16.test,
      Expect = 51
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day16/input.txt").Trim(),
      Expect = 7932
    }
  };
}
