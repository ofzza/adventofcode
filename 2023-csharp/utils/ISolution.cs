namespace ofzza.aoc.utils;

/// <summary>
/// Solution interface
/// Any solution implementing this interface will be executable via CLI
/// </summary>
public interface ISolution<TInput, TOutput> where TOutput : struct {
    /// <summary>
    /// Solution year
    /// </summary>
    public int SolutionYear { get; }
    /// <summary>
    /// Solution day
    /// </summary>
    public int SolutionDay { get; }

    /// <summary>
    /// Definitions of available executions for the solution
    /// </summary>
    public List<SolutionExecution<TInput, TOutput>> Executions { get; }

    /// <summary>
    /// Implementation of the solution
    /// </summary>
    /// <param name="info">Solution execution information</param>
    /// <param name="log">Console instance to use for logging
    /// <param name="verbose">If output should be verbose
    /// <param name="obfuscate">If output should obfuscated</param>
    /// <returns>Solution execution output</returns>
    public TOutput Run (SolutionExecutionRunInfo<TInput> info, Console log, bool verbose, bool obfuscate);
}
