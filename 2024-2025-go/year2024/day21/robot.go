package year2024

import (
	"fmt"
	"strings"
)

type Robot struct {
	key    rune
	keymap KeyMap
}

func (robot *Robot) pathToNextKey(nextKey rune) ([]string, error) {
	// Initialize partial sequences
	var partials = make([]string, 0)

	// Find all partials from key to key
	var current, okCurrent = robot.keymap.keys[robot.key]
	if !okCurrent {
		return []string{}, fmt.Errorf("failed finding '%v' key", robot.key)
	}
	var target, okTarget = robot.keymap.keys[nextKey]
	if !okTarget {
		return []string{}, fmt.Errorf("failed finding '%v' key", nextKey)
	}
	// If NULL path, just hit current key
	if current[0] == target[0] && current[1] == target[1] {
		// Add hitting 'A' to all sequences
		partials = append(partials, "A")
	} else
	// If not NULL path, process path options
	{
		// Try different midpoints
		var midpoints [][]int
		if current[0] == target[0] || current[1] == target[1] {
			midpoints = [][]int{{target[0], current[0]}}
		} else {
			midpoints = [][]int{{target[0], current[1]}, {current[0], target[1]}}
		}
		for _, midpoint := range midpoints {
			// Check if path midpoint is not forbidden
			var midpointValid = true
			for _, forbidden := range robot.keymap.forbidden {
				if midpoint[0] == forbidden[0] && midpoint[1] == forbidden[1] {
					midpointValid = false
					break
				}
			}
			// If path is valid, generate commands
			if midpointValid {
				// Initialize partial sequence
				var partial = ""
				// Generate partial sequence (part #1)
				var paths [][][]int
				if current[0] == target[0] || current[1] == target[1] {
					paths = [][][]int{{current, target}}
				} else {
					paths = [][][]int{{current, midpoint}, {midpoint, target}}
				}
				for _, path := range paths {
					// Check if NULL path
					if path[0][0] == path[1][0] && path[0][1] == path[1][1] {
						continue
					}
					// If horizontal move right
					if path[0][0] < path[1][0] {
						partial += strings.Replace(fmt.Sprintf("%*s", path[1][0]-path[0][0], ""), " ", ">", -1)
					} else
					// If horizontal move left
					if path[0][0] > path[1][0] {
						partial += strings.Replace(fmt.Sprintf("%*s", path[0][0]-path[1][0], ""), " ", "<", -1)
					} else
					// If vertical move down
					if path[0][1] < path[1][1] {
						partial += strings.Replace(fmt.Sprintf("%*s", path[1][1]-path[0][1], ""), " ", "v", -1)
					} else
					// If vertical move up
					if path[0][1] > path[1][1] {
						partial += strings.Replace(fmt.Sprintf("%*s", path[0][1]-path[1][1], ""), " ", "^", -1)
					}
				}
				// Store partial sequence
				partials = append(partials, partial+"A")
			}
		}
	}

	// Set next key
	robot.key = nextKey

	// Return partial sequences
	return partials, nil
}
