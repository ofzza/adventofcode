namespace ofzza.aoc.utils.graph;

using System;

public class GraphNode<T> {
  public required T Payload { init; get; }
  public List<GraphNode<T>> ConnectedNodes { get; set; } = new List<GraphNode<T>>();  
}
