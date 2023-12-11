namespace ofzza.aoc.utils;

using System;
using System.Diagnostics.CodeAnalysis;

/// <summary>
/// Console logging service
/// </summary>
public class Console {
  #region Factory
  public static int DefaultPadding  { get; set; } = 0;
  public static bool DefaultSuppressWrite { get; set; } = false;
  public static ConsoleLoggingLevel DefaultLoggingLevel { get; set; } = ConsoleLoggingLevel.Verbose;
  public static bool DefaultSuppressProgress { get; set; } = false;
  public static string DefaultProgressPrompt { get; set; } = "";
  public static ConsoleColor? DefaultFgColor { get; set; } = null;
  public static ConsoleColor? DefaultBgColor { get; set; } = null;

  public static Console Create () {
    return new Console() {      
      Padding = Console.DefaultPadding,
      SuppressWrite = Console.DefaultSuppressWrite,
      LoggingLevel = Console.DefaultLoggingLevel,
      SuppressProgress = Console.DefaultSuppressProgress,
      ProgressPrompt = Console.DefaultProgressPrompt,      
      FgColor = Console.DefaultFgColor,
      BgColor = Console.DefaultBgColor,
    };
  }
  public static Console Create (ConsoleColor? fgColor) {
    return new Console() {
      Padding = Console.DefaultPadding,
      SuppressWrite = Console.DefaultSuppressWrite,
      LoggingLevel = Console.DefaultLoggingLevel,
      SuppressProgress = Console.DefaultSuppressProgress,
      ProgressPrompt = Console.DefaultProgressPrompt,      
      FgColor = fgColor,
      BgColor = Console.DefaultBgColor,
    };
  }
  public static Console Create (ConsoleColor? fgColor, ConsoleColor? bgColor) {
    return new Console() {
      Padding = Console.DefaultPadding,
      SuppressWrite = Console.DefaultSuppressWrite,
      LoggingLevel = Console.DefaultLoggingLevel,
      SuppressProgress = Console.DefaultSuppressProgress,
      ProgressPrompt = Console.DefaultProgressPrompt,      
      FgColor = fgColor,
      BgColor = bgColor,
    };
  }
  #endregion

  #region Config
  private DateTime CreatedDateTime { init; get; } = DateTime.Now;
  public int Padding  { init; get; } = 0;
  public bool SuppressWrite { init; get; } = false;
  public ConsoleLoggingLevel LoggingLevel { init; get; } = ConsoleLoggingLevel.Verbose;
  public bool SuppressProgress { init; get; } = false;
  public string ProgressPrompt { init; get; } = "";
  public ConsoleColor? FgColor { init; get; }
  public ConsoleColor? BgColor { init; get; }
  #endregion

  #region Progress
  public void Progress (long done, long total, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    this.Progress((double)done / (double)total, level);
  }
  public void Progress (double? progress, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    if (this.SuppressProgress) return;
    if (level > this.LoggingLevel) return;
    if ((DateTime.Now - this.CreatedDateTime).TotalMilliseconds <= 100) return;
    var length = System.Console.WindowWidth - (14 + this.ProgressPrompt.Length + (this.ProgressPrompt.Length > 0 ? 1 : 0));
    var rounded = (int)Math.Ceiling(length * (double)progress!);
    System.Console.SetCursorPosition(0, System.Console.CursorTop);
    System.Console.ForegroundColor = ConsoleColor.Blue;
    System.Console.Write("".PadLeft(3, ' '));
    System.Console.Write($"""{this.ProgressPrompt} """);
    System.Console.Write("".PadLeft(rounded, '#'));
    System.Console.ForegroundColor = ConsoleColor.DarkBlue;
    System.Console.Write("".PadLeft(length - rounded, '.'));
    System.Console.Write(" ");
    System.Console.ForegroundColor = ConsoleColor.Blue;
    System.Console.Write((Math.Ceiling(10000 * (double)progress!) / 100).ToString("N2").PadLeft(6, ' '));
    System.Console.Write("%");
    System.Console.ResetColor();
  }
  public void ClearProgress () {
    if (this.SuppressProgress) return;
    System.Console.SetCursorPosition(0, System.Console.CursorTop);
    System.Console.Write("".PadLeft(System.Console.WindowWidth - 1, ' '));
    System.Console.SetCursorPosition(0, System.Console.CursorTop);
  }

  #endregion

  #region Write utility
    private bool writePaddingAlreadyWritten = false;

    /// <summary>
    /// Checks if a logging level is allowed to write to the log
    /// </summary>
    /// <param name="level">Logging level to check</param>
    /// <returns>If a logging level is allowed to write to the log</returns>
    public bool CheckLogLevel (ConsoleLoggingLevel level) {
      return level <= this.LoggingLevel;
    }

  #endregion

  #region Write

    public void Write(char value, ConsoleWriteOptions options) {
      this.Write(value, ConsoleLoggingLevel.Verbose, options);
    }
    public void Write(char value, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, ConsoleWriteOptions? options = null) {
      if (this.SuppressWrite) return;
      if (!this.CheckLogLevel(level)) return;
      try { this.ClearProgress(); } catch {};
      if (!this.writePaddingAlreadyWritten) {
        System.Console.Write("".PadLeft(this.Padding, ' '));
        writePaddingAlreadyWritten = true;
      }
      System.Console.ForegroundColor = (options != null ? (options.ForegroundColor ?? this.FgColor) : this.FgColor) ?? System.Console.ForegroundColor;
      System.Console.BackgroundColor = (options != null ? (options.BackgroundColor ?? this.BgColor) : this.BgColor) ?? System.Console.BackgroundColor;
      System.Console.Write(value);
      System.Console.ResetColor();
    }

    public void Write(string value, ConsoleWriteOptions options) {
      this.Write(value, ConsoleLoggingLevel.Verbose, options);
    }
    public void Write(string value, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, ConsoleWriteOptions? options = null) {
      if (this.SuppressWrite) return;
      if (!this.CheckLogLevel(level)) return;
      try { this.ClearProgress(); } catch {};
      if (!this.writePaddingAlreadyWritten) {
        System.Console.Write("".PadLeft(this.Padding, ' '));
        writePaddingAlreadyWritten = true;
      }
      System.Console.ForegroundColor = (options != null ? (options.ForegroundColor ?? this.FgColor) : this.FgColor) ?? System.Console.ForegroundColor;
      System.Console.BackgroundColor = (options != null ? (options.BackgroundColor ?? this.BgColor) : this.BgColor) ?? System.Console.BackgroundColor;
      System.Console.Write(value);
      System.Console.ResetColor();
    }

  #endregion

  #region WriteLine
  public void WriteLine(ConsoleWriteOptions options) {
    this.WriteLine(ConsoleLoggingLevel.Verbose, options);
  }
  public void WriteLine(ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, ConsoleWriteOptions? options = null) {
    if (this.SuppressWrite) return;
    if (!this.CheckLogLevel(level)) return;
    try { this.ClearProgress(); } catch {};
    System.Console.Write("".PadLeft(this.Padding, ' '));
      System.Console.ForegroundColor = (options != null ? (options.ForegroundColor ?? this.FgColor) : this.FgColor) ?? System.Console.ForegroundColor;
      System.Console.BackgroundColor = (options != null ? (options.BackgroundColor ?? this.BgColor) : this.BgColor) ?? System.Console.BackgroundColor;
    System.Console.WriteLine();
    System.Console.ResetColor();
    writePaddingAlreadyWritten = false;
  }

  public void WriteLine(string? value, ConsoleWriteOptions options) {
    this.WriteLine(value, ConsoleLoggingLevel.Verbose, options);
  }
  public void WriteLine(string? value, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, ConsoleWriteOptions? options = null) {
    if (this.SuppressWrite) return;
    if (!this.CheckLogLevel(level)) return;
    try { this.ClearProgress(); } catch {};
    System.Console.Write("".PadLeft(this.Padding, ' '));
      System.Console.ForegroundColor = (options != null ? (options.ForegroundColor ?? this.FgColor) : this.FgColor) ?? System.Console.ForegroundColor;
      System.Console.BackgroundColor = (options != null ? (options.BackgroundColor ?? this.BgColor) : this.BgColor) ?? System.Console.BackgroundColor;
    System.Console.WriteLine(value);
    System.Console.ResetColor();
    writePaddingAlreadyWritten = false;
  }

  public void WriteLine([StringSyntax("CompositeFormat")] string format, object? arg0, ConsoleWriteOptions options) {
    this.WriteLine(format, arg0, ConsoleLoggingLevel.Verbose, options);
  }
  public void WriteLine([StringSyntax("CompositeFormat")] string format, object? arg0, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, ConsoleWriteOptions? options = null) {
    if (this.SuppressWrite) return;
    if (!this.CheckLogLevel(level)) return;
    try { this.ClearProgress(); } catch {};
    System.Console.Write("".PadLeft(this.Padding, ' '));
      System.Console.ForegroundColor = (options != null ? (options.ForegroundColor ?? this.FgColor) : this.FgColor) ?? System.Console.ForegroundColor;
      System.Console.BackgroundColor = (options != null ? (options.BackgroundColor ?? this.BgColor) : this.BgColor) ?? System.Console.BackgroundColor;
    System.Console.WriteLine(format, arg0);
    System.Console.ResetColor();
    writePaddingAlreadyWritten = false;
  }

  public void WriteLine([StringSyntax("CompositeFormat")] string format, object? arg0, object? arg1, ConsoleWriteOptions options) {
    this.WriteLine(format, arg0, arg1, ConsoleLoggingLevel.Verbose, options);
  }
  public void WriteLine([StringSyntax("CompositeFormat")] string format, object? arg0, object? arg1, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, ConsoleWriteOptions? options = null) {
    if (this.SuppressWrite) return;
    if (!this.CheckLogLevel(level)) return;
    try { this.ClearProgress(); } catch {};
    System.Console.Write("".PadLeft(this.Padding, ' '));
      System.Console.ForegroundColor = (options != null ? (options.ForegroundColor ?? this.FgColor) : this.FgColor) ?? System.Console.ForegroundColor;
      System.Console.BackgroundColor = (options != null ? (options.BackgroundColor ?? this.BgColor) : this.BgColor) ?? System.Console.BackgroundColor;
    System.Console.WriteLine(format, arg0, arg1);
    System.Console.ResetColor();
    writePaddingAlreadyWritten = false;
  }

  public void WriteLine([StringSyntax("CompositeFormat")] string format, object? arg0, object? arg1, object? arg2, ConsoleWriteOptions options) {
    this.WriteLine(format, arg0, arg1, arg2, ConsoleLoggingLevel.Verbose, options);
  }
  public void WriteLine([StringSyntax("CompositeFormat")] string format, object? arg0, object? arg1, object? arg2, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose, ConsoleWriteOptions? options = null) {
    if (this.SuppressWrite) return;
    if (!this.CheckLogLevel(level)) return;
    try { this.ClearProgress(); } catch {};
    System.Console.Write("".PadLeft(this.Padding, ' '));
      System.Console.ForegroundColor = (options != null ? (options.ForegroundColor ?? this.FgColor) : this.FgColor) ?? System.Console.ForegroundColor;
      System.Console.BackgroundColor = (options != null ? (options.BackgroundColor ?? this.BgColor) : this.BgColor) ?? System.Console.BackgroundColor;
    System.Console.WriteLine(format, arg0, arg1, arg2);
    System.Console.ResetColor();
    writePaddingAlreadyWritten = false;
  }
  
  public void WriteLine([StringSyntax("CompositeFormat")] string format, params object?[]? arg) {
    if (this.SuppressWrite) return;
    try { this.ClearProgress(); } catch {};
    System.Console.Write("".PadLeft(this.Padding, ' '));
    System.Console.ForegroundColor = this.FgColor ?? System.Console.ForegroundColor;
    System.Console.BackgroundColor = this.BgColor ?? System.Console.BackgroundColor;
    System.Console.WriteLine(format, arg);
    System.Console.ResetColor();
    writePaddingAlreadyWritten = false;
  }
  #endregion
}

/// <summary>
/// Console logging levels
/// </summary>
public enum ConsoleLoggingLevel {
  Reduced = 1,
  Verbose = 2,
  All = 9
}

/// <summary>
/// Console write options
/// </summary>
public class ConsoleWriteOptions {

  /// <summary>
  /// Console foreground color
  /// </summary>
  public ConsoleColor? ForegroundColor { init; get; }
  /// <summary>
  /// Console background color
  /// </summary>
  public ConsoleColor? BackgroundColor { init; get; }

}
