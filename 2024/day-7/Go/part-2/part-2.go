package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Operator struct {
	expectedValue int64
	numbers       []int64
}

func readFile(filePath string) []Operator {
	file, _ := os.Open(filePath)
	defer file.Close()

	var operators []Operator
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		parts := strings.Split(line, ":")
		expectedValue, _ := strconv.ParseInt(strings.TrimSpace(parts[0]), 10, 64)

		numsText := strings.Fields(parts[1])
		var numbers []int64
		for _, num := range numsText {
			numInt, _ := strconv.ParseInt(num, 10, 64)
			numbers = append(numbers, numInt)
		}

		operators = append(operators, Operator{
			expectedValue: expectedValue,
			numbers:       numbers,
		})
	}

	return operators
}

func generateOperatorCombinations(length int, operators []string) [][]string {
	if length == 0 {
		return [][]string{{}}
	}

	subCombinations := generateOperatorCombinations(length-1, operators)
	result := [][]string{}

	for _, sub := range subCombinations {
		for _, op := range operators {
			newSub := append([]string(nil), sub...)
			newSub = append(newSub, op)
			result = append(result, newSub)
		}
	}

	return result
}

func evaluate(a int64, b int64, operator string) int64 {
	switch operator {
	case "+":
		return a + b
	case "*":
		return a * b
	case "||":
		strA := strconv.FormatInt(a, 10)
		strB := strconv.FormatInt(b, 10)
		concatenated, _ := strconv.ParseInt(strA+strB, 10, 64)
		return concatenated
	default:
		panic("Unknown operator")
	}
}

func generateExpressionsAndEvaluate(operator Operator, operations []string) int64 {
	numbers := operator.numbers
	expectedValue := operator.expectedValue

	opCombinations := generateOperatorCombinations(len(numbers)-1, operations)
	for _, combination := range opCombinations {
		result := numbers[0]
		for i, op := range combination {
			result = evaluate(result, numbers[i+1], op)
		}

		if result == expectedValue {
			return expectedValue
		}
	}

	return 0
}

func main() {
	operations := [3]string{"+", "*", "||"}
	operators := readFile("./input.txt")

	var totalResult int64

	for _, operator := range operators {
		value := generateExpressionsAndEvaluate(operator, operations[:])
		// fmt.Println(value, operator.numbers)
		totalResult += value
	}

	fmt.Println(totalResult)
}
