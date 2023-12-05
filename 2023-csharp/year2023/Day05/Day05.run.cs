namespace ofzza.aoc.year2023.day05;

using System.Linq;
using ofzza.aoc.utils;
using ofzza.aoc.utils.range;

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
              if (idMapping.SourceRange.Start <= id && (idMapping.SourceRange.End >= id)) {
                mappings[seedId].Add(id + (idMapping.DestinationRange.Start - idMapping.SourceRange.Start));
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
        var ranges = new List<Range<long>>(parsed.SeedRanges);
        for (var i=0; i<parsed.Mappings.Length; i++) {
          var keyMapping = parsed.Mappings[i];
          var convertedRanges = new List<Range<long>>();
          foreach (var idMapping in keyMapping.Mappings) {
            var remainingRanges = new List<Range<long>>();
            foreach (var range in ranges) {
              var overlap = Range<long>.GetOverlap(range, idMapping.SourceRange);
              if (overlap.Overlap == null) {
                remainingRanges.Add(new Range<long>() { Start = range.Start, End = range.End });
              }
              else {
                var offset = idMapping.DestinationRange.Start - idMapping.SourceRange.Start;
                convertedRanges.Add(new Range<long>() {
                  Start = overlap.Overlap.Start + offset,
                  End = overlap.Overlap.End + offset
                });
                foreach (var remainder in overlap.Remainders.Item1) {
                  remainingRanges.Add(new Range<long>() { Start = remainder.Start, End = remainder.End });
                }
              }
            }
            ranges = remainingRanges;
          }
          ranges.AddRange(convertedRanges);
          log.Progress(i, parsed.Mappings.Length);
        }
        return ranges.ToList().Min(m => m.Start);
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
