namespace ofzza.aoc.year2023.day05;

using System.Linq;
using ofzza.aoc.utils;
using ofzza.aoc.utils.interval;

public partial class Day05: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var parsed = parse(info.InputValue!);
    // First
    if (info.ExecutionIndex == 1) {        
        var mappings = new Dictionary<long, List<long>>();
        for (var i=0; i<parsed.SeedIds.Length; i++) {
          var seedId = parsed.SeedIds[i];
          mappings[seedId] = new List<long>() { seedId };
          for (var j=0; j<parsed.Mappings.Length; j++) {
            var keyMapping = parsed.Mappings[j];
            var mapped = false;
            var id = mappings[seedId].Last();
            foreach (var idMapping in keyMapping.Mappings) {
              if (idMapping.SourceInterval.Start <= id && (idMapping.SourceInterval.End >= id)) {
                mappings[seedId].Add(id + (idMapping.DestinationInterval.Start - idMapping.SourceInterval.Start));
                mapped = true;
                break;
              }
            }
            if (!mapped) {
              mappings[seedId].Add(id);
            }
          }
          log.Progress(i, parsed.SeedIds.Length);
        }
        return mappings.Values.Min(m => m.Last());
    }
    // Second
    else if (info.ExecutionIndex == 2) {
        var intervals = new List<Interval<long>>(parsed.SeedIntervals);
        for (var i=0; i<parsed.Mappings.Length; i++) {
          var keyMapping = parsed.Mappings[i];
          var convertedIntervals = new List<Interval<long>>();
          foreach (var idMapping in keyMapping.Mappings) {
            var remainingIntervals = new List<Interval<long>>();
            foreach (var interval in intervals) {
              var overlap = Interval<long>.GetOverlap(interval, idMapping.SourceInterval);
              if (overlap.Overlap == null) {
                remainingIntervals.Add(new Interval<long>() { Start = interval.Start, End = interval.End });
              }
              else {
                var offset = idMapping.DestinationInterval.Start - idMapping.SourceInterval.Start;
                convertedIntervals.Add(new Interval<long>() {
                  Start = overlap.Overlap.Start + offset,
                  End = overlap.Overlap.End + offset
                });
                foreach (var remainder in overlap.Remainders.FirstRemainder) {
                  remainingIntervals.Add(new Interval<long>() { Start = remainder.Start, End = remainder.End });
                }
              }
            }
            intervals = remainingIntervals;
          }
          intervals.AddRange(convertedIntervals);
          log.Progress(i, parsed.Mappings.Length);
        }
        return intervals.ToList().Min(m => m.Start);
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
