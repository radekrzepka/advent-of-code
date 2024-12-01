package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
    fmt.Println(SolvePart1("./part-1-input.txt"))
}

func ReadFile(filePath string) ([]string, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return lines, nil
}

func GetArrays (lines []string) ([]int, []int) {
    var firstList []int
    var secondList []int

    for _ ,line := range lines {
        parts := strings.Split(line, "   ")
		if len(parts) != 2 {
			continue
		}

        firstValue, err1 := strconv.Atoi(parts[0])
		secondValue, err2 := strconv.Atoi(parts[1])
		if err1 == nil && err2 == nil {
			firstList = append(firstList, firstValue)
			secondList = append(secondList, secondValue)
		}
    }

    return firstList, secondList

}

func SolvePart1(inputPath string) int {
    distance := 0

	lines, err := ReadFile(inputPath)
	if err != nil {
		log.Fatalf("Error reading file: %s", err)
	}

	first, second := GetArrays(lines)
    slices.Sort(first)
    slices.Sort(second)

    for i := range first {
        diff := first[i] - second[i]
        distance += int(math.Abs(float64(diff)))
    }

    return distance
}
