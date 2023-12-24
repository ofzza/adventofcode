namespace ofzza.aoc.year2023.utils.pulspropagation;

using ofzza.aoc.utils;

public class PulsePropagation {

  private Module[] ModuleConfiguration { init; get; }

  public PulsePropagation (Module[] moduleConfiguration) {
    // Store module configuration
    this.ModuleConfiguration = moduleConfiguration;
  }

  /// <summary>
  /// Event triggering every time there is a signal generated in the system
  /// </summary>
  public event Action<Signal>? OnNewSignal;
  /// <summary>
  /// Event triggering every time there is a signal is being processed in the system
  /// </summary>
  public event Action<Signal>? OnProcessingSignal;
  /// <summary>
  /// Event triggering every time there is a signal was processed in the system
  /// </summary>
  public event Action<Signal>? OnProcessedSignal;

  /// <summary>
  /// Triggers initial signal for a module and processes the consequences
  /// </summary>
  /// <param name="targetName">Name of the module to trigger</param>
  /// <param name="signalType">Type of the signal to trigger the module with</param>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  /// <returns>Array of all generated signals</returns>
  public Signal[] TriggerSignal (string targetName, SignalType signalType, Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    return this.TriggerSignal(this.ModuleConfiguration.First(m => m.Name == targetName), signalType, log, level);
  }
  /// <summary>
  /// Triggers initial signal for a module and processes the consequences
  /// </summary>
  /// <param name="targetModule">Module to trigger</param>
  /// <param name="signalType">Type of the signal to trigger the module with</param>
  /// <param name="log">Console instance to use for output</param>
  /// <param name="level">Console logging level to use for output</param>
  /// <returns>Array of all generated signals</returns>
  public Signal[] TriggerSignal (Module targetModule, SignalType signalType, Console log, ConsoleLoggingLevel level = ConsoleLoggingLevel.Verbose) {
    // Initialize signals log and pointer
    var signals = new List<Signal>() { new Signal() { Target = targetModule, Type = signalType }};
    if (this.OnNewSignal != null) this.OnNewSignal(signals[0]);
    // Execute signals
    for (var i=0; i<signals.Count; i++) {
      // Get signal info
      var signal = signals[i];
      var target = signal.Target;
      // Log processing module
      log.WriteLine($"""- Processing: {signal.Source?.Name ?? "button"} -[{signal.Type.ToString()}]-> {signal.Target.Name}""", level);
      // Trigger signal event for processing signal
      if (this.OnProcessingSignal != null) this.OnProcessingSignal(signal);
      // Let target module process incoming signal      
      var additionalSignals =        
          target!.Type == ModuleType.Broadcaster ? this.ProcessIncomingSignal((BroadcastModule)target!, signal)
        : target!.Type == ModuleType.FlipFlop    ? this.ProcessIncomingSignal((FlipFlopModule)target!, signal)
        : target!.Type == ModuleType.Conjunction ? this.ProcessIncomingSignal((ConjunctionModule)target!, signal)
        : this.ProcessIncomingSignal((Module)target, signal);
      // Trigger signal event for processing signal
      if (this.OnProcessedSignal != null) this.OnProcessedSignal(signal);
      // Trigger signal event for each new signal
      foreach (var additionalSignal in additionalSignals) {
        if (this.OnNewSignal != null) this.OnNewSignal(additionalSignal);
      }
      // Log newly created signals
      if (log.CheckLogLevel(level)) {
        foreach (var additionalSignal in additionalSignals) {
          log.WriteLine($"""  - Queuing: {additionalSignal.Source?.Name ?? "button"} -[{additionalSignal.Type.ToString()}]-> {additionalSignal.Target.Name}""", ConsoleLoggingLevel.All);
        }
      }
      // Add newly created signals to processing queue
      signals.AddRange(additionalSignals);
    }
    // Return signals
    return signals.ToArray();
  }

  private Signal[] ProcessIncomingSignal (BroadcastModule module, Signal signal) {
    return this.ProcessIncomingSignal((Module)module, signal);
  }
  private Signal[] ProcessIncomingSignal (FlipFlopModule module, Signal signal) {
    // If high pulse, stay inactive
    if (signal.Type == SignalType.High) return new Signal[] {};
    // Initialize additional signals
    var signals = new Signal[module.ConnectedModules.Length];
    // Toggle state on low pulse
    module.Memory = !module.Memory;
    // Send signals to connected modules
    for (var i=0; i<module.ConnectedModules.Length; i++) {
      signals[i] = new Signal() {
        Source = module,
        Target = module.ConnectedModules[i],
        Type = module.Memory ? SignalType.High : SignalType.Low
      };
    }
    // Return additional signals
    return signals;
  }
  private Signal[] ProcessIncomingSignal (ConjunctionModule module, Signal signal) {
    // Initialize additional signals
    var signals = new Signal[module.ConnectedModules.Length];
    // Memorize received signal per sender
    module.Memory[signal.Source!.Name] = signal.Type;
    // Send signal to connected modules
    var sending = module.Memory.Values.All(m => m == SignalType.High) ? SignalType.Low : SignalType.High;
    for (var i=0; i<module.ConnectedModules.Length; i++) {
      signals[i] = new Signal() {
        Source = module,
        Target = module.ConnectedModules[i],
        Type = sending
      };
    }
    // Return additional signals
    return signals;
  }
  private Signal[] ProcessIncomingSignal (Module module, Signal signal) {
    // Initialize additional signals
    var signals = new Signal[module.ConnectedModules.Length];
    // Forward pulse to all connected modules
    for (var i=0; i<module.ConnectedModules.Length; i++) {
      signals[i] = new Signal() {
        Source = module,
        Target = module.ConnectedModules[i],
        Type = signal.Type
      };
    }
    // Return additional signals
    return signals;
  }

}
