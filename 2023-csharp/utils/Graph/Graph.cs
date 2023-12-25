namespace ofzza.aoc.utils.graph;

using System;

/// <summary>
/// Helps manipulating a graph
/// </summary>
public class Graph<T> {
  public required GraphNode<T>[] AllNodes { init; get; }
}
