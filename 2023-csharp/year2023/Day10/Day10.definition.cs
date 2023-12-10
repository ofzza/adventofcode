namespace ofzza.aoc.year2023.day10;

using ofzza.aoc.utils;

public partial class Day10: ISolution<string, long> {
  public int SolutionYear { get; } = 2023;

  public int SolutionDay { get; } = 10;
  
  private static string testA = string.Join('\n', new string[] {
    ".....",
    ".S-7.",
    ".|.|.",
    ".L-J.",
    ".....",
  });
  private static string testB = string.Join('\n', new string[] {
    "..F7.",
    ".FJ|.",
    "SJ.L7",
    "|F--J",
    "LJ...",
  });
  private static string testC = string.Join('\n', new string[] {
    "...........",
    ".S-------7.",
    ".|F-----7|.",
    ".||.....||.",
    ".||.....||.",
    ".|L-7.F-J|.",
    ".|..|.|..|.",
    ".L--J.L--J.",
    "...........",
  });
  private static string testD = string.Join('\n', new string[] {
    ".F----7F7F7F7F-7....",
    ".|F--7||||||||FJ....",
    ".||.FJ||||||||L7....",
    "FJL7L7LJLJ||LJ.L-7..",
    "L--J.L7...LJS7F-7L7.",
    "....F-J..F7FJ|L7L7L7",
    "....L7.F7||L7|.L7L7|",
    ".....|FJLJ|FJ|F7|.LJ",
    "....FJL-7.||.||||...",
    "....L---J.LJ.LJLJ...",
  });
  private static string testE = string.Join('\n', new string[] {
    "FF7FSF7F7F7F7F7F---7",
    "L|LJ||||||||||||F--J",
    "FL-7LJLJ||||||LJL-77",
    "F--JF--7||LJLJ7F7FJ-",
    "L---JF-JLJ.||-FJLJJ7",
    "|F|F-JF---7F7-L7L|7|",
    "|FFJF7L7F-JF7|JL---7",
    "7-L-JL7||F7|L7F-7F7|",
    "L.L7LFJ|||||FJL7||LJ",
    "L7JLJL-JLJLJL--JLJ.L",
  });
  public List<SolutionExecution<string, long>> Executions { get; } = new List<SolutionExecution<string, long>>() {
    // Part #1, Test
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day10.testA,
      Expect = 4
    },
    new SolutionExecution<string, long>(1, Tag.Test) {
      InputValue = Day10.testB,
      Expect = 8
    },
    // Part #1, Solution
    new SolutionExecution<string, long>(1, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day10/input.txt").Trim(),
      Expect = 6690
    },
    // Part #2, Test
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day10.testC,
      Expect = 4
    },
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day10.testD,
      Expect = 8
    },
    new SolutionExecution<string, long>(2, Tag.Test) {
      InputValue = Day10.testE,
      Expect = 10
    },
    // Part #2, Solution
    new SolutionExecution<string, long>(2, Tag.Solution) {
      InputValue = File.ReadAllText("./inputs/Day10/input.txt").Trim(),
      Expect = 525
    }
  };
}
