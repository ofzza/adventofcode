package year2024

import (
	solution "adventofcode/lib"
	"adventofcode/lib/matrix"
	"adventofcode/lib/pathing"
	"errors"
	"fmt"
	"os"
	"strings"
)

// Day one definition
type Day16 struct{}

var Day = Day16{}

// Year and day
func (day Day16) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  16,
	}
}

// Executions
func (day Day16) GetExecutions(index int, tag string) []solution.SolutionExecution {
	var executions = []solution.SolutionExecution{}
	// Part 1/2
	if index == 0 || index == 1 {
		// Test
		if tag == "" || tag == "test" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "test",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day16/input-test-01.txt"); return string(b) }(),
					Expect: 7036,
				},
			)
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "test",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day16/input-test-02.txt"); return string(b) }(),
					Expect: 11048,
				},
			)
		}
		// Solution
		if tag == "" || tag == "solution" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  1,
					Tag:    "solution",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day16/input.txt"); return string(b) }(),
					Expect: 104516,
				},
			)
		}
	}
	// Part 2/2
	if index == 0 || index == 2 {
		// Test
		if tag == "" || tag == "test" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  2,
					Tag:    "test",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day16/input-test-01.txt"); return string(b) }(),
					Expect: 45,
				},
			)
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  2,
					Tag:    "test",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day16/input-test-02.txt"); return string(b) }(),
					Expect: 64,
				},
			)
		}
		// Solution
		if tag == "" || tag == "solution" {
			executions = append(
				executions,
				solution.SolutionExecution{
					Index:  2,
					Tag:    "solution",
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day16/input.txt"); return string(b) }(),
					Expect: 545,
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day16) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var lines = strings.Split(strings.Trim(value, " \n"), "\n")
	var width int = len(lines[0])
	var height int = len(lines)
	var start = make([]int, 0, 2)
	var end = make([]int, 0, 2)
	var labyrinth = make([]rune, 0, width*height)
	var indexer = matrix.CreateIndexer([]int{width, height})
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			if lines[y][x] == 'S' {
				start = []int{x, y}
				labyrinth = append(labyrinth, rune('.'))
			} else if lines[y][x] == 'E' {
				end = []int{x, y}
				labyrinth = append(labyrinth, rune('.'))
			} else {
				labyrinth = append(labyrinth, rune(lines[y][x]))
			}
		}
	}
	var reindeer = Step{coords: start, direction: pathing.DirectionRight}

	// Part 1/2
	if index == 1 {

		// Traverse the labyrinth
		var winningScore int = -1
		traverseLabyrinth(labyrinth, indexer, reindeer, end, []Step{}, make(map[int]int, 0), -1, func(path []Step) {
			// Store score
			if winningScore == -1 || path[len(path)-1].score < winningScore {
				winningScore = path[len(path)-1].score
			}
			// Echo score and path
			if verbose {
				output += fmt.Sprintf("Ending score: %v\n", path[len(path)-1].score)
				output += fmt.Sprintf("%v\n", echoLabyrinth(labyrinth, indexer, path[len(path)-1], start, end, path))
			}
		})

		// Echo best found score and path
		if verbose {
			output += fmt.Sprintf("- Best score: %v\n", winningScore)
		}

		// Return solution
		return winningScore, output, nil
	} else

	// Part 2/2
	if index == 2 {

		// Traverse the labyrinth to find winning score
		var winningScore int = -1
		traverseLabyrinth(labyrinth, indexer, reindeer, end, []Step{}, make(map[int]int, 0), -1, func(path []Step) {
			// Store score
			if winningScore == -1 || path[len(path)-1].score < winningScore {
				winningScore = path[len(path)-1].score
			}
			// Echo score and path
			if verbose {
				output += fmt.Sprintf("Ending score: %v\n", path[len(path)-1].score)
				output += fmt.Sprintf("%v\n", echoLabyrinth(labyrinth, indexer, path[len(path)-1], start, end, path))
			}
		})

		// Traverse the labyrinth again, limited by winning score
		var winningPaths = make([][]Step, 0)
		traverseLabyrinth(labyrinth, indexer, reindeer, end, []Step{}, make(map[int]int, 0), winningScore, func(path []Step) {
			// Store score
			if winningScore == -1 || path[len(path)-1].score < winningScore {
				winningScore = path[len(path)-1].score
				var winningPath = append(make([]Step, 0, len(path)), path...)
				winningPaths = append([][]Step{}, winningPath)
			} else if winningScore != -1 && path[len(path)-1].score == winningScore {
				var winningPath = append(make([]Step, 0, len(path)), path...)
				winningPaths = append(winningPaths, winningPath)
			}
			// Echo score and path
			if verbose {
				output += fmt.Sprintf("Ending score: %v\n", path[len(path)-1].score)
				output += fmt.Sprintf("%v\n", echoLabyrinth(labyrinth, indexer, path[len(path)-1], start, end, path))
			}
		})

		// Echo best found score and path
		if verbose {
			output += fmt.Sprintf("- Best score: %v\n", winningScore)
			output += fmt.Sprintf("- %v winning paths:\n", len(winningPaths))
			for _, winningPath := range winningPaths {
				output += fmt.Sprintf("%v\n\n", echoLabyrinth(labyrinth, indexer, winningPath[len(winningPath)-1], start, end, winningPath))
			}
		}

		// Count up positions on any of the winning paths
		var dedup = make(map[int]bool, 0)
		for _, winningPath := range winningPaths {
			for _, step := range winningPath {
				var index, _ = indexer.CoordinatesToIndex(step.coords)
				dedup[index] = true
			}
		}
		var count int = 0
		for range dedup {
			count++
		}

		// Return solution
		return count, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

type Step struct {
	coords    []int
	direction byte
	score     int
}

func traverseLabyrinth(labyrinth []rune, indexer matrix.MatrixIndexer, reindeer Step, end []int, path []Step, history map[int]int, maxScoreLimitForSearchOfAllWinningPaths int, callback func(path []Step)) (bool, []Step) {
	// Get unique reindeer state id
	var reindeerIndex, _ = indexer.CoordinatesToIndex(reindeer.coords)
	var reindeerUniqueId = reindeerIndex * 10
	if reindeer.direction == pathing.DirectionRight {
		reindeerUniqueId += 1
	} else if reindeer.direction == pathing.DirectionLeft {
		reindeerUniqueId += 2
	} else if reindeer.direction == pathing.DirectionTop {
		reindeerUniqueId += 3
	} else if reindeer.direction == pathing.DirectionBottom {
		reindeerUniqueId += 4
	}

	// Check if path longer than longest allowed score
	if maxScoreLimitForSearchOfAllWinningPaths != -1 && reindeer.score > maxScoreLimitForSearchOfAllWinningPaths {
		return false, []Step{}
	}

	// Check if position already visited with lower score
	var bestCurrentPositionScoreSoFar, bestCurrentPositionScoreSoFarOk = history[reindeerUniqueId]
	if bestCurrentPositionScoreSoFarOk && maxScoreLimitForSearchOfAllWinningPaths == -1 && reindeer.score >= bestCurrentPositionScoreSoFar {
		return false, []Step{}
	}
	if bestCurrentPositionScoreSoFarOk && maxScoreLimitForSearchOfAllWinningPaths != -1 && reindeer.score > bestCurrentPositionScoreSoFar {
		return false, []Step{}
	}

	// Add reindeer position to path and history
	history[reindeerUniqueId] = reindeer.score
	var updatedPath = append(path, reindeer)

	// Check if finished
	if reindeer.coords[0] == end[0] && reindeer.coords[1] == end[1] {
		// Execute callback
		callback(updatedPath)
		// Return end path
		return true, updatedPath
	}

	// Initialize best path info
	var bestFinishPath = make([]Step, 0)

	// Try going straight
	var move = make([]int, 0, 2)
	if reindeer.direction == pathing.DirectionRight {
		move = []int{1, 0}
	} else if reindeer.direction == pathing.DirectionLeft {
		move = []int{-1, 0}
	} else if reindeer.direction == pathing.DirectionTop {
		move = []int{0, -1}
	} else if reindeer.direction == pathing.DirectionBottom {
		move = []int{0, 1}
	}
	var straightReindeer = Step{coords: []int{reindeer.coords[0] + move[0], reindeer.coords[1] + move[1]}, direction: reindeer.direction, score: reindeer.score + 1}
	var straightReindeerIndex, straightReindeerIndexOk = indexer.CoordinatesToIndex(straightReindeer.coords)
	if straightReindeerIndexOk == nil && labyrinth[straightReindeerIndex] == '.' {
		var finished, finishedPath = traverseLabyrinth(labyrinth, indexer, straightReindeer, end, updatedPath, history, maxScoreLimitForSearchOfAllWinningPaths, callback)
		if finished && len(finishedPath) != 0 && (len(bestFinishPath) == 0 || finishedPath[len(finishedPath)-1].score < bestFinishPath[len(bestFinishPath)-1].score) {
			bestFinishPath = finishedPath
		}
	}

	// Try turning left
	var turnLeftDirection byte
	if reindeer.direction == pathing.DirectionRight {
		turnLeftDirection = pathing.DirectionTop
	} else if reindeer.direction == pathing.DirectionLeft {
		turnLeftDirection = pathing.DirectionBottom
	} else if reindeer.direction == pathing.DirectionTop {
		turnLeftDirection = pathing.DirectionLeft
	} else if reindeer.direction == pathing.DirectionBottom {
		turnLeftDirection = pathing.DirectionRight
	}
	var turnLeftReindeer = Step{coords: reindeer.coords, direction: turnLeftDirection, score: reindeer.score + 1000}
	{
		var finished, finishedPath = traverseLabyrinth(labyrinth, indexer, turnLeftReindeer, end, updatedPath, history, maxScoreLimitForSearchOfAllWinningPaths, callback)
		if finished && len(finishedPath) != 0 && (len(bestFinishPath) == 0 || finishedPath[len(finishedPath)-1].score < bestFinishPath[len(bestFinishPath)-1].score) {
			bestFinishPath = finishedPath
		}
	}

	// Try turning right
	var turnRightDirection byte
	if reindeer.direction == pathing.DirectionRight {
		turnRightDirection = pathing.DirectionBottom
	} else if reindeer.direction == pathing.DirectionLeft {
		turnRightDirection = pathing.DirectionTop
	} else if reindeer.direction == pathing.DirectionTop {
		turnRightDirection = pathing.DirectionRight
	} else if reindeer.direction == pathing.DirectionBottom {
		turnRightDirection = pathing.DirectionLeft
	}
	var turnRightReindeer = Step{coords: reindeer.coords, direction: turnRightDirection, score: reindeer.score + 1000}
	{
		var finished, finishedPath = traverseLabyrinth(labyrinth, indexer, turnRightReindeer, end, updatedPath, history, maxScoreLimitForSearchOfAllWinningPaths, callback)
		if finished && len(finishedPath) != 0 && (len(bestFinishPath) == 0 || finishedPath[len(finishedPath)-1].score < bestFinishPath[len(bestFinishPath)-1].score) {
			bestFinishPath = finishedPath
		}
	}

	// If path found, return path
	return len(bestFinishPath) > 0, bestFinishPath
}

func echoLabyrinth(labyrinth []rune, indexer matrix.MatrixIndexer, reindeer Step, start []int, end []int, path []Step) string {
	// Initialize output
	var output string = ""
	// Compose warehouse layout
	for y := 0; y < indexer.GetDimensions()[1]; y++ {
		for x := 0; x < indexer.GetDimensions()[0]; x++ {
			// Search for location on the path
			var isPath = false
			for _, s := range path {
				if x == s.coords[0] && y == s.coords[1] {
					isPath = true
					break
				}
			}
			// Check if reindeer
			if x == reindeer.coords[0] && y == reindeer.coords[1] {
				if reindeer.direction == pathing.DirectionRight {
					output += ">"
				} else if reindeer.direction == pathing.DirectionLeft {
					output += "<"
				} else if reindeer.direction == pathing.DirectionTop {
					output += "^"
				} else if reindeer.direction == pathing.DirectionBottom {
					output += "v"
				}
			} else
			// Check if path
			if isPath {
				output += "+"
			} else
			// Check if start
			if x == start[0] && y == start[1] {
				output += "S"
			} else
			// Check if end
			if x == end[0] && y == end[1] {
				output += "E"
			} else
			// Else, output labyrinth content
			{
				var i, _ = indexer.CoordinatesToIndex([]int{x, y})
				output += string(labyrinth[i])
			}
		}
		output += "\n"
	}
	// Return output
	return output
}
