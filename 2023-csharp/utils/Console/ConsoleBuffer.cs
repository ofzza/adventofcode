namespace ofzza.aoc.utils;

using System.Net;
using ofzza.aoc.utils.matrix;

/// <summary>
/// Console canvas is used to prepare Console output in a memory buffer before outputting to console
/// </summary>
public class ConsoleBuffer {

  /// <summary>
  /// Converts a numeric index into a letter for easier displaying
  /// </summary>
  /// <param name="index">Index to map to a letter</param>
  /// <returns>A letter the index provided was mapped to</returns>
  public static char IndexToLetter (long? index) {
    return index == null ? '0' : index < 26 ? (char)(65 + index!) : index < 52 ? (char)(97 + index! - 26) : '?';
  }

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
  /// Clears the buffer by filling it with whitespace characters
  /// </summary>
  public void ClearBuffer() {
    this.FillBuffer(' ');
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
