namespace ofzza.aoc.year2023.day11;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day11: ISolution<(long Expansion, string Space), long> {
  private static Input parse (string input) {
    // Find coordinates and empty rows and columns
    var parsed = input.Split('\n').Select(l => l.ToCharArray()).ToArray();
    var coordinates = new List<long[]>();
    for (var y=0; y<parsed.Length; y++) {
      for (var x=0; x<parsed[0].Length; x++) {
        if (parsed[y][x] == '#') coordinates.Add(new long[] { x, y });
      }
    }

    // Return parsed input
    return new Input() {
      Size = new long[] { parsed[0].Length, parsed.Length },
      Coordinates = coordinates.ToArray()
    };
  }
}
