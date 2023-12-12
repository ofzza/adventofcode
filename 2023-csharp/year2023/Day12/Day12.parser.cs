namespace ofzza.aoc.year2023.day12;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day12: ISolution<string, long> {
  private static (string Springs, int[] Checksum)[] parse (string input) {
    var lines = input.Split('\n');
    return lines.Select(l => {
      var parsed = l.Split(' ');
      var springs = parsed[0].Trim();
      var checksum = parsed[1].Trim().Split(',').Select(n => int.Parse(n)).ToArray();    
      return (springs, checksum);
    }).ToArray();
  }
}
