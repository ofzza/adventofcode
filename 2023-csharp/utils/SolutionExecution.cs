namespace ofzza.aoc.utils;

/// <summary>
/// Solution execution definition
/// </summary>
public class SolutionExecution<TInput, TOutput>: SolutionExecutionRunInfo<TInput> where TOutput : struct {
    /// <summary>
    /// Execution tag
    /// </summary>
    public Tag ExecutionTag { init; get; }

    /// <summary>
    /// Expected output value for execution
    /// </summary>
    public TOutput? Expect { init; get; }

    /// <summary>
    /// Constructor
    /// </summary>
    /// <param name="index">Execution index</param>
    /// <param name="tag">Execution tag</param>
    public SolutionExecution (int index, Tag tag) {
        this.ExecutionIndex = index;
        this.ExecutionTag = tag;
    }

    /// <summary>
    /// Gets run information subsection of execution definition
    /// </summary>
    /// <param name="customInputValue">If custom value is to be used in place of configured value</param>
    /// <param name="value">Custom value to use in place of configured value (only if "customInputValue == true")</param>
    /// <returns>Run information subsection of execution definition</returns>
    public SolutionExecutionRunInfo<TInput> GetRunInfo(bool customInputValue, TInput? value) {
        return new SolutionExecutionRunInfo<TInput>() {
            ExecutionIndex = this.ExecutionIndex,
            InputValue = customInputValue ? value : this.InputValue
        };
    }
}
