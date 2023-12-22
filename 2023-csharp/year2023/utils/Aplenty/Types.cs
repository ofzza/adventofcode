namespace ofzza.aoc.year2023.utils.aplenty;

public class Input {

  public required Workflow[] Workflows { init; get; }
  public required Part[] Parts { init; get; }
}

public class Workflow {
  public required string Name { init; get; }
  public required Rule[] Rules { init; get; }

}

public class Rule {
  public required PartProperty? ConditionProperty { init; get; }
  public required Operator? ConditionOperator { init; get; }
  public required int? ConditionValue { init; get; }
  public required string TargetWorkflowName { init; get; }
  public Workflow? TargetWorkflow { get; set; } = null;
}

public class Part {
  public required int X { init; get; }
  public required int M { init; get; }
  public required int A { init; get; }
  public required int S { init; get; }
  public int Value { get; set; }
}

public enum PartProperty {
  X = 0,
  M = 1,
  A = 2,
  S = 3,
}

public enum Operator {
  LessThan = '<',
  GreaterThan = '>'
}
