package year2024

import (
	solution "adventofcode/lib"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Day one definition
type Day09 struct{}

var Day = Day09{}

// Year and day
func (day Day09) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2024,
		Day:  9,
	}
}

// Executions
func (day Day09) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day09/input-test.txt"); return string(b) }(),
					Expect: 1928,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day09/input.txt"); return string(b) }(),
					Expect: 6288707484810,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day09/input-test.txt"); return string(b) }(),
					Expect: 2858,
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
					Input:  func() string { var b, _ = os.ReadFile("./year2024/data/day09/input.txt"); return string(b) }(),
					Expect: 6311837662089,
				},
			)
		}
	}
	return executions
}

type Block struct {
	id   int
	size int
}

// Implementation
func (day Day09) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var line = strings.Trim(value, " \n")
	var totalSize int = 0
	var diskImage = make([]int, 0, 10*len(line))
	var diskFs = make([]Block, 0, len(line))
	for i, r := range line {
		// Parse and set size
		var size, _ = strconv.Atoi(fmt.Sprintf("%c", r))
		totalSize += size
		// Calculate file id
		var id int
		if i%2 == 0 {
			id = i / 2
		} else {
			id = -1
		}
		// Write disk image
		for j := 0; j < size; j++ {
			diskImage = append(diskImage, id)
		}
		// Write disk FS
		if id != -1 {
			// Write file to FS
			diskFs = append(diskFs, Block{id: id, size: size})
		} else {
			// Write empty space to disk and preallocate single zero-size records for files post compaction
			diskFs = append(diskFs, Block{id: -1, size: size})
			for j := 0; j < size; j++ {
				diskFs = append(diskFs, Block{id: -1, size: 0})
			}
		}
	}

	// Part 1/2
	if index == 1 {

		// Echo disk image
		if verbose {
			log.Log("- Disk: " + echoDiskImage(diskImage))
		}

		// Compact
		var emptyPointer int = 0
		for i := (len(diskImage) - 1); i >= 0; i-- {
			// Check if current position contains a data
			if diskImage[i] == -1 {
				continue
			}
			// Find first empty block
			for j := emptyPointer; j <= i; j++ {
				emptyPointer = j
				if diskImage[j] == -1 {
					break
				}
			}
			// Check if empty space is past currently compacting block pointer
			if emptyPointer >= i {
				break
			}
			// Move block
			diskImage[emptyPointer] = diskImage[i]
			diskImage[i] = -1
			// Echo disk image
			if verbose {
				log.Log("- Disk: " + echoDiskImage(diskImage))
			}
		}

		// Calculate checksum
		var cksum int = 0
		for i := 0; i < len(diskImage); i++ {
			if diskImage[i] != -1 {
				cksum += i * diskImage[i]
			}
		}

		// Return solution
		return cksum, log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Echo disk image
		if verbose {
			log.Log("- FS: " + echoDiskFs(diskFs))
		}

		// Compact
		for i := (len(diskFs) - 1); i >= 0; i-- {
			// Check if current position contains a data
			if diskFs[i].id == -1 {
				continue
			}
			// Echo FS record being compacted
			if verbose {
				log.Log(fmt.Sprintf("- Compacting block id=%v, size=%v\n", diskFs[i].id, diskFs[i].size))
			}
			// Find first empty block large enough for file
			var emptyPointer int = -1
			for j := 0; j < i; j++ {
				if diskFs[j].id == -1 && diskFs[j].size >= diskFs[i].size {
					emptyPointer = j
					break
				}
			}
			// If no empty space found, move on
			if emptyPointer == -1 {
				continue
			}
			if emptyPointer >= i {
				continue
			}
			// Echo FS empty space found
			if verbose {
				log.Log(fmt.Sprintf("  - Found empty space at index=%v, size=%v\n", emptyPointer, diskFs[emptyPointer].size))
			}
			// Move block
			if diskFs[emptyPointer].size >= diskFs[i].size {
				// Check for pre-alocated empty space extension block(s)
				if diskFs[emptyPointer+1].id != -1 || diskFs[emptyPointer+1].size != 0 {
					return 0, log.Dump(), errors.New("(pre)alocated empty space not found")
				}
				var emptySpaceSize = diskFs[emptyPointer].size
				// Move file to empty space
				diskFs[emptyPointer].id = diskFs[i].id
				diskFs[emptyPointer].size = diskFs[i].size
				diskFs[emptyPointer+1].size = emptySpaceSize - diskFs[emptyPointer].size
				diskFs[i].id = -1
			}
			// Echo disk image
			if verbose {
				log.Log("  - FS: " + echoDiskFs(diskFs))
			}
		}

		// Echo disk image
		diskImage = generateDiskImage(diskFs)
		if verbose {
			log.Log("- Disk: " + echoDiskImage(diskImage))
		}

		// Calculate checksum
		var cksum int = 0
		for i := 0; i < len(diskImage); i++ {
			if diskImage[i] != -1 {
				cksum += i * diskImage[i]
			}
		}

		// Return solution
		return cksum, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

func echoDiskImage(diskImage []int) string {
	// Initialize output
	var output string = ""
	// Output disk image
	for i := 0; i < len(diskImage); i++ {
		if diskImage[i] != -1 {
			output += fmt.Sprintf("%v", diskImage[i])
		} else {
			output += "."
		}
	}
	output += "\n"
	// Return output
	return output
}

func echoDiskFs(diskFs []Block) string {
	return echoDiskImage(generateDiskImage(diskFs))
}

func generateDiskImage(diskFs []Block) []int {
	// Calculate size
	var size int = 0
	for i := 0; i < len(diskFs); i++ {
		size += diskFs[i].size
	}
	// Allocate disk image
	var diskImage = make([]int, 0, size)
	// Write disk image
	for i := 0; i < len(diskFs); i++ {
		for j := 0; j < diskFs[i].size; j++ {
			diskImage = append(diskImage, diskFs[i].id)
		}
	}
	// Return disk image
	return diskImage
}
