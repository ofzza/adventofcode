namespace ofzza.aoc.year2023.day01;

using System.Linq;
using System.Text.RegularExpressions;
using ofzza.aoc.utils;

public partial class Day01: ISolution<string[], int> {
    public int Run(SolutionExecutionRunInfo<string[]> info, Console log, bool verbose, bool obfuscate) {
    // First
    if (info.ExecutionIndex == 1) {
        var sum = 0;
        for (var i=0; i<info.InputValue!.Length; i++) {
            var line = info.InputValue![i];
            if (line == null) continue;
            var match = Regex.Matches(line, "[1234567890]");
            if (match.Count > 0) {
                // Process line number
                var first = match.First();
                var last = match.Last();
                var lineNumber = int.Parse($"""{first.Value}{last.Value}""");
                sum += lineNumber;
                // Log line
                log.WriteLine($"""- '{line}': {lineNumber}""");
            }
            else {
                throw new Exception($"""Line doesn't contain digits: {line}""");
            }
            // Log progress
            log.Progress(i, info.InputValue!.Length);
        }
        return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
        var sum = 0;
        for (var i=0; i<info.InputValue!.Length; i++) {
            var line = info.InputValue![i];
            if (line == null) continue;
            var match = Regex.Matches(line, "(?=([1234567890]|one|two|three|four|five|six|seven|eight|nine))");
            if (match.Count > 0)
            {
                // Process line number
                var first = match.First().Groups[1];
                var last = match.Last().Groups[1];
                var lineNumber = int.Parse($"""{this._nameToDigit(first.Value)}{this._nameToDigit(last.Value)}""");
                sum += lineNumber;
                // Log line
                log.WriteLine($"""- '{line}': {lineNumber}""");
            }
            else
            {
                throw new Exception($"""Line doesn't contain digits: {line}""");
            }
            // Log progress
            log.Progress(i, info.InputValue!.Length);
        }
        return sum;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }

  private string _nameToDigit(string name) {
    switch (name) {
        case "one": return "1";
        case "two": return "2";
        case "three": return "3";
        case "four": return "4";
        case "five": return "5";
        case "six": return "6";
        case "seven": return "7";
        case "eight": return "8";
        case "nine": return "9";
        default: return name;
    }
  }
}
