namespace ofzza.aoc.year2023.day20;

using ofzza.aoc.utils;
using ofzza.aoc.utils.primes;
using ofzza.aoc.year2023.utils.pulspropagation;

public partial class Day20: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // Initialize pulse propagation
    var pulse = new PulsePropagation(input);
    // First
    if (info.ExecutionIndex == 1) {
      // Initialize signals count
      long high = 0;
      long low = 0;
      // Trigger initial signal 1000 times in a row
      for (var i=0; i<1000; i++) {
        // Trigger initial signal
        var signals = pulse.TriggerSignal("broadcaster", SignalType.Low, log);
        // Count and return signals triggered
        var h = signals.Where(s => s.Type == SignalType.High).Count();
        high += h;
        low += signals.Length - h;
      }
      // Return signals count
      return high * low;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Initialize cycle tracking for relevant output modules
      var count = 0;
      long lcm = 0;
      var names = new string[] { "lk", "fn", "fh", "hh", "nc", "rx" };
      var states = new bool[] { false, false, false, false, false, false };
      var last = new int[] { 0, 0, 0, 0, 0, 0 };
      var cycle = new int[] { 0, 0, 0, 0, 0, 0 };
      var offset = new int[] { 0, 0, 0, 0, 0, 0 };
      // Track cycles for relevant output modules
      pulse.OnNewSignal += s => {
        // Track signal changes
        var index = Array.IndexOf(names, s.Source?.Name);
        if (index != -1) {
          var state = s.Type == SignalType.High;
          // Track when any signal changed compared to last state
          if (state != states[index]) {
            // Store state
            states[index] = state;
            // Log
            var logLine = "";
            for (var i=0; i<names.Length; i++) {
              var stateStr = states[i] ? "1" : "0";
              var cycleStr = cycle[i].ToString();
              var offsetStr = offset[i].ToString();
              logLine += $"""{names[i]}={stateStr} ({cycleStr}|{offsetStr}) | """;
            }
            log.WriteLine($"""> {count.ToString()} : {logLine}""");
            // Keep track of cycles per signal
            if (state) {
              cycle[index] = count - last[index];
              offset[index] = count % cycle[index];
              last[index] = count;
            }
            // Once all cycles known, find least common multiple
            if (cycle[0] != 0 && cycle[1] != 0 && cycle[2] != 0 && cycle[3] != 0) {
              lcm = Primes.GetLeastCommonMultiple(new long[] { cycle[0], cycle[1], cycle[2], cycle[3] });              
            }
          }
        }
      };
      // Trigger initial signal for as long as required to get the "rx" module to receive a low pulse
      while (lcm == 0) {
        // Count triggered initial signal
        count++;
        // Trigger initial signal
        var signals = pulse.TriggerSignal("broadcaster", SignalType.Low, log, ConsoleLoggingLevel.All);
      }
      // Return count of initial signal dispatches
      return lcm;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
