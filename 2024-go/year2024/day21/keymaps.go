package year2024

type KeyMap struct {
	id        string
	keys      map[rune][]int
	forbidden [][]int
}

var numericKeymap = KeyMap{
	id: "numeric",
	keys: map[rune][]int{
		'7': {0, 0},
		'8': {1, 0},
		'9': {2, 0},
		'4': {0, 1},
		'5': {1, 1},
		'6': {2, 1},
		'1': {0, 2},
		'2': {1, 2},
		'3': {2, 2},
		'_': {0, 3},
		'0': {1, 3},
		'A': {2, 3},
	},
	forbidden: [][]int{{0, 3}},
}

var directionalKeymap = KeyMap{
	id: "directional",
	keys: map[rune][]int{
		'_': {0, 0},
		'^': {1, 0},
		'A': {2, 0},
		'<': {0, 1},
		'v': {1, 1},
		'>': {2, 1},
	},
	forbidden: [][]int{{0, 0}},
}
