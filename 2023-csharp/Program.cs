namespace ofzza.aoc;

using McMaster.Extensions.CommandLineUtils;
using System.Collections.Generic;
using System.Diagnostics;
using System.Reflection;
using ofzza.aoc.utils;

public class Program
{
    [Option("-y|--year", CommandOptionType.SingleValue)]
    public int? FilteringYear { get; } = null;

    [Option("-d|--day", CommandOptionType.SingleValue)]
    public int? FilteringDay { get; } = null;

    [Option("-i|--index", CommandOptionType.SingleValue)]
    public int? FilteringIndex { get; } = null;

    [Option("-t|--tag", CommandOptionType.SingleValue)]
    public string? FilteringTag { get; } = null;

    [Option("--input-value", CommandOptionType.SingleValue)]
    public string? InputValue { get; } = null;

    [Option("--input-file", CommandOptionType.SingleValue)]
    public string? InputFile { get; } = null;

    [Option("--expect", CommandOptionType.SingleValue)]
    public string? Expect { get; } = null;

    [Option("--repeat", CommandOptionType.SingleValue)]
    public int Repeat { get; } = 1;

    [Option("--verbose", CommandOptionType.SingleOrNoValue)]
    public int? Verbose { get; } = 0;

    [Option("--obfuscate", CommandOptionType.NoValue)]
    public bool Obfuscate { get; } = false;

    [Option("--progress", CommandOptionType.NoValue)]
    public bool Progress { get; } = false;

    [Option("--summary", CommandOptionType.NoValue)]
    public bool Summary { get; } = false;

    public static int Main(string[] args) => CommandLineApplication.Execute<Program>(args);

    /// <summary>
    /// CLI entry point, after arguments parsing
    /// </summary>
    private void OnExecute() {
        // Find all available solution
        var solutionTypes = Assembly.GetExecutingAssembly().GetTypes()
            .Where(t => t.GetInterfaces().Any(i => i.IsGenericType && i.GetGenericTypeDefinition() == typeof(ISolution<,>)));
        // Instantiate all available types and sort by year/day
        var typesDictionary = new SortedDictionary<int, Type>();
        foreach (Type solutionType in solutionTypes) {
            // Instantiate a solution
            var solution = Activator.CreateInstance(solutionType);
            // Get solution year and day
            var yearGetter = solutionType.GetProperty("SolutionYear")!.GetAccessors().First(m => m.ReturnType != typeof(void));
            var year = (int)yearGetter.Invoke(solution, new object[] { })!;
            var yearDay = solutionType.GetProperty("SolutionDay")!.GetAccessors().First(m => m.ReturnType != typeof(void));
            var day = (int)yearDay.Invoke(solution, new object[] { })!;
            // Set solution type by year-day
            typesDictionary.Add(1000 * year + day, solutionType);
        }
        // Initialize summary
        var successByTag = new Dictionary<string, long>() { { "*", 0 } };
        var failByTag = new Dictionary<string, long>() { { "*", 0 } };
        var unknownByTag = new Dictionary<string, long>() { { "*", 0 } };
        var timeByTag = new Dictionary<string, long>() { { "*", 0 } };
        foreach (var tag in Enum.GetValues(typeof(Tag))) {
            successByTag.Add(tag.ToString()!.ToLower(), 0);
            failByTag.Add(tag.ToString()!.ToLower(), 0);
            unknownByTag.Add(tag.ToString()!.ToLower(), 0);
            timeByTag.Add(tag.ToString()!.ToLower(), 0);
        }
        // Process all available types
        foreach (Type solutionType in typesDictionary.Values) {
            // Detect generic definition
            var iface = solutionType.GetInterfaces().First(i => i.IsGenericType && i.GetGenericTypeDefinition() == typeof(ISolution<,>));
            var ifaceArguments = iface.GetGenericArguments();
            var typeInput = ifaceArguments[0];
            var typeOutput = ifaceArguments[0];
            // Instantiate a solution
            var solution = Activator.CreateInstance(solutionType);
            // Get solution executions and other information
            var yearGetter = solutionType.GetProperty("SolutionYear")!.GetAccessors().First(m => m.ReturnType != typeof(void));
            var year = (int)yearGetter.Invoke(solution, new object[] { })!;
            var yearDay = solutionType.GetProperty("SolutionDay")!.GetAccessors().First(m => m.ReturnType != typeof(void));
            var day = (int)yearDay.Invoke(solution, new object[] { })!;
            var executionsGetter = solutionType.GetProperty("Executions")!.GetAccessors().First(m => m.ReturnType != typeof(void));
            var executions = (IEnumerable<object>)executionsGetter.Invoke(solution, new object[] { })!;
            var runMethod = solutionType.GetMethod("Run");
            // Check if year or day filtered
            if (this.FilteringYear != null && this.FilteringYear != year) continue;
            if (this.FilteringDay != null && this.FilteringDay != day) continue;
            // Filter executions
            foreach (var execution in executions) {
                // Get execution information
                var executionType = execution.GetType();
                var indexGetter = executionType.GetProperty("ExecutionIndex")!.GetAccessors().First(m => m.ReturnType != typeof(void));
                var index = (int)indexGetter.Invoke(execution, new object[] { })!;
                var tagGetter = executionType.GetProperty("ExecutionTag")!.GetAccessors().First(m => m.ReturnType != typeof(void));
                var tag = Enum.GetName(typeof(Tag), (Tag)tagGetter.Invoke(execution, new object[] { })!);
                // Check if index or tag filtered
                if (this.FilteringIndex != null && this.FilteringIndex != index) continue;
                if (this.FilteringTag != null && this.FilteringTag?.ToLower() != tag?.ToLower()) continue;
                // Get execution run arguments
                var getRunInfoMethod = executionType.GetMethod("GetRunInfo");
                var infoValue = new object[] { false, null! };
                if (this.InputValue != null) infoValue = new object[] { true, this.InputValue };
                if (this.InputFile != null) infoValue = new object[] { true, File.ReadAllText(this.InputFile).Trim() };
                var info = getRunInfoMethod!.Invoke(execution, infoValue)!;
                var expectGetter = executionType.GetProperty("Expect")!.GetAccessors().First(m => m.ReturnType != typeof(void));
                var expect = expectGetter.Invoke(execution, new object[] { });
                // Run execution
                var repeatString = this.Repeat > 1 ? $""" (X{this.Repeat})""" : "";
                Console.Create(ConsoleColor.Gray).WriteLine($"""➡️ Year {year}, Day {day}, Index {index}, Tag {tag}{repeatString}:""");
                object? output = null;
                var executionSW = new Stopwatch();
                for (var i=0; i<this.Repeat; i++) {
                    var verbose = this.Verbose == null ? (int)ConsoleLoggingLevel.Verbose : this.Verbose;
                    var console = new Console() {
                        ProgressPrompt = this.Repeat > 1 ? $"""{i + 1}/{this.Repeat}""" : "",
                        FgColor = ConsoleColor.DarkGray,
                        Padding = 4,
                        SuppressWrite = verbose <= 0,
                        LoggingLevel = (ConsoleLoggingLevel)verbose,
                        SuppressProgress = !this.Progress
                    };
                    executionSW.Start();
                    output = runMethod!.Invoke(solution, new object[] { info, console, true, true })!;
                    executionSW.Stop();
                }
                // Evaluate output
                var outputString = this.Obfuscate ? "".PadLeft(output?.ToString()?.Length ?? 0, '#') : output?.ToString();
                var expectString = this.Obfuscate ? "".PadLeft(expect?.ToString()?.Length ?? 0, '#') : expect?.ToString();
                if (expect != null && object.Equals(output, expect))
                {
                    Console.Create(ConsoleColor.Green).WriteLine($"""   ✅ {outputString} (In {(executionSW.Elapsed.TotalNanoseconds / (1e6 * this.Repeat)).ToString("N2")}ms)""");
                    successByTag[tag?.ToLower()!] += 1;
                    successByTag["*"] += 1;
                    timeByTag[tag?.ToLower()!] += (long)(executionSW.Elapsed.TotalNanoseconds / (this.Repeat));
                    timeByTag["*"] += (long)(executionSW.Elapsed.TotalNanoseconds / (this.Repeat));
                }
                else if (expect != null && !object.Equals(output, expect)) {
                    Console.Create(ConsoleColor.Red).WriteLine($"""   ❌ Output {outputString} (In {(executionSW.Elapsed.TotalNanoseconds / (1e6 * this.Repeat)).ToString("N2")}ms, Expected {expectString})""");
                    failByTag[tag?.ToLower()!] += 1;
                    failByTag["*"] += 1;
                    timeByTag[tag?.ToLower()!] += (long)(executionSW.Elapsed.TotalNanoseconds / (this.Repeat));
                    timeByTag["*"] += (long)(executionSW.Elapsed.TotalNanoseconds / (this.Repeat));
                } else {
                    Console.Create(ConsoleColor.Yellow).WriteLine($"""   ❔ Output {outputString} (In {(executionSW.Elapsed.TotalNanoseconds / (1e6 * this.Repeat)).ToString("N2")}ms)""");
                    unknownByTag[tag?.ToLower()!] += 1;
                    unknownByTag["*"] += 1;
                    timeByTag[tag?.ToLower()!] += (long)(executionSW.Elapsed.TotalNanoseconds / (this.Repeat));
                    timeByTag["*"] += (long)(executionSW.Elapsed.TotalNanoseconds / (this.Repeat));
                }
            }
        }
        // Printout summary
        if (this.Summary) {
            var console = Console.Create(ConsoleColor.White);
            console.WriteLine();
            console.WriteLine("--- SUMMARY ---");
            console.WriteLine();
            console.WriteLine($"""- ✅ Successful executions: {successByTag["*"]}/{successByTag["*"] + failByTag["*"] + unknownByTag["*"]}""");
            foreach (var tag in Enum.GetValues(typeof(Tag))) {
                console.WriteLine($"""   - {tag,-8}: {successByTag[tag.ToString()!.ToLower()]}/{successByTag[tag.ToString()!.ToLower()] + failByTag[tag.ToString()!.ToLower()]}""");
            }
            if (unknownByTag["*"] > 0) {
                console.WriteLine();
                console.WriteLine($"""- ❔ Unknown executions: {unknownByTag["*"]}/{successByTag["*"] + failByTag["*"] + unknownByTag["*"]}""");
                foreach (var tag in Enum.GetValues(typeof(Tag))) {
                    console.WriteLine($"""   - {tag,-8}: {unknownByTag[tag.ToString()!.ToLower()]}/{successByTag[tag.ToString()!.ToLower()] + failByTag[tag.ToString()!.ToLower()]}""");
                }
            }
            if (failByTag["*"] > 0) {
                console.WriteLine();
                console.WriteLine($"""- ❌ Failed executions: {failByTag["*"]}/{successByTag["*"] + failByTag["*"] + unknownByTag["*"]}""");
                foreach (var tag in Enum.GetValues(typeof(Tag))) {
                    console.WriteLine($"""   - {tag,-8}: {failByTag[tag.ToString()!.ToLower()]}/{successByTag[tag.ToString()!.ToLower()] + failByTag[tag.ToString()!.ToLower()]}""");
                }
            }
            console.WriteLine();
            console.WriteLine($"""- 🕐 Execution time: {(timeByTag["*"] / (1e6 * this.Repeat)).ToString("N2")}ms""");
            foreach (var tag in Enum.GetValues(typeof(Tag))) {
                console.WriteLine($"""   - {tag,-8}: {(timeByTag[tag.ToString()!.ToLower()] / (1e6 * this.Repeat)).ToString("N2")}ms""");
            }
        }
    }
}
