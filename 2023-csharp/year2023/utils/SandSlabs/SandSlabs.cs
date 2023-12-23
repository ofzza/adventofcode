
namespace ofzza.aoc.year2023.utils.sandslabs;

using System.Runtime.ExceptionServices;
using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

public class SandSlabs {

  private Slab[] Slabs { init; get; }
  
  private MatrixIndexer Index { init; get; }

  public SandSlabs ((long[] Start, long[] End)[] input) {
    // Initialize slabs
    var slabs = new List<Slab>();
    foreach (var i in input) slabs.Add(new Slab(i.Start, i.End));
    // Sort slabs by bottom height
    this.Slabs = slabs.ToArray();
    Array.Sort(this.Slabs, new SlabBottomHeightComparer());
    // Find min/max of each coordinate and initialize a matrix index
    var min = new long[] { long.MaxValue, long.MaxValue, long.MaxValue };
    var max = new long[] { long.MinValue, long.MinValue, long.MinValue };
    foreach (var slab in this.Slabs) {
      if (slab.Min[0] < min[0]) min[0] = slab.Min[0];
      if (slab.Min[1] < min[1]) min[1] = slab.Min[1];
      if (slab.Min[2] < min[2]) min[2] = slab.Min[2];
      if (slab.Max[0] > max[0]) max[0] = slab.Max[0];
      if (slab.Max[1] > max[1]) max[1] = slab.Max[1];
      if (slab.Max[2] > max[2]) max[2] = slab.Max[2];
    }
    this.Index = new MatrixIndexer(new long[] { max[0] + 1, max[1] + 1, max[2] + 1 });
    if (min[0] < 0 || min[1] < 0 || min[2] < 0) throw new Exception("This should never happen. Coordinates od all slabs should be positive!");
  }

  /// <summary>
  /// Stacks slabs on top of each other based on their initial positions
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  /// </summary>
  public Slab[] Stack(Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    // Initialize top-down overview
    var topOverviewProjection = new long?[this.Index.Dimensions[0] * this.Index.Dimensions[1]];
    for (var i=0; i<topOverviewProjection.Length; i++) topOverviewProjection[i] = null;
    var topOverviewProjectionIndex = new MatrixIndexer(new long[] { this.Index.Dimensions[0], this.Index.Dimensions[1] });
    
    // Initialize console buffer
    var buffer = new ConsoleBuffer(topOverviewProjectionIndex);

    // Stack slabs on top of each other
    var droppedSlabs = new Slab[this.Slabs.Length];
    for (var i=0; i<this.Slabs.Length; i++) {
      var slab = this.Slabs[i];

      // Log
      log.WriteLine($"""- Dropping slab {ConsoleBuffer.IndexToLetter(i)}""", level);

      // Get "shadow" of the slab about to be dropped
      var slabShadowCoords = new List<long[]>();
      // Get shadow of a Z-index oriented slab
      if (slab.Min[0] == slab.Max[0] && slab.Min[1] == slab.Max[1]) {
        slabShadowCoords.Add(new long[] { slab.Min[0], slab.Min[1] });
      }
      // Get shadow of a X-index oriented slab
      else if (slab.Min[0] != slab.Max[0] && slab.Min[1] == slab.Max[1] && slab.Min[2] == slab.Max[2]) {
        for (var x=slab.Min[0]; x<=slab.Max[0]; x++) slabShadowCoords.Add(new long[] { x, slab.Min[1] });
      }
      // Get shadow of a Y-index oriented slab
      else if (slab.Min[0] == slab.Max[0] && slab.Min[1] != slab.Max[1] && slab.Min[2] == slab.Max[2]) {
        for (var y=slab.Min[1]; y<=slab.Max[1]; y++) slabShadowCoords.Add(new long[] { slab.Min[0], y });
      }
      else throw new Exception("This should never happen. All slabs should be 1D!");

      // Log shadow
      buffer.FillBuffer('.');
      for (var j=0; j<topOverviewProjection.Length; j++) if (topOverviewProjection[j] != null) buffer.WriteToBuffer(ConsoleBuffer.IndexToLetter(topOverviewProjection[j]), topOverviewProjectionIndex.IndexToCoordinates(j));
      foreach (var c in slabShadowCoords) buffer.WriteToBuffer('*', c);
      buffer.WriteToLog(log, ConsoleLoggingLevel.All);
      
      // Find top previously dropped slab's high point the slab can drop down to
      long maxShadowLevel = 0;
      var underlyingDroppedSlabIndexes = new List<int>();
      foreach (var coords in slabShadowCoords) {
        var underlyingDroppedSlabIndex = topOverviewProjection[(int)topOverviewProjectionIndex.CoordinatesToIndex(coords!)];
        if (underlyingDroppedSlabIndex != null && !underlyingDroppedSlabIndexes.Contains((int)underlyingDroppedSlabIndex!)) {
          var underlyingDroppedSlab = droppedSlabs[(int)underlyingDroppedSlabIndex!];
          underlyingDroppedSlabIndexes.Add((int)underlyingDroppedSlabIndex!);
          if (underlyingDroppedSlab.Max[2] > maxShadowLevel) maxShadowLevel = underlyingDroppedSlab.Max[2];
        }
      }

      // Drop the slab
      var dropLength = slab.Min[2] - (maxShadowLevel + 1) ;
      var droppedSlab = new Slab(
        new long[] { slab.Min[0], slab.Min[1], slab.Min[2] - dropLength },
        new long[] { slab.Max[0], slab.Max[1], slab.Max[2] - dropLength }
      );

      // Reference touching underlying slabs from current slab and vice versa
      foreach (var index in underlyingDroppedSlabIndexes.Where(index => droppedSlabs[index].Max[2] == maxShadowLevel)) {
        droppedSlabs[index].SlabsStackedOnTop.Add(i);
        droppedSlab.SlabsStackedOnTopOf.Add(index);
      }

      // Log
      log.WriteLine($"""  ... Slab {ConsoleBuffer.IndexToLetter(i)} rests on top of: {(droppedSlab.SlabsStackedOnTopOf.Count() == 0 ? "Nothing" : string.Join(", ", droppedSlab.SlabsStackedOnTopOf.Select(i => ConsoleBuffer.IndexToLetter(i))))}""", level);
      if (droppedSlab.SlabsStackedOnTopOf.Count == 1) log.WriteLine($"""      Making slab {ConsoleBuffer.IndexToLetter(droppedSlab.SlabsStackedOnTopOf[0])} critical!""", level);

      // Store the stacked slab
      droppedSlabs[i] = droppedSlab;
      foreach (var coords in slabShadowCoords) {
        topOverviewProjection[(int)topOverviewProjectionIndex.CoordinatesToIndex(coords!)!] = i;
      }

      // Log
      log.WriteLine(level);
    }

    // Return dropped slabs
    return droppedSlabs.ToArray();
  }

  /// <summary>
  /// Finds critical slabs within a stacked slabs structure. Critical slabs are ones which if removed, would cause other slabs to collapse.
  /// </summary>
  /// <param name="stacked">Stacked slabs structure</param>
  /// <returns>Indexes of critical slabs from within a stacked slabs structure</returns>
  public int[] FindCriticalSlabs (Slab[] stacked) {
    // Initialize  deduplicating hashset
    var criticalIndexes = new HashSet<int>();
    // Find critical slab indexes
    foreach (var s in stacked) if (s.SlabsStackedOnTopOf.Count() == 1) criticalIndexes.Add(s.SlabsStackedOnTopOf[0]);
    // Return critical slabs
    return criticalIndexes.ToArray();
  }

  /// <summary>
  /// Evaluates a critical slab by finding how many slabs would cascade-collapse if it were removed
  /// </summary>
  /// <param name="stacked">Stacked slabs structure</param>
  /// <param name="criticalIndex">Index of a critical slab being evaluated</param>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  /// <returns>Number of slabs which would collapse</returns>
  public int[] EvaluateCriticalSlab (Slab[] stacked, int criticalIndex, Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    // Keep track of disintegrated
    var disintegratedRegistry = new bool[stacked.Length];
    for (var i=0; i<disintegratedRegistry.Length; i++) disintegratedRegistry[i] = i == criticalIndex;

    // Var keep track of newly disintegrated
    var disintegratedCount = 1;
    while (disintegratedCount > 0) {
      // Reset count
      disintegratedCount = 0;
      // Check if any slabs are now left without slabs to stack on
      for (var i=0; i<stacked.Length; i++) {
        if (!disintegratedRegistry[i]) {
          var remainingOnTopOf = stacked[i].SlabsStackedOnTopOf.Where(j => !disintegratedRegistry[j]);
          if (stacked[i].SlabsStackedOnTopOf.Count() != 0 && remainingOnTopOf.Count() == 0) {
            disintegratedRegistry[i] = true;
            disintegratedCount++;
          }
        }
      }
    }

    // Collect all disintegrated
    var disintegratedSlabs = new List<int>();
    for (var i=0; i<disintegratedRegistry.Length; i++) if (disintegratedRegistry[i]) disintegratedSlabs.Add(i);
    return disintegratedSlabs.ToArray();
  }

}
