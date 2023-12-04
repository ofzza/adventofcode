namespace ofzza.aoc.year2023.day03;

public class Value {
  public ValueType Type { init; get; }
  public PartSerialNumber? SerialNumber { init; get; }
  public char? Part { init; get; }  
}

public class PartSerialNumber {
  public int Number { init; get; }
}

public enum ValueType {
  Empty,
  SerialNumber,
  Part
}
