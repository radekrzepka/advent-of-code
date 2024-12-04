package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func reverse(str string) (result string) {
	for _, v := range str {
		result = string(v) + result
	}
	return
}

func min(a int, b int) int {
	if a < b {
		return a
	}

	return b
}

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

func getRowWord(letters [][]string, wordSize int, i int, j int) string {
	return strings.Join(letters[i][j:min(j+wordSize, len(letters))], "")
}

func getColWord(letters [][]string, wordSize int, i int, j int) string {
	word := ""

	for k := i; k < min(i+wordSize, len(letters)); k++ {
		word += letters[k][j]
	}

	return word
}

func getDiagonalWordLTR(letters [][]string, wordSize, i, j int) string {
	word := ""
	for k := 0; k < wordSize; k++ {
		if i+k < len(letters) && j+k < len(letters[0]) {
			word += letters[i+k][j+k]
		} else {
			break
		}
	}
	return word
}

func getDiagonalWordRTL(letters [][]string, wordSize, i, j int) string {
	word := ""
	for k := 0; k < wordSize; k++ {
		if i+k < len(letters) && j-k >= 0 {
			word += letters[i+k][j-k]
		} else {
			break
		}
	}
	return word
}

func main() {
	letters, _ := readFile("./input.txt")

	founded := 0

	wordToFind := "XMAS"
	reverseWordToFind := reverse(wordToFind)
	wordToFindLength := len(wordToFind)

	rowNum := len(letters)
	colNum := len(letters[0])

	for i := 0; i < rowNum; i++ {
		for j := 0; j < colNum; j++ {
			rowWord := getRowWord(letters, wordToFindLength, i, j)
			if rowWord == wordToFind || rowWord == reverseWordToFind {
				founded++
			}

			colWord := getColWord(letters, wordToFindLength, i, j)
			if colWord == wordToFind || colWord == reverseWordToFind {
				founded++
			}

			diagWordLTR := getDiagonalWordLTR(letters, wordToFindLength, i, j)
			if diagWordLTR == wordToFind || diagWordLTR == reverseWordToFind {
				founded++
			}

			diagWordRTL := getDiagonalWordRTL(letters, wordToFindLength, i, j)
			if diagWordRTL == wordToFind || diagWordRTL == reverseWordToFind {
				founded++
			}
		}
	}

	fmt.Println(founded)
}
