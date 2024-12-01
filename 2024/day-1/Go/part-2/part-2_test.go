package main

import "testing"

func TestSolvePart2(t *testing.T) {
	expected := 31
	result := SolvePart2("./part-2-test-input.txt")
	if result != expected {
		t.Errorf("Expected %d, but got %d", expected, result)
	}
}
