namespace ofzza.aoc.year2023.day25;

using ofzza.aoc.utils;
using ofzza.aoc.utils.graph;

public partial class Day25: ISolution<string, long> {
  public long Run(SolutionExecutionRunInfo<string> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // First
    if (info.ExecutionIndex == 1) {
      // Test starting at each node
      for (var i=0; i<input.AllNodes.Length; i++) {
        var start = input.AllNodes[i];

        // Log starting node
        var connectionsStr = string.Join(", ", start.ConnectedNodes.Select(c => $"""'{c.Payload}'"""));
        log.WriteLine($"""- Starting at node '{start.Payload}' with {start.ConnectedNodes.Count()} connections to:""");
        log.WriteLine($"""    - Connections: {connectionsStr}""");
        log.Progress(i, input.AllNodes.Length);

        // Get initial node group and connections
        var nodes = new List<GraphNode<string>>() { start };
        var connections = new List<(GraphNode<string> Source, GraphNode<string> Target)>();
        connections.AddRange(start.ConnectedNodes.Select(n => (start, n)));

        // Crawl the graph trying to get to only 3 outgoing connections
        while (connections.Count() != 3 && connections.Count() != 0) {
          // Add another node to the graph
          var connection = connections[0];
          var node = connection.Target;
          nodes.Add(node);
          connections.AddRange(node.ConnectedNodes.Select(n => (node, n)));

          // Clear already added nodes from connections
          connections = connections.Where(c => !nodes.Any(n => n == c.Target)).ToList();

          // Log
          if (log.CheckLogLevel(ConsoleLoggingLevel.All)) {
            var groupNodesStr = string.Join(", ", nodes.Select(c => $"""'{c.Payload}'"""));
            var groupConnectionsStr = string.Join(", ", connections.Select(c => $"""'{c.Source.Payload}->{c.Target.Payload}'"""));
            log.WriteLine($"""  - Grouped {nodes.Count()} nodes with {connections.Count()} connections:""", ConsoleLoggingLevel.All);
            log.WriteLine($"""    - Nodes: {groupNodesStr}""", ConsoleLoggingLevel.All);
            log.WriteLine($"""    - Connections: {groupConnectionsStr}""", ConsoleLoggingLevel.All);
          }
        }

        // If 3 connections left, return success
        if (connections.Count() == 3) {
          return nodes.Count() * (input.AllNodes.Length - nodes.Count());
        }
      }
      // Return default
      return 0;
    }
    // Second
    else if (info.ExecutionIndex == 2) {
      return 0;
    }
    // No other index supported
    else {
      throw new Exception($"""Index {info.ExecutionIndex} not supported!""");
    }
  }
}
