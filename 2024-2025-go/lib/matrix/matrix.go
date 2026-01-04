package matrix

import (
	"adventofcode/lib/numbers"
	"fmt"
	"slices"
)

// Matrix type
type Matrix struct {
	Rank MatrixRank
	Values [][]numbers.RationalNumber
}

// Matrix rank (width and height)
type MatrixRank struct {
	Width int
	Height int
}

type MatrixGaussianEliminationStatus struct {
	VariablesReduced int
	EquationsReduced int
	Done bool
}

// Removes indexed rows (in place)
func (matrix *Matrix) RemoveRows(indexes []int) {
	var rows = [][]numbers.RationalNumber{}
	for i:=0; i<matrix.Rank.Height; i++ {
		if !slices.Contains(indexes, i) {
			rows = append(rows, matrix.Values[i])
		} 
	}
	matrix.Rank = MatrixRank{ Width: matrix.Rank.Width, Height: matrix.Rank.Height - len(indexes) }
	matrix.Values = rows
	
}

// Removes indexed columns (in place)
func (matrix *Matrix) RemoveColumns(indexes []int) {
	var rows = [][]numbers.RationalNumber{}
	for i:=0; i<matrix.Rank.Height; i++ {
		var row = []numbers.RationalNumber{}
		for j:=0; j<matrix.Rank.Width; i++ {
			if !slices.Contains(indexes, j) {
				row = append(row, matrix.Values[i][j])
			}
		}
		rows = append(rows, row)
	}
	matrix.Rank = MatrixRank{ Width: matrix.Rank.Width, Height: matrix.Rank.Height - len(indexes) }
	matrix.Values = rows
}

// Clones the matrix
func (matrix Matrix) Clone() Matrix {
	var rows = make([][]numbers.RationalNumber, len(matrix.Values))
	for i:=0; i<len(rows); i++ {
		rows[i] = make([]numbers.RationalNumber, len(matrix.Values[i]))
		copy(rows[i], matrix.Values[i])
	}
	var next = Matrix{ Rank: MatrixRank{ Width: matrix.Rank.Width, Height: matrix.Rank.Height }, Values: rows }
	return next
}

// (ToString) Outputs a string representation of the matrix
func (matrix *Matrix) ToString(padding int) string {
	var output = ""

	output += "     "
	for i:=0; i<matrix.Rank.Width-1; i++ {
		var divider = ""
		if i > 0 { divider = ", " }
		var outputFmt = fmt.Sprintf("%ss%s%dd", "%", "%", padding+3);
		output += fmt.Sprintf(outputFmt, divider, i)
	}
	output += "\n"

	for i, row := range matrix.Values {
		output += fmt.Sprintf("%2d | ", i)
		for i, value := range row {
			var divider = ""
			if i > 0 { divider = ", " }
			var outputFmt = fmt.Sprintf("%ss%s%d.2f", "%", "%", padding+3);
			output += fmt.Sprintf(outputFmt, divider, value.ToFloat())
		}
		output += " |\n"
	}

	return output
}
