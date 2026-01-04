package year2025

import (
	solution "adventofcode/lib"
	"errors"
	"os"
	"slices"
	"strings"
)

// Day one definition
type Day11 struct{}

var Day = Day11{}

// Year and day
func (day Day11) GetInfo() solution.SolutionInfo {
	return solution.SolutionInfo{
		Year: 2025,
		Day:  11,
	}
}

// Executions
func (day Day11) GetExecutions(index int, tag string) []solution.SolutionExecution {
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day11/input-test-01.txt"); return string(b) }(),
					Expect: uint64(5),
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day11/input.txt"); return string(b) }(),
					Expect: uint64(500),
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day11/input-test-02.txt"); return string(b) }(),
					Expect: uint64(2),
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
					Input:  func() string { var b, _ = os.ReadFile("./year2025/data/day11/input.txt"); return string(b) }(),
					Expect: uint64(287039700129600),
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day11) Run(index int, tag string, input any, verbose bool, log solution.Logger) (any, string, error) {
	// Initialize
	var value, ok = input.(string)
	if !ok {
		return nil, log.Dump(), errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var linesStr = strings.Split(strings.Trim(value, "\r\n "), "\n")
	var devices = make([]Device, len(linesStr))
	for i, lineStr := range linesStr {
		var lineParts = strings.Split(strings.Trim(lineStr, "\r\n "), ":")
		var id = strings.Trim(lineParts[0], "\r\n ")
		var outputsStr = strings.Split(strings.Trim(lineParts[1], "\r\n "), " ")
		var outputs = make([]string, len(outputsStr))
		for j, outputStr := range outputsStr {
			outputs[j] = strings.Trim(outputStr, "\r\n ")
		}
		devices[i] = Device{
			self: DeviceId{
				index: i,
				id:    id,
			},
			outputs: outputs,
		}
	}

	// Connect map indexes by name
	var deviceIndexByIdMap = make(map[string]int)
	for _, device := range devices {
		deviceIndexByIdMap[device.self.id] = device.self.index
	}

	// Register endpoint devices
	var count = len(devices)
	for i := 0; i < count; i++ {
		for j := range devices[i].outputs {
			var output = devices[i].outputs[j]
			// If endpoint device, register and set index
			if _, ok = deviceIndexByIdMap[output]; !ok {
				var index = len(devices)
				deviceIndexByIdMap[output] = index
				devices = append(devices, Device{
					self: DeviceId{
						id:    output,
						index: index,
					},
				})
			}
		}
	}

	// Part 1/2
	if index == 1 {

		// Return solution
		return countPaths("you", "out", []string{}, devices, deviceIndexByIdMap), log.Dump(), nil
	} else

	// Part 2/2
	if index == 2 {

		// Organize devices: svr -> dac -> fft -> out
		var svrToDac = countPaths("svr", "dac", []string{}, devices, deviceIndexByIdMap)
		var dacToFft = countPaths("dac", "fft", []string{}, devices, deviceIndexByIdMap)
		var fftToOut = countPaths("fft", "out", []string{}, devices, deviceIndexByIdMap)
		// Organize devices: svr -> fft -> dac -> out
		var svrToFft = countPaths("svr", "fft", []string{}, devices, deviceIndexByIdMap)
		var fftToDac = countPaths("fft", "dac", []string{}, devices, deviceIndexByIdMap)
		var dacToOut = countPaths("dac", "out", []string{}, devices, deviceIndexByIdMap)

		// Return solution
		var count = (svrToDac * dacToFft * fftToOut) + (svrToFft * fftToDac * dacToOut)
		return count, log.Dump(), nil
	}

	// Missing implementation
	return nil, log.Dump(), errors.New("missing implementation for required index")
}

type Device struct {
	self    DeviceId
	outputs []string
	info    DeviceInfo
}

type DeviceId struct {
	index int
	id    string
}

type DeviceInfo struct {
	processed bool
	paths     uint64
}

func countPaths(from string, to string, skip []string, orgDevices []Device, indexByOutputIdMap map[string]int) uint64 {
	var fromIndex, fromOk = indexByOutputIdMap[from]
	var toIndex, toOk = indexByOutputIdMap[to]
	if !fromOk || !toOk {
		panic("unknown device id")
	}
	var skipIndices = make([]int, len(skip))
	for i, s := range skip {
		var sIndex, sOk = indexByOutputIdMap[s]
		if !sOk {
			panic("unknown device id")
		}
		skipIndices[i] = sIndex
	}
	var devices = organize([]int{toIndex}, skipIndices, orgDevices, indexByOutputIdMap)
	return devices[fromIndex].info.paths
}

func organize(endpoints []int, skip []int, orgDevices []Device, indexByOutputIdMap map[string]int) []Device {
	// Initialize a copy of devices and prepare for processing
	var devices = make([]Device, len(orgDevices))
	for i := range orgDevices {
		devices[i] = orgDevices[i]
		// If endpoint, set as having a single path  and freeze as already (pre)processed
		if slices.Contains(endpoints, devices[i].self.index) {
			devices[i].info.processed = true
			devices[i].info.paths = 1
		} else
		// If must be skipped, set as having no paths and freeze as already (pre)processed
		if slices.Contains(skip, devices[i].self.index) {
			devices[i].info.processed = true
			devices[i].info.paths = 0
		} else
		// If device has no outputs, set as having no paths and freeze as already (pre)processed
		if len(devices[i].outputs) == 0 {
			devices[i].info.processed = true
			devices[i].info.paths = 0
		}
	}

	// Find paths to endpoint(s)
	for {
		// Find all devices connected to endpoint(s) or to other devices with known paths to endpoint(s)
		var pending = make(map[int]uint64)
		var hasUnprocessedDevices = false
		for i := range devices {
			if devices[i].info.processed {
				continue
			}

			// Process unprocessed device's outputs ...
			hasUnprocessedDevices = true
			var hasUnprocessedOutputs = false
			var paths uint64 = 0
			for _, output := range devices[i].outputs {
				var outputIndex, _ = indexByOutputIdMap[output]
				if !devices[outputIndex].info.processed {
					hasUnprocessedOutputs = true
					break
				}

				paths += devices[outputIndex].info.paths
			}

			if hasUnprocessedOutputs {
				continue
			}

			pending[i] = paths
		}

		// Apply pending changes
		for i, paths := range pending {
			devices[i].info.processed = true
			devices[i].info.paths = paths
		}

		// Done when all devices processed
		if !hasUnprocessedDevices {
			break
		}
	}

	// Verify all devices are processed
	for i := range devices {
		if !devices[i].info.processed {
			panic("device not processed")
		}
		if slices.Contains(endpoints, i) {
			if devices[i].info.paths != 1 {
				panic("endpoint device path differs from 1")
			}
		} else if slices.Contains(skip, i) {
			if devices[i].info.paths != 0 {
				panic("skipped device path differs from 0")
			}
		} else {
			var paths uint64 = 0
			for _, output := range devices[i].outputs {
				paths += devices[indexByOutputIdMap[output]].info.paths
			}
			if devices[i].info.paths != paths {
				panic("device paths count differs from its outputs'")
			}
		}
	}

	// Return processed devices
	return devices
}
