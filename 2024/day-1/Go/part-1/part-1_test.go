package main

import "testing"

func TestSolvePart1(t *testing.T) {
    expected := 11
    result := SolvePart1("./part-1-test-input.txt")
    if result != expected {
        t.Errorf("Expected %d, but got %d", expected, result)
    }
}
