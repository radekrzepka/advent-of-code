package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	fmt.Println(SolvePart2("./part-2-input.txt"))
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

func GetArrays(lines []string) ([]int, []int) {
	var firstList []int
	var secondList []int

	for _, line := range lines {
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

func CountOccurrences(numbers []int) map[int]int {
	counts := make(map[int]int)
	for _, number := range numbers {
		counts[number]++
	}
	return counts
}

func CountSimilarityScore(first []int, second []int) int {
	similarityScore := 0

	firstMap := CountOccurrences(first)
	secondMap := CountOccurrences(second)

	fmt.Println(firstMap, secondMap)

	for key, value := range firstMap {
		if secondMap[key] == 0 {
			continue
		}

		scoreToAdd := key * secondMap[key] * value
		similarityScore += int(scoreToAdd)
	}

	return similarityScore
}

func SolvePart2(inputPath string) int {
	lines, err := ReadFile(inputPath)
	if err != nil {
		log.Fatalf("Error reading file: %s", err)
	}

	first, second := GetArrays(lines)
	similarityScore := CountSimilarityScore(first, second)

	return similarityScore
}
