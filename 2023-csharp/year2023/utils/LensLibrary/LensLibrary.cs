namespace ofzza.aoc.year2023.utils.lenslibrary;

public class LensLibrary {
  
  /// <summary>
  /// Generates a unique byte hash of a string
  /// </summary>
  /// <param name="input">String to hash</param>
  /// <returns>Unique byte hash of the input string</returns>
  public static byte CalculateHash(string input) {
    byte hash = 0;
    for (var i=0; i<input.Length; i++) {
      hash = (byte)(hash + (byte)input[i]);
      hash = (byte)(hash + (hash << 4));
    }
    return hash;
  }

  /// <summary>
  /// Holds the initialization sequence
  /// </summary>
  public (string Label, char Operator, byte? FocalLength)[] Sequence { init; get; }

  /// <summary>
  /// Constructor
  /// </summary>
  /// <param name="sequence">Unparsed initialization sequence</param>
  public LensLibrary (string[] sequence) {
    // Parse and store the initialization sequence
    this.Sequence = sequence.Select(l => {
      // Process as "replace" operation
      if (l.EndsWith('-')) {
        return ((string Label, char Operator, byte? FocalLength))(l.Substring(0, l.Length - 1), '-', null);
      }
      // Process as "add" operation
      else {
        var parsed = l.Split('=');
        return ((string Label, char Operator, byte? FocalLength))(parsed[0], '=', byte.Parse(parsed[1]));
      }
    }).ToArray();
  }

  /// <summary>
  /// Performs the initialization sequence
  /// </summary>
  public int RunInitializationSequence () {
    // Initialize
    var boxes = new List<(string Label, byte FocalLength)>[256];

    // Perform the initialization sequence
    foreach (var item in this.Sequence) {
      // Find the targeted box
      var hash = LensLibrary.CalculateHash(item.Label);
      var box = boxes[hash];
      
      // Implement the "remove" operation
      if (box != null && item.Operator == '-') {
        // Search box for lens with same label
        var index = box != null ? box!.FindIndex(l => l.Label == item.Label) : -1;
        if (index != -1) box!.RemoveAt(index);
      }
      // Process the "add" operation
      else if (item.Operator == '=') {
        // Search box for lens with same label
        var index = box != null ? box!.FindIndex(l => l.Label == item.Label) : -1;
        if (index != -1) {
          // Replace lens with same label
          box![index] = (item.Label, (byte)item.FocalLength!);
        } else {
          // Add lens with new label
          if (box == null) { box = boxes[hash] = new List<(string Label, byte FocalLength)>(); }
          box.Add((item.Label, (byte)item.FocalLength!));
        }
      }
    }

    // Evaluate focusing power
    var power = 0;
    for (var i=0; i<boxes.Length; i++) {
      var box = boxes[i];
      if (box != null) for (var j=0; j<box.Count; j++) {
        var lens = box[j];
        power += (1 + i) * (1 + j) * lens.FocalLength;
      }
    }
    return power;
  }
  
}
