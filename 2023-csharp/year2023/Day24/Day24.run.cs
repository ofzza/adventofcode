namespace ofzza.aoc.year2023.day24;

using ofzza.aoc.utils;
using Vector = ofzza.aoc.utils.vector.Vector;

public partial class Day24: ISolution<(double[] Min, double[] Max, string Input), long> {
  public long Run(SolutionExecutionRunInfo<(double[] Min, double[] Max, string Input)> info, Console log, bool verbose, bool obfuscate) {
    // Parse input
    var input = parse(info.InputValue!);
    // First
    if (info.ExecutionIndex == 1) {
      // Find intersecting vectors
      long count = 0;
      for (var i=0; i<input.Length - 1; i++) {
        var a = input[i];
        for (var j=i + 1; j<input.Length; j++) {
          // Check intersection
          var b = input[j];
          var intersection = Vector.FindLineIntersection(a, b, (0, 1));
          var inside = intersection != null
            && intersection[0] >= info.InputValue.Min[0] && intersection[0] <= info.InputValue.Max[0]
            && intersection[1] >= info.InputValue.Min[1] && intersection[1] <= info.InputValue.Max[1];
          var a2d = new Vector() { Origin = new double[] { a.Origin[0], a.Origin[1] }, Magnitude = new double[] { a.Magnitude[0], a.Magnitude[1] } };
          var scaleA = intersection == null ? null : a2d.GetScaleToPoint(intersection!);
          var b2d = new Vector() { Origin = new double[] { b.Origin[0], b.Origin[1] }, Magnitude = new double[] { b.Magnitude[0], b.Magnitude[1] } };
          var scaleB = intersection == null ? null : b2d.GetScaleToPoint(intersection!);
          // Log
          log.WriteLine($"""- Vector {a.Origin[0]} x {a.Origin[1]} @ {a.Magnitude[0]} x {a.Magnitude[1]}""");
          log.WriteLine($"""- Vector {b.Origin[0]} x {b.Origin[1]} @ {b.Magnitude[0]} x {b.Magnitude[1]}""");
          if (intersection == null) {
            log.WriteLine($"""  Have no intersection""");
          }
          else if (!inside) {
            log.WriteLine($"""  Have an intersection OUTSIDE the monitored area: {intersection![0]} x {intersection![1]}""");
          } else if (scaleA < 0) {
            log.WriteLine($"""  Have an intersection  INSIDE the monitored area: {intersection![0]} x {intersection![1]} (In past for vector A)""");
          } else if (scaleB < 0) {
            log.WriteLine($"""  Have an intersection  INSIDE the monitored area: {intersection![0]} x {intersection![1]} (In past for vector B)""");
          } else {
            log.WriteLine($"""  Have an intersection  INSIDE the monitored area: {intersection![0]} x {intersection![1]}""");
          }
          log.WriteLine();
          // Count intersections
          if (intersection != null && inside && scaleA > 0 && scaleB > 0) count++;
        }
      }
      // Return number of intersecting vectors
      return count;
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
