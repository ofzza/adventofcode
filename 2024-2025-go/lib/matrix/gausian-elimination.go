package matrix

import "adventofcode/lib/numbers"

// (PerformGaussianElimination) Reduces the matrix using GaussianElimination
func (matrix *Matrix) PerformGaussianEliminationStep(status *MatrixGaussianEliminationStatus) {
	var equations = matrix.Rank.Height

	// Check if no more variables to reduce
	if status.VariablesReduced >= matrix.Rank.Width - 1 {
		status.Done = true;
		return
	}

	// Find values to reduce
	var reducingEquationIndex = -1
	var remainingEquationIndex = -1
	for equation:=0; equation<equations; equation++ {
		if matrix.Values[equation][status.VariablesReduced].Numerator != 0 {
			if reducingEquationIndex == -1 {
				reducingEquationIndex = equation
			} else {
				remainingEquationIndex = equation
			}
		}
	}

	// If not values for the variable, done
	if reducingEquationIndex == -1 {
		// Reduction complete
		status.VariablesReduced++
		return
	}

	// If remaining equation is already reduce and can't be touched
	if remainingEquationIndex != -1 && remainingEquationIndex < status.EquationsReduced {
		// Reduction complete
		status.VariablesReduced++
		return
	}

	// If not value to reduce with, done
	if remainingEquationIndex == -1 {
		// Make sure leading variable is positive
		if matrix.Values[reducingEquationIndex][status.VariablesReduced].ToFloat() != 1 {
			var factor = matrix.Values[reducingEquationIndex][status.VariablesReduced].Invert()
			for i:=0; i<matrix.Rank.Width; i++ {
				matrix.Values[reducingEquationIndex][i] = matrix.Values[reducingEquationIndex][i].Multiply(factor)
			}
		}
		// Reorder to top (available) position
		matrix._gaussianEliminationSortEquation(reducingEquationIndex, *status)
		// Reduction complete
		status.VariablesReduced++
		status.EquationsReduced++
		return
	}

	// If values found, reduce
	var factor = numbers.ToRationalNumber(-1).Multiply(matrix.Values[reducingEquationIndex][status.VariablesReduced].Divide(matrix.Values[remainingEquationIndex][status.VariablesReduced]))
	for i:=0; i<matrix.Rank.Width; i++ {
		matrix.Values[reducingEquationIndex][i] = matrix.Values[reducingEquationIndex][i].Add(matrix.Values[remainingEquationIndex][i].Multiply(factor))
	}

	// Remove empty rows	
	var empty = []int{}
	for i := range matrix.Values {
		var rowEmpty = true
		for _, value := range matrix.Values[i] {
			if value.ToFloat() != 0 {
				rowEmpty = false;
				break;
			}
		}
		if rowEmpty { empty = append(empty, i )}
	}
	matrix.RemoveRows(empty)

	// Normalize values
	for i:=0; i<len(matrix.Values); i++ {
		for j:=0; j<len(matrix.Values[i]); j++ {
			matrix.Values[i][j].Reduce()
		}
	}
}

// (_gaussianEliminationSortEquation) Reorders equations so that the requested equation is on the top possible position
func (matrix *Matrix) _gaussianEliminationSortEquation(equation int, status MatrixGaussianEliminationStatus) {
	var top = matrix.Values[status.EquationsReduced]
	matrix.Values[status.EquationsReduced] = matrix.Values[equation]
	matrix.Values[equation] = top
}
