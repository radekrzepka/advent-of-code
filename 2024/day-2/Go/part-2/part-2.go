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
	fmt.Println(SolvePart2("./part-2-input.txt"))
}

func ReadFile(filePath string) ([][]int, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines [][]int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		text := scanner.Text()
		values := strings.Split(text, " ")

		var nums []int
		for _, value := range values {
			num, err := strconv.Atoi(value)
			if err != nil {
				return nil, err
			}
			nums = append(nums, num)
		}

		lines = append(lines, nums)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return lines, nil
}

func CheckSortedAndDiff(level []int) bool {
	if !slices.IsSorted(level) {
		reversed := slices.Clone(level)
		slices.Reverse(reversed)
		if !slices.IsSorted(reversed) {
			return false
		}
	}

	for i := 0; i < len(level)-1; i++ {
		diff := int(math.Abs(float64(level[i]) - float64(level[i+1])))
		if diff == 0 || diff > 3 {
			return false
		}
	}

	return true
}

func IsLevelSafe(level []int) bool {
	if CheckSortedAndDiff(level) {
		return true
	}

	for i := 0; i < len(level); i++ {
		temp := append([]int{}, level[:i]...)
		temp = append(temp, level[i+1:]...)
		if CheckSortedAndDiff(temp) {
			return true
		}
	}

	return false
}

func CountSaveLevels(levels [][]int) int {
	saveLevels := 0

	for _, level := range levels {
		isLevelSave := IsLevelSafe(level)
		if isLevelSave {
			saveLevels++
		}
	}

	return saveLevels
}

func SolvePart2(inputPath string) int {
	levels, err := ReadFile(inputPath)
	if err != nil {
		log.Fatalf("Error reading file: %s", err)
	}

	saveLevels := CountSaveLevels(levels)

	return saveLevels
}
