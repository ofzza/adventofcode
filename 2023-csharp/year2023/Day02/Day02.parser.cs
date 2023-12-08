namespace ofzza.aoc.year2023.day02;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day02: ISolution<string[], int> {
  private static Input parse (string[] input) {
    return new Input () {
      Limits = new() {
        new() { Color = CubeColor.Red, Count = 12 },
        new() { Color = CubeColor.Green, Count = 13 },
        new() { Color = CubeColor.Blue, Count = 14 }
      },
      Games = input.Select(game => {
        var parsed = game.Trim().Split(':');
        return new CubeGame() {
          Index = int.Parse(parsed[0].Split(' ')[1]),
          Rounds = parsed[1].Trim().Split(';').Select(round => {
            return new CubeGameRound() {
              Results = round.Split(',').Select(cube => {
                var parsed = cube.Trim().Split(' ');
                return new CubeGameRoundResult() {
                  Count = int.Parse(parsed[0]),
                  Color = (CubeColor)Enum.GetNames(typeof(CubeColor)).Select(name => name.ToLower()).ToList().IndexOf(parsed[1])!
                };
              }).ToList()
            };
          }).ToList()
        };
      }).ToList()
    };
  }
}
