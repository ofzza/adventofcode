namespace ofzza.aoc.utils;

/// <summary>
/// Enumerates all primary directions, represented as a binary 4-digit number 0bABCD, where:
/// - "A" is 1 if direction is Top
/// - "B" is 1 if direction is Left
/// - "C" is 1 if direction is Bottom
/// - "D" is 1 if direction is Right
/// </summary>
public enum Direction {
  None = 0b0000,
  Top = 0b1000,
  Bottom = 0b0010,
  Left = 0b0100,
  Right = 0b0001,
  Any = 0b1111,
}

/// <summary>
/// Enumerates all diagonal directions, representing connectivity of each primary direction as a binary 4-digit number 0bABCD, where:
/// - "A" is 1 if tile has connectivity to the tile above it
/// - "B" is 1 if tile has connectivity to the tile left to it
/// - "C" is 1 if tile has connectivity to the tile below it
/// - "D" is 1 if tile has connectivity to the tile right to it
/// </summary>
public enum DiagonalDirection {
  None = 0b0000,
  TopBottom = 0b1010,
  LeftRight = 0b0101,
  TopLeft = 0b1100,
  TopRight = 0b1001,
  BottomLeft = 0b0110,
  BottomRight = 0b0011
}
