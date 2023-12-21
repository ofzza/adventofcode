namespace ofzza.aoc.year2023.utils.pulspropagation;

public class Module {
  public required ModuleType Type { init; get; }
  public required string Name { init; get; }
  public string[] ConnectedModuleNames { init; get; } = new string[] {};
  public Module[] ConnectedModules { get; set; } = new Module[] {};
}

public class BroadcastModule : Module {}

public class FlipFlopModule : Module {
  public bool Memory { get; set; } = false;
}

public class ConjunctionModule : Module {
  public Dictionary<string, SignalType> Memory { get; set; } = new Dictionary<string, SignalType>();
}

public enum ModuleType {
  Generic = 0,
  Broadcaster = 1,
  FlipFlop = 2,
  Conjunction = 3
}

public class Signal {
  public Module? Source { init; get;} = null;
  public required Module Target { init; get;}
  public required SignalType Type { init; get; }
}

public enum SignalType {
  High = 1,
  Low = 0
}
