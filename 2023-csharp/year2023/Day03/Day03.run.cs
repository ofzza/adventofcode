namespace ofzza.aoc.year2023.day03;

using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

public partial class Day03: ISolution<string[], long> {
  public long Run(SolutionExecutionRunInfo<string[]> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var parsed = parse(info.InputValue!);
    var indexer = new MatrixIndexer(new long[] { info.InputValue!.Length, info.InputValue![0].Length });
    // First
    if (info.ExecutionIndex == 1) {
        // Find all serial numbers adjecent to parts
        var partAdjecentSerialNumbers = new HashSet<PartSerialNumber>();
        for (var i=0; i<indexer.Length; i++) {
          if (parsed[i].Type == ValueType.Part) {
            var neighbors = indexer.GetNeighboringIndices(i, true);
            foreach (var neighbor in neighbors) {
              if (parsed[neighbor].Type == ValueType.SerialNumber) {
                partAdjecentSerialNumbers.Add((PartSerialNumber)parsed[neighbor].SerialNumber!);
              }
            }
          }
          log.Progress(i, indexer.Length);
        }
        
        // Sum up all serial numbers adjecent to parts
        var sum = 0;
        foreach (var seralNumber in partAdjecentSerialNumbers) {
          sum += seralNumber.Number;
        }
        return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
        // Find all serial numbers adjecent to parts
        var sum = 0;
        for (var i=0; i<indexer.Length; i++) {
          if (parsed[i].Type == ValueType.Part && parsed[i].Part == '*') {
            var neighbors = indexer.GetNeighboringIndices(i, true);
            var neighborsSerialNumbers = new List<PartSerialNumber>();
            var hash = new HashSet<PartSerialNumber>();
            foreach (var neighbor in neighbors) {
              if (parsed[neighbor].Type == ValueType.SerialNumber) {
                var serialNumer = (PartSerialNumber)parsed[neighbor].SerialNumber!;
                if (!hash.Contains(serialNumer)) {
                  neighborsSerialNumbers.Add(serialNumer);
                  hash.Add(serialNumer);
                }
              }
            }
            if (neighborsSerialNumbers.Count == 2) {
              sum += neighborsSerialNumbers[0].Number * neighborsSerialNumbers[1].Number;
            }
          }
          log.Progress(i, indexer.Length);
        }
        
        // Sum up all serial numbers adjecent to parts
        return sum;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
