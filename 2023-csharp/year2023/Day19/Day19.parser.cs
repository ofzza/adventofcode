namespace ofzza.aoc.year2023.day19;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.aplenty;

public partial class Day19: ISolution<string, long> {
  private static Input parse (string input) {
    var parsed = input.Split("\n\n");
    var workflows = parsed[0].Split('\n').Select(l => {
      var parsed = l.Replace("}", "").Split('{');
      var name = parsed[0];
      var rules = parsed[1].Split(',').Select(r => {
        var parsed = r.Split(':');
          if (parsed.Length == 2) {
          var property = parsed[0][0] == 'x' ? PartProperty.X : parsed[0][0] == 'm' ? PartProperty.M : parsed[0][0] == 'a' ? PartProperty.A : PartProperty.S; 
          var op = parsed[0][1] == '<' ? Operator.LessThan : Operator.GreaterThan;
          var value = int.Parse(parsed[0].Substring(2));
          return new Rule() {
            ConditionProperty = property,
            ConditionOperator = op,
            ConditionValue = value,
            TargetWorkflowName = parsed[1]
          };
        } else if (parsed.Length == 1) {
          return new Rule() {
            ConditionProperty = null,
            ConditionOperator = null,
            ConditionValue = null,
            TargetWorkflowName = r
          };
        } else {
          throw new Exception($"""This should never happen! Error parsing rule '{r}'""");
        }
      }).ToArray();
      return new Workflow() {
        Name = name,
        Rules = rules
      };
    }).ToArray();
    var parts = parsed[1].Split('\n').Select(l => {
      var parts = l.Replace("{", "").Replace("}", "").Split(',').Select(p => {
        var parsed = p.Split('=');
        switch (parsed[0]) {
          case "x": return ((PartProperty Property, int Value))(PartProperty.X, int.Parse(parsed[1]));
          case "m": return ((PartProperty Property, int Value))(PartProperty.M, int.Parse(parsed[1]));
          case "a": return ((PartProperty Property, int Value))(PartProperty.A, int.Parse(parsed[1]));
          case "s": return ((PartProperty Property, int Value))(PartProperty.S, int.Parse(parsed[1]));
        }
        throw new Exception($"""This should never happen! Unknown property '{parsed[0]}'""");
      }).ToArray();
      return new Part() {
        X = parts.First(p => p.Property == PartProperty.X).Value,
        M = parts.First(p => p.Property == PartProperty.M).Value,
        A = parts.First(p => p.Property == PartProperty.A).Value,
        S = parts.First(p => p.Property == PartProperty.S).Value,
      };
    }).ToArray();
    return new Input() {
      Workflows = workflows,
      Parts = parts
    };
  }
}
