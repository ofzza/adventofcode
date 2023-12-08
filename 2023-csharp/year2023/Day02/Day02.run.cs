namespace ofzza.aoc.year2023.day02;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day02: ISolution<string[], int> {
  public int Run(SolutionExecutionRunInfo<string[]> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // First
    if (info.ExecutionIndex == 1) {
      var sum = 0;
      var limits = new int[] {
        input.Limits.First(limit => limit.Color == CubeColor.Red)!.Count,
        input.Limits.First(limit => limit.Color == CubeColor.Green)!.Count,
        input.Limits.First(limit => limit.Color == CubeColor.Blue)!.Count,
      };
      for (var i=0; i<input.Games.Count; i++) {
        var game = input.Games[i];
        var possibleGame = true;
        foreach (var round in game.Rounds) {
          var possibleRound = true;
          foreach (var result in round.Results) {
            if (result.Count > limits[(int)result.Color]) {
              possibleRound = false;
              break;
            }
          }
          if (!possibleRound) { possibleGame = false; break; }
        }
        if (possibleGame) { sum += game.Index; }
        log.Progress(i, input.Games.Count);
      }
      return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
        var sum = 0;
        for (var i=0; i<input.Games.Count; i++) {
          var game = input.Games[i];
          var limits = new int[] { 0, 0, 0 };
          foreach (var round in game.Rounds) {
            foreach (var result in round.Results) {
              if (result.Count > limits[(int)result.Color]) {
                limits[(int)result.Color] = result.Count;
              }
            }
          }
          sum += limits[0] * limits[1] * limits[2];
          log.Progress(i, input.Games.Count);
        }
        return sum;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
