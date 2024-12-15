package pathing

// Define constant directions
const DirectionUnknown byte = byte(0b00000000)
const DirectionAny byte = byte(0b00001111)
const DirectionHorizontal byte = byte(0b00000101)
const DirectionVertical byte = byte(0b00001010)
const DirectionRight byte = byte(0b00000001)
const DirectionTopRight byte = byte(0b00000011)
const DirectionTop byte = byte(0b00000010)
const DirectionTopLeft byte = byte(0b00000110)
const DirectionLeft byte = byte(0b00000100)
const DirectionBottomLeft byte = byte(0b01001100)
const DirectionBottom byte = byte(0b00001000)
const DirectionBottomRight byte = byte(0b00001001)
