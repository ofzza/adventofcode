namespace ofzza.aoc.year2023.day03;

using System.Linq;
using System.Text.RegularExpressions;
using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

public partial class Day03: ISolution<string[], long> {
  private static Value[] parse (string[] input) {
    // Initialize values for all indices
    var values = new Value[input.Length * input[0].Length];
    // Extract all serial numbers from all lines
    var serials = new List<List<PartSerialNumber>>();
    foreach (var line in input) {
      var numbers = Regex.Matches(line, "[0-9]+").Select(m => int.Parse(m.Value)).ToArray();
      var lineSerials = new List<PartSerialNumber>();
      foreach (var number in numbers) {
        var serial = new PartSerialNumber() { Number = number };
        lineSerials.Add(serial);
      }
      serials.Add(lineSerials);
    }
    // Process values for all grid indices
    var indexer = new MatrixIndexer(new long[] { input.Length, input[0].Length });
    for (var y=0; y<input.Length; y++) {
      var serialNumberReadyForNext = false;
      var serialNumberCurrentIndex = 0;
      for (var x=0; x<input[y].Length; x++) {
        var i = indexer.CoordinatesToIndex(new long[] { y, x });
        // Process empty cell
        if (input[y][x] == '.') {
          // Register cell
          values[i] = new Value() {
            Type = ValueType.Empty
          };
          // If serial number consumed, move onto next serial number
          if (serialNumberReadyForNext) {
            serialNumberCurrentIndex++;
            serialNumberReadyForNext = false;
          }
        }
        // Process serial number cell
        else if (Regex.IsMatch(input[y][x].ToString(), "[0-9]")) {
          // Register cell
          values[i] = new Value() {
            Type = ValueType.SerialNumber,
            SerialNumber = serials[y][serialNumberCurrentIndex]
          };
          // Mark serial number as consumed
          serialNumberReadyForNext = true;
        }
        // Process part cell
        else {
          // Register cell
          values[i] = new Value() {
            Type = ValueType.Part,
            Part = input[y][x]
          };
          // If serial number consumed, move onto next serial number
          if (serialNumberReadyForNext) {
            serialNumberCurrentIndex++;
            serialNumberReadyForNext = false;
          }
        }
      }
    }
    // Return values
    return values;
  }
}
