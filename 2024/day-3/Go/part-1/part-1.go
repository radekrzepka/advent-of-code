package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func ReadFromFile(filePath string) string {
	file, _ := os.ReadFile(filePath)
	return string(file)
}

func main() {
	sum := 0
	input := ReadFromFile("./test-input.txt")

	r, _ := regexp.Compile(`mul\(([0-9]+),([0-9]+)\)`)

	matches := r.FindAllStringSubmatch(input, -1)

	for _, match := range matches {
		first, _ := strconv.Atoi(match[1])
		second, _ := strconv.Atoi(match[2])

		sum += first * second
	}

	fmt.Println(sum)
}
