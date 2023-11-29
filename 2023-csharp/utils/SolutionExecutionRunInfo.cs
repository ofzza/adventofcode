namespace ofzza.aoc.utils;

public class SolutionExecutionRunInfo<TInput> {
    /// <summary>
    /// Explicit value to be used as input
    /// </summary>
    public TInput? InputValue { init; get; }
    /// <summary>
    /// Execution index
    /// </summary>
    public int ExecutionIndex { init; get; }
}
