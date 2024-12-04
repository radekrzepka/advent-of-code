package main

import (
	"fmt"
	"os"
	"regexp"
	"sort"
	"strconv"
)

func ReadFromFile(filePath string) string {
	file, _ := os.ReadFile(filePath)
	return string(file)
}

func min(a int, b int) int {
	if a < b {
		return a
	}

	return b
}

func GetIndexes(input string, literal string) []int {
	var indexes []int

	index := 0
	literalLength := len(literal)
	inputLength := len(input)

	for index < inputLength {
		slice := input[index:min(index+literalLength, inputLength)]

		if slice == literal {
			indexes = append(indexes, index)
			index += literalLength
			continue
		}

		index++
	}

	return indexes
}

func IsEnabled(index int, enableIndexes, disableIndexes []int) bool {
	allIndexes := append(enableIndexes, disableIndexes...)
	sort.Ints(allIndexes)

	enabled := true
	for _, idx := range allIndexes {
		if index < idx {
			break
		}
		if contains(enableIndexes, idx) {
			enabled = true
		} else if contains(disableIndexes, idx) {
			enabled = false
		}
	}
	return enabled
}

func contains(slice []int, val int) bool {
	for _, v := range slice {
		if v == val {
			return true
		}
	}
	return false
}

func main() {
	sum := 0
	input := ReadFromFile("./input.txt")
	inputLength := len(input)

	mulRegex, _ := regexp.Compile(`mul\(([0-9]+),([0-9]+)\)`)

	enableIndexes := GetIndexes(input, "do()")
	disableIndexes := GetIndexes(input, "don't()")

	fmt.Println("Enable indexes:", enableIndexes)
	fmt.Println("Disable indexes:", disableIndexes)

	index := 0
	for index < inputLength {
		mulMatch := mulRegex.FindStringSubmatchIndex(input[index:])
		if mulMatch != nil {
			startIndex := index + mulMatch[0]

			if IsEnabled(startIndex, enableIndexes, disableIndexes) {
				x, _ := strconv.Atoi(input[startIndex+mulMatch[2]-mulMatch[0] : startIndex+mulMatch[3]-mulMatch[0]])
				y, _ := strconv.Atoi(input[startIndex+mulMatch[4]-mulMatch[0] : startIndex+mulMatch[5]-mulMatch[0]])
				sum += x * y
			}

			index += mulMatch[1]
		} else {
			index++
		}
	}

	fmt.Println("Sum of enabled multiplications:", sum)
}
