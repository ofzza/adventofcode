namespace ofzza.aoc.year2023.day08;

using ofzza.aoc.utils;

public class Input {
  public required Direction[] Directions { init; get; }
  public required Node[] Nodes { init; get; }

}

public class Node {
  /// <summary>
  /// Global registry of all created nodes by their ID
  /// </summary>
  private static Dictionary<string, Node> nodes = new Dictionary<string, Node>();
  /// <summary>
  /// Clears all collected references to nodes from global registry
  /// </summary>
  public static void ClearNodes () {
    Node.nodes.Clear();
  }
  /// <summary>
  /// Gets a registered node from the global registry by its Id
  /// </summary>
  /// <param name="id">ID of the node to look up</param>
  /// <returns>Registered node with the requested Id, if found, else null</returns>
  public static Node? Get (string id) {
    return Node.nodes[id];
  }
  /// <summary>
  /// Gets all registered nodes from the global registry
  /// </summary>
  /// <returns>All registered nodes</returns>
  public static Node[] GetAll () {
    return Node.nodes.Values.ToArray();
  }

  /// <summary>
  /// Holds node Id
  /// </summary>
  private string? id;
  /// <summary>
  /// Initializes / Gets node Id, and registers a node by this Id to the global registry
  /// </summary>
  public required string Id {
    init { Node.nodes.Add(this.id = value, this); }
    get { return this.id!; }
  }

  /// <summary>
  /// Id of the node to the left
  /// </summary>
  public required string LeftNodeId { init; get; }
  /// <summary>
  /// Reference to the node on the left
  /// </summary>
  private Node? leftNode;
  /// <summary>
  /// Gets reference to the node on the left
  /// </summary>
  public Node LeftNode {
    get { return this.leftNode != null ? this.leftNode : this.leftNode = Node.nodes[this.LeftNodeId!]; }
  }

  /// <summary>
  /// Id of the node to the right
  /// </summary>
  public required string RightNodeId { init; get; }
  /// <summary>
  /// Reference to the node on the right
  /// </summary>
  private Node? rightNode;
  /// <summary>
  /// Gets reference to the node on the right
  /// </summary>
  public Node RightNode {
    get { return this.rightNode != null ? this.rightNode : this.rightNode = Node.nodes[this.RightNodeId!]; }
  }
}
