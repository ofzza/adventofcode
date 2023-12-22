namespace ofzza.aoc.year2023.day06;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day06: ISolution<string, long> {
  private static Input parse (string input) {
    var parsed = input.Split('\n');
    var times = parsed[0].Split(':')[1].Trim().Split(' ').Where(t => t.Length > 0).Select(t => int.Parse(t)).ToArray();
    var distances = parsed[1].Split(':')[1].Trim().Split(' ').Where(t => t.Length > 0).Select(d => int.Parse(d)).ToArray();
    var races = new List<Race>();
    for (var i=0; i<times.Length; i++) {
      races.Add(new Race() { TotalTime = times[i], MaxDistance = distances[i] });
    }
    return new Input() { Races = races.ToArray() };
  }
}
