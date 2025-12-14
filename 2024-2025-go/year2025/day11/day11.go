package year2024

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
					Expect: 5,
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
					Expect: 500,
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
					Expect: 2,
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
					Expect: 0, // 284861348640000 too low
				},
			)
		}
	}
	return executions
}

// Implementation
func (day Day11) Run(index int, tag string, input any, verbose bool) (any, string, error) {
	// Initialize
	var output = ""
	var value, ok = input.(string)
	if !ok {
		return nil, output, errors.New("failed casting execution to correct Input/Output types")
	}

	// Parse inputs
	var linesStr = strings.Split(strings.Trim(value, "\r\n "), "\n")
	var devices = make([]Device, len(linesStr))
	for i, lineStr := range linesStr {
		var lineParts = strings.Split(strings.Trim(lineStr, "\r\n "), ":")
		var id = strings.Trim(lineParts[0], "\r\n ")
		var outputsStr = strings.Split(strings.Trim(lineParts[1], "\r\n "), " ")
		var outputs = make([]DeviceId, len(outputsStr))
		for j, outputStr := range outputsStr {
			outputs[j] = DeviceId{id: strings.Trim(outputStr, "\r\n ")}
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
	var _map = make(map[string]int)
	for _, device := range devices {
		_map[device.self.id] = device.self.index
	}
	// Map indexes to output devices, and register endpoint devices
	for i := 0; i < len(devices); i++ {
		var device = &devices[i]
		for j := range device.outputs {
			var output = &device.outputs[j]
			var index, ok = _map[output.id]
			// If existing device, set index
			if ok {
				output.index = index
			} else
			// If endpoint device, register and set index
			{
				var index = len(devices)
				var new = Device{
					self: DeviceId{
						id:    output.id,
						index: index,
					},
				}
				devices = append(devices, new)
				output.index = index
				_map[output.id] = output.index
			}
		}
	}

	// Part 1/2
	if index == 1 {

		// Return solution
		return countPaths(_map["you"], _map["out"], []int{}, devices), output, nil
	} else

	// Part 2/2
	if index == 2 {

		// var test = countPaths(_map["dac"], _map["fft"], []int{}, devices)
		// panic(test)

		// Organize devices
		var svrToDac = countPaths(_map["svr"], _map["dac"], []int{_map["fft"], _map["out"]}, devices)
		var dacToFft = countPaths(_map["dac"], _map["fft"], []int{_map["svr"], _map["out"]}, devices)
		var fftToOut = countPaths(_map["fft"], _map["out"], []int{_map["svr"], _map["dac"]}, devices)
		var svrToFft = countPaths(_map["svr"], _map["fft"], []int{_map["dac"], _map["out"]}, devices)
		var fftToDac = countPaths(_map["fft"], _map["dac"], []int{_map["svr"], _map["out"]}, devices)
		var dacToOut = countPaths(_map["dac"], _map["out"], []int{_map["svr"], _map["fft"]}, devices)

		// Return solution
		var count = (svrToDac * dacToFft * fftToOut) + (svrToFft * fftToDac * dacToOut)
		return count, output, nil
	}

	// Missing implementation
	return nil, output, errors.New("missing implementation for required index")
}

type Device struct {
	self    DeviceId
	outputs []DeviceId
	info    DeviceInfo
}

type DeviceId struct {
	index int
	id    string
}

type DeviceInfo struct {
	processed bool
	paths     int
}

func countPaths(from int, to int, skip []int, devices []Device) int {
	organize([]int{to}, skip, devices)
	return devices[from].info.paths
}

func organize(endpoints []int, skip []int, devices []Device) {
	// Initialize devices
	for i := range devices {
		var device = &devices[i]
		// If endpoint, set as having a single path  and freeze as already (pre)processed
		if slices.Contains(endpoints, device.self.index) {
			device.info.processed = true
			device.info.paths = 1
		} else
		// If must be skipped, set as having no paths and freeze as already (pre)processed
		if slices.Contains(skip, device.self.index) {
			device.info.processed = true
			device.info.paths = 0
		} else
		// If device has no outputs, set as having no paths and freeze as already (pre)processed
		if len(device.outputs) == 0 {
			device.info.processed = true
			device.info.paths = 0
		} else
		// ... else, reset device
		{
			device.info.processed = false
			device.info.paths = 0
		}
	}

	// Find paths to endpoint(s)
	for {

		// Find all devices connected to endpoint(s) or to other devices with known paths to endpoint(s)
		var hasUnprocessedDevices = false
		for i := range devices {
			var device = &devices[i]
			if device.info.processed {
				continue
			}

			// If processed, process all outputs
			var hasUnprocessedOutputs = false
			var paths = 0
			for j := range device.outputs {
				var outputDevice = &devices[device.outputs[j].index]

				if !outputDevice.info.processed {
					hasUnprocessedOutputs = true
					break
				}

				paths += outputDevice.info.paths
			}

			if hasUnprocessedOutputs {
				hasUnprocessedDevices = true
				continue
			}

			device.info.processed = true
			device.info.paths = paths
		}

		if !hasUnprocessedDevices {
			break
		}

	}
}
