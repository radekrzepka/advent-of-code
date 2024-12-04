package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func readFile(filePath string) ([][]string, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines [][]string

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		text := scanner.Text()
		values := strings.Split(text, "")

		lines = append(lines, values)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return lines, nil
}

func isXMASPattern(letters [][]string, i, j int) bool {
	if i-1 < 0 || i+1 >= len(letters) || j-1 < 0 || j+1 >= len(letters[0]) {
		return false
	}

	center := letters[i][j]

	topLeft := letters[i-1][j-1] + center + letters[i+1][j+1]
	topRight := letters[i-1][j+1] + center + letters[i+1][j-1]

	return (topLeft == "MAS" || topLeft == "SAM") &&
		(topRight == "MAS" || topRight == "SAM")
}

func main() {
	letters, err := readFile("./input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	founded := 0

	rowNum := len(letters)
	colNum := len(letters[0])

	for i := 0; i < rowNum; i++ {
		for j := 0; j < colNum; j++ {
			if isXMASPattern(letters, i, j) {
				founded++
			}
		}
	}

	fmt.Println(founded)
}
