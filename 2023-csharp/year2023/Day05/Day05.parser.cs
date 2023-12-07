namespace ofzza.aoc.year2023.day05;

using System.Linq;
using ofzza.aoc.utils;
using ofzza.aoc.utils.interval;

public partial class Day05: ISolution<string, long> {
  private static Input parse (string input) {
    var parsed = input.Split("\n\n");
    var seedIds = parsed[0].Trim().Split(':')[1].Trim().Split(' ').Select(n => long.Parse(n)).ToArray();
    var seedIntervals = new List<Interval<long>>();
    for (var i=0; i<seedIds.Length / 2; i++) seedIntervals.Add(new Interval<long>() { Start = seedIds[2 * i], End = seedIds[2 * i] + seedIds[2 * i + 1] - 1 });
    return new Input() {
      SeedIds = seedIds,
      SeedIntervals = seedIntervals.ToArray(),
      Mappings = parsed.ToList().GetRange(1, parsed.Length - 1).Select(m => {
        var parsed = m.Split('\n');
        var keys = parsed[0].Split("map:")[0].Trim().Split("-to-");
        return new KeyMapping() {
          SourceKey = keys[0],
          DestinationKey = keys[1],
          Mappings = parsed.ToList().GetRange(1, parsed.Length - 1).Select(m => {
            var parsed = m.Split(' ').Select(n => long.Parse(n)).ToArray();
            return new IdMapping() {
              SourceInterval = new Interval<long>() { Start = parsed[1], End = parsed[1] + parsed[2] - 1 },
              DestinationInterval = new Interval<long>() { Start = parsed[0], End = parsed[0] + parsed[2] - 1 }
            };
          }).ToArray()
        };
      }).ToArray()
    };
  }
}
