namespace ofzza.aoc.year2023.day25;

using ofzza.aoc.utils;
using ofzza.aoc.utils.graph;

public partial class Day25: ISolution<string, long> {
  private static Graph<string> parse (string input) {
    // Parse connections
    var connections = input.Split('\n').Select(l => {
      var parsed = l.Split(':');
      return ((string Source, string[] Targets))(
        parsed[0].Trim(),
        parsed[1].Trim().Split(' ').Select(s => s.Trim()).ToArray()
      );
    }).ToArray();

    // Connect nodes
    var nodes = new Dictionary<string, GraphNode<string>>();
    foreach (var s in connections) {
      // Get/Add source node
      if (!nodes.ContainsKey(s.Source)) nodes.Add(s.Source, new GraphNode<string>() { Payload = s.Source });
      var source = nodes[s.Source];
      // Get/Add target nodes
      foreach (var t in s.Targets) {
        // Get/Add target node
        if (!nodes.ContainsKey(t)) nodes.Add(t, new GraphNode<string>() { Payload = t });
        var target = nodes[t];
        // Add connection
        source.ConnectedNodes.Add(target);
        target.ConnectedNodes.Add(source);
      }
    }

    // Return Graph
    return new Graph<string>() { AllNodes = nodes.Values.ToArray() };
  }
}
