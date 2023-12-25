namespace ofzza.aoc.year2023.day24;

using ofzza.aoc.utils;

public partial class Day24: ISolution<(double[] Min, double[] Max, string Input), long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 24;
  
  private static string test = string.Join('\n', new string[] {
    "19, 13, 30 @ -2,  1, -2",
    "18, 19, 22 @ -1, -1, -2",
    "20, 25, 34 @ -2, -2, -4",
    "12, 31, 28 @ -1, -2, -1",
    "20, 19, 15 @  1, -5, -3",
  });
  public List<SolutionExecution<(double[] Min, double[] Max, string Input), long>> Executions { get; } = new List<SolutionExecution<(double[] Min, double[] Max, string Input), long>>() {
    // Part #1, Test
    new SolutionExecution<(double[] Min, double[] Max, string Input), long>(1, Tag.Test) {
      InputValue = (
        new double[] { 7, 7, 7 },
        new double[] { 27, 27, 27 },
        Day24.test
      ),
      Expect = 2
    },
    // Part #1, Solution
    new SolutionExecution<(double[] Min, double[] Max, string Input), long>(1, Tag.Solution) {
      InputValue = (
        new double[] { 200000000000000, 200000000000000, 200000000000000 },
        new double[] { 400000000000000, 400000000000000, 400000000000000},
        File.ReadAllText("./inputs/Day24/input.txt").Trim()
      ),
      Expect = 13754
    },
    // Part #2, Test
    new SolutionExecution<(double[] Min, double[] Max, string Input), long>(2, Tag.Test) {
      InputValue = (
        new double[] {},
        new double[] {},
        Day24.test
      ),
      Expect = 47
    },
    // Part #2, Solution
    new SolutionExecution<(double[] Min, double[] Max, string Input), long>(2, Tag.Solution) {
      InputValue = (
        new double[] {},
        new double[] {},
        File.ReadAllText("./inputs/Day24/input.txt").Trim()
      ),
      Expect = null
    }
  };
}
