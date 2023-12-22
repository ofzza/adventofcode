namespace ofzza.aoc.year2023.utils.aplenty;
using ofzza.aoc.utils.interval;

public class Aplenty {

  private Workflow[] Workflows { init; get; }
  private Part[] Parts { init; get; }

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="input">Parsed input containing workflows and parts definitions</param>
  public Aplenty (Input input) {
    // Connect workflow rules, to target workflows
    var dict = new Dictionary<string, Workflow>();
    foreach (var workflow in input.Workflows) {
      dict.Add(workflow.Name, workflow);
    }
    foreach (var workflow in input.Workflows) {
      foreach (var rule in workflow.Rules) {
        rule.TargetWorkflow = dict.ContainsKey(rule.TargetWorkflowName) ? dict[rule.TargetWorkflowName] : null;
      }
    }
    foreach (var part in input.Parts) {
      part.Value = part.X + part.M + part.A + part.S;
    }
    // Store data
    this.Workflows = input.Workflows;
    this.Parts = input.Parts;
  }

  /// <summary>
  /// Processes all parts using provided workflows and returns accepter and rejected ones
  /// </summary>
  /// <param name="initialWorkflowName">name of the initial workflow to use</param>
  /// <returns>Arrays of accepter and rejected parts</returns>
  public (Part[] Accepted, Part[] Rejected) ProcessParts (string initialWorkflowName) {
    return this.ProcessPartsInternal(initialWorkflowName, this.Parts);
  }

  public (Part[] Accepted, Part[] Rejected) ProcessPartsInternal (string initialWorkflowName, Part[] parts) {
    // Initialize accepter and rejected parts' buckets
    var accepted = new List<Part>();
    var rejected = new List<Part>();
    // Get initial workflow
    var initialWorkflow = this.Workflows.First(wf => wf.Name == initialWorkflowName);
    // Process parts
    foreach (var part in parts) {
      // Evaluated through forwarded workflows
      var workflow = (Workflow?)initialWorkflow;
      while (workflow != null) {
        // Evaluate part using current workflow
        var eval = this.EvaluateWorkflow(workflow, part);
        // Process accepted pard
        if (eval.AccepterOrRejected == true) {
          accepted.Add(part);
          break;
        }
        // Process rejected part
        else if (eval.AccepterOrRejected == false) {
          rejected.Add(part);
          break;
        }
        // Forward part to next workflow
        else if (eval.AccepterOrRejected == null && eval.NextWorkflow != null) {
          workflow = eval.NextWorkflow;
          continue;
        }
        else throw new Exception("This should never happen! Disallowed result from workflow evaluation");
      }
    }
    // Return accepter and rejected parts
    return (accepted.ToArray(), rejected.ToArray());
  }

  private (bool? AccepterOrRejected, Workflow? NextWorkflow) EvaluateWorkflow (Workflow workflow, Part part) {
    foreach (var rule in workflow.Rules) {
      // Evaluate rule
      var accepted = this.EvaluateRule(rule, part);
      // If accepted by rule, return as accepted
      if (accepted) {
        // If target workflow specified for rule, forward to target workflow
        if (rule.TargetWorkflow != null) {
          return (null, rule.TargetWorkflow);
        }
        // If rule explicitly accepting
        else if (rule.TargetWorkflowName == "A") {
          return (true, null);
        }
        // If rule explicitly rejecting
        else if (rule.TargetWorkflowName == "R") {
          return (false, null);
        }
      }
    }
    throw new Exception($"""This should never happen! Workflow with no rule satisfied""");
  }

  private bool EvaluateRule (Rule rule, Part part) {
    // Evaluate rule with no condition
    if (rule.ConditionProperty == null) {
      return true;
    }
    // Evaluate less-than rule
    else if (rule.ConditionOperator == Operator.LessThan) {
      var partValue = this.GetPartPropertyValue(part, (PartProperty)rule.ConditionProperty!);
      return partValue < rule.ConditionValue!;
    }
    // Evaluate less-than rule
    else if (rule.ConditionOperator == Operator.GreaterThan) {
      var partValue = this.GetPartPropertyValue(part, (PartProperty)rule.ConditionProperty!);
      return partValue > rule.ConditionValue!;
    }
    throw new Exception($"""This should never happen! Failed evaluating rule""");
  }

  private int GetPartPropertyValue (Part part, PartProperty property) {
    switch (property) {
      case PartProperty.X: return part.X;
      case PartProperty.M: return part.M;
      case PartProperty.A: return part.A;
      case PartProperty.S: return part.S;
    }
    throw new Exception($"""This should never happen! Unknown part property '{property}'""");
  }

  /// <summary>
  /// Calculates number of possible acceptable parts
  /// </summary>
  /// <param name="initialWorkflowName">name of the initial workflow to use</param>
  /// <returns>Number of possible acceptable parts</returns>
  public long GetPossibleAcceptablePartsCount (string initialWorkflowName) {
    // Get initial workflow
    var initialWorkflow = this.Workflows.First(wf => wf.Name == initialWorkflowName);
    // Extract acceptable intervals for each part property from rules
    var intervals = this.GetPossibleAcceptablePartsCountInternal(
      initialWorkflow,
      new (Interval<int> X, Interval<int> M, Interval<int> A, Interval<int> S)[] {(
        new Interval<int>() { Start = 1, End = 4000 },
        new Interval<int>() { Start = 1, End = 4000 },
        new Interval<int>() { Start = 1, End = 4000 },
        new Interval<int>() { Start = 1, End = 4000 }
      )}
    );
    // Count all possible parts
    long count = 0;
    foreach (var interval in intervals) {
      var dx = interval.X.End - interval.X.Start + 1;
      var dm = interval.M.End - interval.M.Start + 1;
      var da = interval.A.End - interval.A.Start + 1;
      var ds = interval.S.End - interval.S.Start + 1;
      count += dx * dm * da * ds;
    }
    return count;    
  }

  private (Interval<int> X, Interval<int> M, Interval<int> A, Interval<int> S)[] GetPossibleAcceptablePartsCountInternal (Workflow workflow, (Interval<int> X, Interval<int> M, Interval<int> A, Interval<int> S)[] intervals) {
    // Return number of possible acceptable parts
    return new (Interval<int> X, Interval<int> M, Interval<int> A, Interval<int> S)[] {};
  }
}
