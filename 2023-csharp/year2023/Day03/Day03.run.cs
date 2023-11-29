namespace ofzza.aoc.year2023.day03;

using System.Linq;
using ofzza.aoc;
using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

public partial class Day03: ISolution<string[], int> {
  public int Run(SolutionExecutionRunInfo<string[]> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var values = parse(info.InputValue!);
    var indexer = new MatrixIndexer(new[] { info.InputValue!.Length, info.InputValue![0].Length });
    // First
    if (info.ExecutionIndex == 1) {
        // Find all serial numbers adjecent to parts
        var partAdjecentSerialNumbers = new HashSet<PartSerialNumber>();
        for (var i=0; i<indexer.Length; i++) {
          if (values[i].Type == ValueType.Part) {
            var neighbors = indexer.GetNeighboringIndices(i, true);
            foreach (var neighbor in neighbors) {
              if (values[neighbor].Type == ValueType.SerialNumber) {
                partAdjecentSerialNumbers.Add((PartSerialNumber)values[neighbor].SerialNumber!);
              }
            }
          }
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
          if (values[i].Type == ValueType.Part && values[i].Part == '*') {
            var neighbors = indexer.GetNeighboringIndices(i, true);
            var neighborsSerialNumbers = new List<PartSerialNumber>();
            var hash = new HashSet<PartSerialNumber>();
            foreach (var neighbor in neighbors) {
              if (values[neighbor].Type == ValueType.SerialNumber) {
                var serialNumer = (PartSerialNumber)values[neighbor].SerialNumber!;
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
