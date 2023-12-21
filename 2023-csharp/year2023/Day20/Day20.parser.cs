namespace ofzza.aoc.year2023.day20;

using ofzza.aoc.utils;
using ofzza.aoc.year2023.utils.pulspropagation;

public partial class Day20: ISolution<string, long> {
  private static Module [] parse (string input) {
    // Initialize a module dictionary
    var modules = new Dictionary<string, Module>();
    
    // Parse input into modules
    var parsed = input.Split('\n').Select(l => {
      // Parse a connection rule
      var parsed = l.Split(" -> ");
      var type = parsed[0][0] == '%' ? ModuleType.FlipFlop : parsed[0][0] == '&' ? ModuleType.Conjunction : parsed[0] == "broadcaster" ? ModuleType.Broadcaster : ModuleType.Generic;
      var name = type != ModuleType.Broadcaster ? parsed[0].Substring(1) : parsed[0];
      var connectedNames = parsed[1].Split(',').Select(n => n.Trim()).ToArray();
      // Compose a module and its connections
      Module module = 
        type == ModuleType.Broadcaster ?
          new BroadcastModule() {
            Type = type,
            Name = name,
            ConnectedModuleNames = connectedNames
          }
        : type == ModuleType.FlipFlop ? 
          new FlipFlopModule() {
            Type = type,
            Name = name,
            ConnectedModuleNames = connectedNames,
            Memory = false
          }
        : type == ModuleType.Conjunction ? 
          new ConjunctionModule() {
            Type = type,
            Name = name,
            ConnectedModuleNames = connectedNames
          }
        : new Module() {
            Type = type,
            Name = name,
            ConnectedModuleNames = connectedNames
          };
      modules.Add(name, module);
      return module;
    }).ToArray();
    
    // Interconnect modules
    foreach (var source in parsed) {
      var targets = new List<Module>();
      foreach (var targetName in source.ConnectedModuleNames) {
        if (modules.ContainsKey(targetName)) {
          targets.Add(modules[targetName]);
        } else {
          var module = new Module() {
            Name = targetName,
            Type = targetName == "broadcaster" ? ModuleType.Broadcaster : ModuleType.Generic
          };
          modules.Add(targetName, module);
          targets.Add(module);
        }
      }
      source.ConnectedModules = targets.ToArray();
    }

    // Initialize conjunction module memory
    foreach (var source in parsed) {
      foreach (var target in source.ConnectedModules.Where(m => m.Type == ModuleType.Conjunction)) {
        ((ConjunctionModule)target).Memory.Add(source.Name, SignalType.Low);
      }
    }
    
    // Return parsed result
    return parsed;
  }
}
