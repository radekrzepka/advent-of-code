package main

import (
	"os"
	"testing"
)

func TestReadFile(t *testing.T) {
	tests := []struct {
		name     string
		content  string
		expected [][]int
		hasError bool
	}{
		{
			name:    "Valid data with two lines",
			content: "1 3 6 1 2\n4 5 6\n",
			expected: [][]int{
				{1, 3, 6, 1, 2},
				{4, 5, 6},
			},
			hasError: false,
		},
		{
			name:     "Empty file",
			content:  "",
			expected: [][]int{},
			hasError: false,
		},
		{
			name:     "Invalid data with non-integer values",
			content:  "1 3 a 1 2\n4 5 6\n",
			expected: nil,
			hasError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			//ARRANGE
			file, err := os.CreateTemp("", "test.txt")
			if err != nil {
				t.Fatalf("Failed to create temp file: %v", err)
			}
			defer os.Remove(file.Name())
			file.WriteString(tt.content)
			file.Close()

			// ACT
			result, err := ReadFile(file.Name())

			// ASSERT
			if tt.hasError {
				if err == nil {
					t.Errorf("Expected an error but got none")
				}
				return
			}
			if err != nil {
				t.Errorf("Unexpected error: %v", err)
				return
			}

			if len(result) != len(tt.expected) {
				t.Errorf("Expected %d lines, got %d", len(tt.expected), len(result))
				return
			}
			for i, line := range result {
				for j, num := range line {
					if num != tt.expected[i][j] {
						t.Errorf("Expected %d at [%d][%d], got %d", tt.expected[i][j], i, j, num)
					}
				}
			}
		})
	}
}

func TestIsLevelSafe(t *testing.T) {
	tests := []struct {
		name     string
		level    []int
		expected bool
	}{
		{
			name: "Safe without removing any level",
			level: []int{
				7, 6, 4, 2, 1,
			},
			expected: true,
		},
		{
			name: "Unsafe regardless of which level is removed",
			level: []int{
				1, 2, 7, 8, 9,
			},
			expected: false,
		},
		{
			name: "Unsafe regardless of which level is removed",
			level: []int{
				9, 7, 6, 2, 1,
			},
			expected: false,
		},
		{
			name: "Safe by removing the second level, 3",
			level: []int{
				1, 3, 2, 4, 5,
			},
			expected: true,
		},
		{
			name: "Safe by removing the third level, 4",
			level: []int{
				8, 6, 4, 4, 1,
			},
			expected: true,
		},
		{
			name: "Safe without removing any level",
			level: []int{
				1, 3, 6, 7, 9,
			},
			expected: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// ACT
			result := IsLevelSafe(tt.level)

			// ASSERT
			if tt.expected != result {
				t.Errorf("Expected %t, got %t", tt.expected, result)
			}
		})
	}
}

func TestCountSaveLevels(t *testing.T) {
	tests := []struct {
		name     string
		levels   [][]int
		expected int
	}{
		{
			name: "All safe levels",
			levels: [][]int{
				{1, 2, 3, 4, 5}, // ascending safe
				{5, 4, 3, 2, 1}, // descending safe
			},
			expected: 2,
		},
		{
			name: "Mixed safe and unsafe levels",
			levels: [][]int{
				{1, 2, 3, 4, 5}, // ascending safe
				{5, 4, 3, 2, 1}, // descending safe
				{1, 2, 7, 8, 9}, // unsafe, jump of 5
				{1, 3, 2, 4, 5}, // safe by removing one level
			},
			expected: 3,
		},
		{
			name: "No safe levels",
			levels: [][]int{
				{1, 2, 7, 8, 9}, // unsafe, jump of 5
				{9, 7, 6, 2, 1}, // unsafe, drop of 4
				{1, 3, 2, 4, 5}, // safe by removing one level
			},
			expected: 1,
		},
		{
			name:     "Empty input",
			levels:   [][]int{},
			expected: 0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := CountSaveLevels(tt.levels)

			if result != tt.expected {
				t.Errorf("Expected %d, got %d for input %v", tt.expected, result, tt.levels)
			}
		})
	}
}

func TestSolvePart2(t *testing.T) {
	expected := 4
	result := SolvePart2("./part-2-test-input.txt")
	if result != expected {
		t.Errorf("Expected %d, but got %d", expected, result)
	}
}
