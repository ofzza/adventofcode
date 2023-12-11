namespace ofzza.aoc.utils;

using System;
using System.Diagnostics.CodeAnalysis;
using ofzza.aoc.utils.matrix;

/// <summary>
/// Console canvas is used to prepare Console output in a memory buffer before outputting to console
/// </summary>
public class ConsoleBuffer {
  /// <summary>
  /// Matrix index for the output canvas
  /// </summary>
  private MatrixIndexer Index { init; get; }

  /// <summary>
  /// Output buffer
  /// </summary>
  private char[] Buffer { init; get; }

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="dimensions">Dimensions of the output canvas</param>
  public ConsoleBuffer (long[] dimensions) {
    this.Index = new MatrixIndexer(dimensions);
    this.Buffer = new char[dimensions[1] * dimensions[0]];
  }
  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="index">Matrix index, indexing the output canvas</param>
  public ConsoleBuffer (MatrixIndexer index) {
    this.Index = index;
    this.Buffer = new char[index.Dimensions[1] * index.Dimensions[0]];
  }

  /// <summary>
  /// Fills the entire buffer with a single character
  /// </summary>
  /// <param name="c">Character to fill the buffer with</param>
  public void FillBuffer(char c) {
    for (var y=0; y<this.Index.Dimensions[1]; y++) {
      for (var x=0; x<this.Index.Dimensions[0]; x++) {
        this.Buffer[this.Index.CoordinatesToIndex(new long[] { x, y })] = c;
      }
    }
  }

  /// <summary>
  /// Writes a character to the console buffer at a specified location
  /// </summary>
  /// <param name="c">Character to write to the buffer</param>
  /// <param name="coords">Coordinates to write the character to</param>
  public void WriteToBuffer (char c, long[] coords) {
    this.WriteToBuffer(c, this.Index.CoordinatesToIndex(coords));
  }
  /// <summary>
  /// Writes a character to the console buffer at a specified location
  /// </summary>
  /// <param name="c">Character to write to the buffer</param>
  /// <param name="index">Index to write the character to</param>
  public void WriteToBuffer (char c, long index) {
    this.Buffer[index] = c;
  }

  /// <summary>
  /// Outputs prepared buffer to console
  /// </summary>
  /// <param name="log">Console instance to use</param>
  /// <param name="level">Logging level</param>
  /// <param name="options">Console write options</param>
  public void WriteToLog(Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, ConsoleWriteOptions? options = null) {
    for (var y=0; y<this.Index.Dimensions[1]; y++) {
      var line = "";
      for (var x=0; x<this.Index.Dimensions[0]; x++) line += this.Buffer[this.Index.CoordinatesToIndex(new long[] { x, y })];
      log.WriteLine(line, level, options);
    }
  }

}
