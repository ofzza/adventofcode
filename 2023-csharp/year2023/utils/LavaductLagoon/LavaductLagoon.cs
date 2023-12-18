namespace ofzza.aoc.year2023.utils.lavaductlagoon;

using System.Collections;
using ofzza.aoc.utils;

public class LavaductLagoon {

  /// <summary>
  /// Digging instructions
  /// </summary>
  private (Direction Direction, long Length, string Color)[] Instructions { init; get; }

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="input">Digging instructions</param>
  public LavaductLagoon (string[][] input, bool valueFromColor = false) {
    // Parse instructions
    this.Instructions = new (Direction Direction, long Length, string Color)[input.Length];
    for (var i=0; i<input.Length; i++) {
      // Parse values as given
      if (!valueFromColor) {
        switch (input[i][0]) {
          case "U": this.Instructions[i] = (Direction.Top, int.Parse(input[i][1]), input[i][2]); break;
          case "D": this.Instructions[i] = (Direction.Bottom, int.Parse(input[i][1]), input[i][2]); break;
          case "L": this.Instructions[i] = (Direction.Left, int.Parse(input[i][1]), input[i][2]); break;
          case "R": this.Instructions[i] = (Direction.Right, int.Parse(input[i][1]), input[i][2]); break;
        }
      }
      // Parse color as value
      else {
        var lengthFromColor = Convert.ToInt32(input[i][2].Substring(2, 5), 16);
        var directionFromColor = input[i][2].Substring(7, 1);
        switch (directionFromColor) {
          case "3": this.Instructions[i] = (Direction.Top, lengthFromColor, input[i][2]); break;
          case "1": this.Instructions[i] = (Direction.Bottom, lengthFromColor, input[i][2]); break;
          case "2": this.Instructions[i] = (Direction.Left, lengthFromColor, input[i][2]); break;
          case "0": this.Instructions[i] = (Direction.Right, lengthFromColor, input[i][2]); break;
        } 
      }
    }
  }

  public ((long[] Start, long[] End) Coords, Direction Direction)[] DigPath () {
    // Initialize path
    var path = new List<((long[] Start, long[] End) Coords, Direction Direction)>() { };
    
    // Follow instructions
    foreach (var instruction in this.Instructions) {
      // Get current location
      var current = path.Count > 0 ? path.Last().Coords.End : new long[] { 0, 0 };
      // Find next section of path
      switch (instruction.Direction) {
        case Direction.Top:    path.Add(((current, new long[] { current[0], current[1] - instruction.Length }), Direction.Top)); break;
        case Direction.Bottom: path.Add(((current, new long[] { current[0], current[1] + instruction.Length }), Direction.Bottom)); break;
        case Direction.Left:   path.Add(((current, new long[] { current[0] - instruction.Length, current[1] }), Direction.Left)); break;
        case Direction.Right:  path.Add(((current, new long[] { current[0] + instruction.Length, current[1] }), Direction.Right)); break;
      }
    }
    
    // Return path
    return path.ToArray();
  }

  public long GetPathSize (((long[] Start, long[] End) Coords, Direction Direction)[] path) {
    long size = 0;
    foreach (var p in path) {
      size += Math.Abs(p.Coords.End[0] - p.Coords.Start[0]) + Math.Abs(p.Coords.End[1] - p.Coords.Start[1]);
    }
    return size;
  }

  public (long[] Start, long[] End)[] FillArea (((long[] Start, long[] End) Coords, Direction Direction)[] path) {
    // Find path bounds
    var xmin = long.MaxValue;
    var xmax = long.MinValue;
    var ymin = long.MaxValue;
    var ymax = long.MinValue;
    foreach (var c in path) {
      if (c.Coords.Start[0] < xmin) xmin = c.Coords.Start[0];
      if (c.Coords.End[0] < xmin) xmin = c.Coords.End[0];
      if (c.Coords.Start[0] > xmax) xmax = c.Coords.Start[0];
      if (c.Coords.End[0] > xmax) xmax = c.Coords.End[0];
      if (c.Coords.Start[1] < ymin) ymin = c.Coords.Start[1];
      if (c.Coords.End[1] < ymin) ymin = c.Coords.End[1];
      if (c.Coords.Start[1] > ymax) ymax = c.Coords.Start[1];
      if (c.Coords.End[1] > ymax) ymax = c.Coords.End[1];
    }
    xmin--; xmax++; ymin--; ymax++;
    // Initialize filled area
    var area = new List<(long[] Start, long[] End)>();
    // Find bands
    var ys = new List<long>();
    foreach (var p in path) {
      ys.Add(p.Coords.Start[1]);
      ys.Add(p.Coords.End[1]);
    }
    ys.Sort();
    ys = ys.Distinct().ToList();
    // For each band, find areas
    for (var y=1; y<ys.Count; y++) {
      // Get vertical band
      var y1 = ys[y - 1];
      var y2 = ys[y + 0];
      // Find all vertical sections of path intersecting the band
      var yIntersections = path.Where(s => (s.Direction == Direction.Top || s.Direction == Direction.Bottom) && ((s.Coords.Start[1] <= y1 && s.Coords.End[1] >= y2) || (s.Coords.End[1] <= y1 && s.Coords.Start[1] >= y2))).ToArray();
      // Sort intersections by x coordinate
      Array.Sort(yIntersections, new PathVerticalSectionXComparer());
      // Add up all the areas
      for (var x=1; x<yIntersections.Length; x++) {
        if (yIntersections[x - 1].Direction == Direction.Top && yIntersections[x].Direction == Direction.Bottom) {
          var x1 = yIntersections[x - 1].Coords.Start[0];
          var x2 = yIntersections[x + 0].Coords.Start[0];
          area.Add((new long[] { x1 + 1, y1 + 1 }, new long[] { x2 - 1, y2 - 1 }));
        }
      }
    }
    // For each band border, find areas
    foreach (var y in ys) {
      // Find lines on the same y coordinate
      var yIntersections = path.Where(s => (s.Direction == Direction.Top || s.Direction == Direction.Bottom) && ((s.Coords.Start[1] <= y && s.Coords.End[1] >= y) || (s.Coords.End[1] <= y && s.Coords.Start[1] >= y))).ToArray();
      var xIntersections = path.Where(s => (s.Direction == Direction.Left || s.Direction == Direction.Right) && s.Coords.Start[1] == y).ToArray();
      // Join and sort all lines
      var intersections = yIntersections.Concat(xIntersections).ToArray();
      Array.Sort(intersections, new PathAnySectionXComparer());
      // Find top-bottom vertical path section pairs
      for (var i=1; i<intersections.Length; i++) {
        var intersectionA = intersections[i - 1];
        var intersectionB = intersections[i - 0];
        if (intersectionA.Direction == Direction.Top && intersectionB.Direction == Direction.Bottom) {
          area.Add((new long[] { intersectionA.Coords.Start[0] + 1, y }, new long[] { intersectionB.Coords.Start[0] - 1, y}));
        }
      }
    };
    // Return filled area
    return area.ToArray();
  }

  public long GetAreaSize ((long[] Start, long[] End)[] area) {
    long size = 0;
    foreach (var a in area) size += Math.Abs(a.End[0] - a.Start[0] + 1) * Math.Abs(a.End[1] - a.Start[1] + 1);
    return size;
  }


  /// <summary>
  /// Outputs maze visual representation to Console
  /// </summary>
  /// <param name="path">Path to be logged</param>
  /// <param name="area">Area to be logged</param>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  public void Log (((long[] Start, long[] End) Coords, Direction Direction)[] path, (long[] Start, long[] End)[] area, Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    // Check log level
    if (!log.CheckLogLevel(level)) return;
    // Find path bounds
    var xmin = long.MaxValue;
    var xmax = long.MinValue;
    var ymin = long.MaxValue;
    var ymax = long.MinValue;
    foreach (var c in path) {
      if (c.Coords.Start[0] < xmin) xmin = c.Coords.Start[0];
      if (c.Coords.End[0] < xmin) xmin = c.Coords.End[0];
      if (c.Coords.Start[0] > xmax) xmax = c.Coords.Start[0];
      if (c.Coords.End[0] > xmax) xmax = c.Coords.End[0];
      if (c.Coords.Start[1] < ymin) ymin = c.Coords.Start[1];
      if (c.Coords.End[1] < ymin) ymin = c.Coords.End[1];
      if (c.Coords.Start[1] > ymax) ymax = c.Coords.Start[1];
      if (c.Coords.End[1] > ymax) ymax = c.Coords.End[1];
    }
    xmin--; xmax++; ymin--; ymax++;
    // Normalize path
    var normalizedPath = new ((long[] Start, long[] End) Coords, Direction Direction)[path.Length];
    for (var i=0; i<path.Length; i++) normalizedPath[i] = ((new long[] { path[i].Coords.Start[0] - xmin, path[i].Coords.Start[1] - ymin }, new long[] { path[i].Coords.End[0] - xmin, path[i].Coords.End[1] - ymin }), path[i].Direction);
    var normalizedArea = new (long[] Start, long[] End)[area.Length];
    for (var i=0; i<area.Length; i++) normalizedArea[i] = (new long[] { area[i].Start[0] - xmin, area[i].Start[1] - ymin }, new long[] { area[i].End[0] - xmin, area[i].End[1] - ymin });
    // Write path
    var buffer = new ConsoleBuffer(new long[] { xmax - xmin + 1, ymax - ymin + 1 });
    buffer.FillBuffer('.');
    for (var i=0; i<normalizedPath.Length; i++) {
      if (normalizedPath[i].Direction == Direction.Top) {
        for (var y=normalizedPath[i].Coords.End[1]; y<=normalizedPath[i].Coords.Start[1]; y++) {
          buffer.WriteToBuffer('^', new long[] { normalizedPath[i].Coords.Start[0], y });
        }
      }
      else if (normalizedPath[i].Direction == Direction.Bottom) {
        for (var y=normalizedPath[i].Coords.Start[1]; y<=normalizedPath[i].Coords.End[1]; y++) {
          buffer.WriteToBuffer('v', new long[] { normalizedPath[i].Coords.Start[0], y });
        }
      }
      else if (normalizedPath[i].Direction == Direction.Left) {
        for (var x=normalizedPath[i].Coords.End[0]; x<=normalizedPath[i].Coords.Start[0]; x++) {
          buffer.WriteToBuffer('<', new long[] { x, normalizedPath[i].Coords.Start[1] });
        }
      }
      else if (normalizedPath[i].Direction == Direction.Right) {
        for (var x=normalizedPath[i].Coords.Start[0]; x<=normalizedPath[i].Coords.End[0]; x++) {
          buffer.WriteToBuffer('>', new long[] { x, normalizedPath[i].Coords.Start[1] });
        }
      }
    }
    for (var i=0; i<normalizedArea.Length; i++) {
      for (var x=normalizedArea[i].Start[0]; x<=normalizedArea[i].End[0]; x++) {
        for (var y=normalizedArea[i].Start[1]; y<=normalizedArea[i].End[1]; y++) {
          buffer.WriteToBuffer('+', new long[] { x, y });
        }
      }
    }
    // Output to log
    buffer.WriteToLog(log, level);
  }

}

class PathVerticalSectionXComparer : IComparer {
    public int Compare(object? a, object? b) {
        return ((((long[] Start, long[] End) Coords, Direction Direction))a!).Coords.Start[0].CompareTo(
          ((((long[] Start, long[] End) Coords, Direction Direction))b!).Coords.Start[0]
        );
    }
}

class PathAnySectionXComparer : IComparer {
    public int Compare(object? a, object? b) {
        var first = ((((long[] Start, long[] End) Coords, Direction Direction))a!).Coords.Start[0].CompareTo(
          ((((long[] Start, long[] End) Coords, Direction Direction))b!).Coords.Start[0]
        );
        return first != 0 ? first : ((((long[] Start, long[] End) Coords, Direction Direction))a!).Coords.End[0].CompareTo(
          ((((long[] Start, long[] End) Coords, Direction Direction))b!).Coords.End[0]
        );
    }
}
