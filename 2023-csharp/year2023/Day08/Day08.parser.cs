namespace ofzza.aoc.year2023.day08;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day08: ISolution<string, long> {
  private static Input parse (string input, bool jokers = false) {
    Node.ClearNodes();
    var parsed = input.Split("\n\n");
    return new Input() {
      Directions = parsed[0].ToCharArray().Select(d => d == 'L' ? Direction.Left : Direction.Right).ToArray(),
      Nodes = parsed[1].Split('\n').Select(n => {
        var parsed = n.Split('=');
        var id = parsed[0].Trim();
        var right = parsed[1].Trim();
        var nodes = right.Substring(1, right.Length - 2).Split(',').Select(n => n.Trim()).ToArray();
        return new Node() {
          Id = id,
          LeftNodeId = nodes[0],
          RightNodeId = nodes[1]
        };
      }).ToArray()
    };
  }
}
