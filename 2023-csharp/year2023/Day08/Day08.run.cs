namespace ofzza.aoc.year2023.day08;

using System.Linq;
using ofzza.aoc.utils;
using ofzza.aoc.utils.primes;

public partial class Day08: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // First
    if (info.ExecutionIndex == 1) {        
      // Initialize
      var count = 0;
      var pointer = 0;
      var node = Node.Get("AAA")!;
      // Find path
      while (node.Id != "ZZZ") {
        // Move to next node
        var direction = input.Directions[pointer % input.Directions.Length];
        var next = direction == Direction.Left ? node.LeftNode : node.RightNode;
        count++;
        pointer++;
        // Log movement
        log.WriteLine($"""- {count}. {node.Id} ({node.LeftNodeId}, {node.RightNodeId}): {direction} -> {next.Id}""");
        // Apply movement
        node = next;
      }
      return count;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      // Initialize
      var nodes = Node.GetAll().Where(n => n.Id.EndsWith('A')).ToArray();
      var counts = new long[nodes.Length];
      log.WriteLine("> Nodes: {0:N0}", nodes.Length);
      // Find paths
      for (var i=0; i<nodes.Length; i++) {
        var count = 0;
        var pointer = 0;
        var node = nodes[i];
        while (!node.Id.EndsWith('Z')) {
          // Move to next node
          var direction = input.Directions[pointer % input.Directions.Length];
          var next = direction == Direction.Left ? node.LeftNode : node.RightNode;
          count++;
          pointer++;
          // Log movement
          log.WriteLine($"""    - {count}. {node.Id} ({node.LeftNodeId}, {node.RightNodeId}): {direction} -> {next.Id}""", ConsoleLoggingLevel.All);
          // Apply movement
          node = next;
        }
        log.WriteLine($"""  = {i}. -> {count} -> {string.Join(", ", Primes.GetFactors(count).Select(n => n.ToString()))}""");
        counts[i] = count;        
      }
      // Calculate least common multiple of all counts
      var leastCommonMultiple = Primes.GetLeastCommonMultiple(counts);
      log.WriteLine($"""> Least common multiple: {leastCommonMultiple}""");
      return leastCommonMultiple;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
