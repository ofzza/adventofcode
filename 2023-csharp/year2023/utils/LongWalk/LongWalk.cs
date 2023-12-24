
namespace ofzza.aoc.year2023.utils.longwalk;

using System.Runtime.ExceptionServices;
using ofzza.aoc.utils;
using ofzza.aoc.utils.matrix;

public class LongWalk {

  private Direction[] Labyrinth { init; get; }
  private MatrixIndexer Index { init; get; }
  private long LabyrinthStartIndex { init; get; }
  private long LabyrinthEndIndex { init; get; }

  public event Action<long[]>? OnReachedEndIndex = null;

  public LongWalk (char[][] input) {
    // Initialize labyrinth and indexer
    this.Index = new MatrixIndexer(new long[] { input[0].Length, input.Length });
    this.Labyrinth = new Direction[this.Index.Length];
    // Map input into a directional map
    for (var y=0; y<input.Length; y++) {
      // Map tiles
      for (var x=0; x<input[y].Length; x++) {
        var i = this.Index.CoordinatesToIndex(new long[] { x, y });
        switch (input[y][x]) {
          case '#': this.Labyrinth[i] = Direction.None; break;
          case '^': this.Labyrinth[i] = Direction.Top; break;
          case 'v': this.Labyrinth[i] = Direction.Bottom; break;
          case '<': this.Labyrinth[i] = Direction.Left; break;
          case '>': this.Labyrinth[i] = Direction.Right; break;
          case '.': {
            this.Labyrinth[i] = Direction.Any;
            if (y == 0) this.LabyrinthStartIndex = i;
            if (y == this.Index.Dimensions[1] - 1) this.LabyrinthEndIndex = i;
            break;
          }
        }
      }
    }
  }

  public void Walk (Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, bool ignoreSlopes = false,long? index = null, long[]? path = null, long? lastIntersectionIndex = null, Dictionary<long, (bool Valid, long IntersectionIndex, long[] PathBetweenIntersections)>? cache = null) {
    // Initialize cache and last intersection index
    var nextLastIntersectionIndex = lastIntersectionIndex;
    if (cache == null) cache = new Dictionary<long, (bool Valid, long IntersectionIndex, long[] PathBetweenIntersections)>();

    // Set current position
    index = index != null ? index : this.LabyrinthStartIndex;

    // Append to current path
    var updatedPath = (path != null ? path : new long[0]).Concat(new long[] { (long)index! }).ToArray();

    // Log
    log.WriteLine("- Walking ...", ConsoleLoggingLevel.All);
    this.Log(updatedPath, log, ConsoleLoggingLevel.All);
    log.WriteLine(ConsoleLoggingLevel.All);

    // Check if reached the end
    if (index == this.LabyrinthEndIndex && this.OnReachedEndIndex != null) this.OnReachedEndIndex(updatedPath);

    // Slide down slope(s) // TODO: Cache paths between intersections
    if (!ignoreSlopes && this.Labyrinth[(long)index!] == Direction.Top) {
      var current = this.Index.IndexToCoordinates((long)index!);
      var next = this.Index.CoordinatesToIndex(new long[] { current[0], current[1] - 1 });
      if (this.Labyrinth[next] != Direction.None && !updatedPath.Contains(next)) this.Walk(log, level, ignoreSlopes, next, updatedPath, lastIntersectionIndex, cache);
    }
    else if (!ignoreSlopes && this.Labyrinth[(long)index!] == Direction.Bottom) {
      var current = this.Index.IndexToCoordinates((long)index!);
      var next = this.Index.CoordinatesToIndex(new long[] { current[0], current[1] + 1 });
      if (this.Labyrinth[next] != Direction.None && !updatedPath.Contains(next)) this.Walk(log, level, ignoreSlopes, next, updatedPath, lastIntersectionIndex, cache);
    }
    else if (!ignoreSlopes && this.Labyrinth[(long)index!] == Direction.Left) {
      var current = this.Index.IndexToCoordinates((long)index!);
      var next = this.Index.CoordinatesToIndex(new long[] { current[0] - 1, current[1] });
      if (this.Labyrinth[next] != Direction.None && !updatedPath.Contains(next)) this.Walk(log, level, ignoreSlopes, next, updatedPath, lastIntersectionIndex, cache);
    }
    else if (!ignoreSlopes && this.Labyrinth[(long)index!] == Direction.Right) {
      var current = this.Index.IndexToCoordinates((long)index!);
      var next = this.Index.CoordinatesToIndex(new long[] { current[0] + 1, current[1] });
      if (this.Labyrinth[next] != Direction.None && !updatedPath.Contains(next)) this.Walk(log, level, ignoreSlopes, next, updatedPath, lastIntersectionIndex, cache);
    }
    // Walk to every available next tile that wan't traveled yet
    else {
      // Get neighbors (not in a wall, or going back)
      var neighbors = this.Index.GetNeighboringIndices((long)index!, false).Where(i => this.Labyrinth[i] != Direction.None && !updatedPath.Contains(i) && (!cache.ContainsKey(i) || cache[i].Valid));
      
      // If dead end, cache as dead end
      // if (index != this.LabyrinthEndIndex && neighbors.Count() == 0 && lastIntersectionIndex != null && lastIntersectionIndex != index && !cache.ContainsKey((int)lastIntersectionIndex!)) {
      //   // Ready data for cache
      //   var s = this.Index.IndexToCoordinates((long)lastIntersectionIndex!);
      //   var e = this.Index.IndexToCoordinates((long)index!);
      //   var skip = Array.IndexOf(updatedPath, (long)lastIntersectionIndex!);
      //   var cachingPath = updatedPath.Skip(skip).Take(updatedPath.Length - skip - 1).ToArray();
      //   // Log caching path
      //   log.WriteLine($"""- Caching dead-end from {s[0]}x{s[1]} -> {e[0]}x{e[1]} (length: {cachingPath.Count()}): {string.Join(", ", cachingPath.Select(i => { var c = this.Index.IndexToCoordinates(i); return $"""{c[0]}x{c[1]}"""; }))}""", ConsoleLoggingLevel.All);
      //   this.Log(cachingPath, log, ConsoleLoggingLevel.All);
      //   log.WriteLine(ConsoleLoggingLevel.All);
      //   // Cache first path between 2 intersections
      //   cache.Add((long)lastIntersectionIndex!, (false, (long)index!, new long[0]));
      // }
      // // If looping, cache as dead end
      // if (neighbors.Count() > 1 && lastIntersectionIndex != null && lastIntersectionIndex != index && cache.ContainsKey((int)lastIntersectionIndex!)) {
      //   // Ready data for cache
      //   var s = this.Index.IndexToCoordinates((long)lastIntersectionIndex!);
      //   var e = this.Index.IndexToCoordinates((long)index!);
      //   var skip = Array.IndexOf(updatedPath, (long)lastIntersectionIndex!);
      //   var cachingPath = updatedPath.Skip(skip).Take(updatedPath.Length - skip - 1).ToArray();
      //   // Log caching path
      //   log.WriteLine($"""- Caching loop from {s[0]}x{s[1]} -> {e[0]}x{e[1]} (length: {cachingPath.Count()}): {string.Join(", ", cachingPath.Select(i => { var c = this.Index.IndexToCoordinates(i); return $"""{c[0]}x{c[1]}"""; }))}""", ConsoleLoggingLevel.All);
      //   this.Log(cachingPath, log, ConsoleLoggingLevel.All);
      //   log.WriteLine(ConsoleLoggingLevel.All);
      //   // Cache first path between 2 intersections
      //   cache.Add((long)lastIntersectionIndex!, (false, (long)index!, new long[0]));
      // }
      // If intersection, cache path and set new latest intersection
      // else
      if (neighbors.Count() > 1 && lastIntersectionIndex != null && lastIntersectionIndex != index && !cache.ContainsKey((int)lastIntersectionIndex!)) {
        // Ready data for cache
        var s = this.Index.IndexToCoordinates((long)lastIntersectionIndex!);
        var e = this.Index.IndexToCoordinates((long)index!);
        var skip = Array.IndexOf(updatedPath, (long)lastIntersectionIndex!);
        var cachingPath = updatedPath.Skip(skip).Take(updatedPath.Length - skip - 1).ToArray();
        // Log caching path
        log.WriteLine($"""- Caching path from {s[0]}x{s[1]} -> {e[0]}x{e[1]} (length: {cachingPath.Count()}): {string.Join(", ", cachingPath.Select(i => { var c = this.Index.IndexToCoordinates(i); return $"""{c[0]}x{c[1]}"""; }))}""", ConsoleLoggingLevel.All);
        this.Log(cachingPath, log, ConsoleLoggingLevel.All);
        log.WriteLine(ConsoleLoggingLevel.All);
        // Cache first path between 2 intersections
        cache.Add((long)lastIntersectionIndex!, (true, (long)index!, cachingPath));
      }

      // Explore each neighbor
      foreach (var neighbor in neighbors) {
        // If cached answer(s) available, continue from cached answer
        if (cache.ContainsKey(neighbor!)) {
          // Get data from cache
          var s = this.Index.IndexToCoordinates(neighbor!);
          var e = this.Index.IndexToCoordinates(cache[neighbor].IntersectionIndex!);
          var cachedPath = cache[neighbor].PathBetweenIntersections;
          var appendedPath = updatedPath.Concat(cache[neighbor].PathBetweenIntersections).ToArray();
          // Log fast forwarding
          log.WriteLine($"""- Fast-Forwarding using cached path from {s[0]}x{s[1]} -> {e[0]}x{e[1]} (length: {cachedPath.Count()}): {string.Join(", ", cachedPath.Select(i => { var c = this.Index.IndexToCoordinates(i); return $"""{c[0]}x{c[1]}"""; }))}""", ConsoleLoggingLevel.All);
          this.Log(cachedPath, log, ConsoleLoggingLevel.All);
          log.WriteLine(ConsoleLoggingLevel.All);
          // Use cache to fast-forward
          this.Walk(log, level, ignoreSlopes, cache[neighbor].IntersectionIndex, appendedPath, cache[neighbor].IntersectionIndex, cache);
        }
        // Follow neighbor path
        else {
          var lastTakenIntersectionIndex = neighbors.Count() > 1 ? neighbor : lastIntersectionIndex;
          this.Walk(log, level, ignoreSlopes, neighbor, updatedPath, lastTakenIntersectionIndex, cache);
        }
      }
    }

  }

  public void Log (long[] path, Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    var buffer = new ConsoleBuffer(this.Index);
    for (var i=0; i<this.Labyrinth.Length; i++) buffer.WriteToBuffer(this.Labyrinth[i] == Direction.None ? '#' : '.', i);
    if (path != null) foreach (var i in path) buffer.WriteToBuffer('+', i);
    buffer.WriteToLog(log, level);
  }

}
