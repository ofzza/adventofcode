namespace ofzza.aoc.year2023.day06;

using System.Linq;
using ofzza.aoc.utils;

public partial class Day06: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var parsed = parse(info.InputValue!);
    // First
    if (info.ExecutionIndex == 1) {        
        // D = T[wait] * (T[total] - T[wait]) > D[max]
        // D = T[wait] * T[total] - T[wait]^2 > D[max]
        // T[wait]^2 - T[total]T[wait] + D[max] < 0
        // T[wait](D == D[max]) = ( T[total] +/- sqrt(T[total]^2 - 4 * D[max]) ) / 2
        var sum = 1;
        for (var i=0; i<parsed.Races.Length; i++) {
          var race = parsed.Races[i];
          var t1 = (int)Math.Ceiling((race.TotalTime - (double)Math.Sqrt((double)Math.Pow(race.TotalTime, 2) - 4 * (double)race.MaxDistance)) / 2);
          var d1 = t1 * (race.TotalTime - t1);
          if (d1 == race.MaxDistance) t1 += 1;
          var t2 = (int)Math.Floor((race.TotalTime + (double)Math.Sqrt((double)Math.Pow(race.TotalTime, 2) - 4 * (double)race.MaxDistance)) / 2);
          var d2 = t2 * (race.TotalTime - t2);
          if (d2 == race.MaxDistance) t2 -= 1;
          log.WriteLine($"""- Time = {race.TotalTime}, Distance = {race.MaxDistance}: {t1} - {t2} -> {t2 - t1 + 1}""");
          log.Progress(i, parsed.Races.Length);
          sum *= t2 - t1 + 1;
        }
        return sum;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
        var race = new Race() {
          TotalTime = long.Parse(string.Join("", parsed.Races.Select(r => r.TotalTime.ToString()))),
          MaxDistance = long.Parse(string.Join("", parsed.Races.Select(r => r.MaxDistance.ToString())))
        };
        var t1 = (int)Math.Ceiling((race.TotalTime - (double)Math.Sqrt((double)Math.Pow(race.TotalTime, 2) - 4 * (double)race.MaxDistance)) / 2);
        var d1 = t1 * (race.TotalTime - t1);
        if (d1 == race.MaxDistance) t1 += 1;
        var t2 = (int)Math.Floor((race.TotalTime + (double)Math.Sqrt((double)Math.Pow(race.TotalTime, 2) - 4 * (double)race.MaxDistance)) / 2);
        var d2 = t2 * (race.TotalTime - t2);
        if (d2 == race.MaxDistance) t2 -= 1;
        log.WriteLine($"""- Time = {race.TotalTime}, Distance = {race.MaxDistance}: {t1} - {t2} -> {t2 - t1 + 1}""");
        return t2 - t1 + 1;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
