package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type rule struct {
	before int
	after  int
}

func readRules(rulesPath string) []rule {
	file, _ := os.Open(rulesPath)

	defer file.Close()

	scanner := bufio.NewScanner(file)

	var rules []rule

	for scanner.Scan() {
		line := scanner.Text()
		nums := strings.Split(line, "|")

		num1, _ := strconv.Atoi(nums[0])
		num2, _ := strconv.Atoi(nums[1])

		newRule := rule{
			before: num1,
			after:  num2,
		}

		rules = append(rules, newRule)
	}

	return rules
}

func readBooks(pagesPath string) [][]int {
	file, _ := os.Open(pagesPath)

	defer file.Close()

	scanner := bufio.NewScanner(file)

	var pages [][]int

	for scanner.Scan() {
		line := scanner.Text()
		nums := strings.Split(line, ",")

		var page []int
		for _, num := range nums {
			intNum, _ := strconv.Atoi(num)
			page = append(page, intNum)
		}
		pages = append(pages, page)
	}

	return pages
}

func checkIfOrderIsCorrect(beforePage int, afterPage int, rules []rule) bool {
	for _, rule := range rules {
		if afterPage == rule.before && beforePage == rule.after {
			return false
		}
	}

	return true
}

func checkIfBookIsCorrectOrder(book []int, rules []rule) bool {
	for i := 0; i < len(book); i++ {
		for j := i + 1; j < len(book); j++ {
			beforePage := book[i]
			afterPage := book[j]

			isCorrectOrder := checkIfOrderIsCorrect(beforePage, afterPage, rules)

			if !isCorrectOrder {
				return false
			}

		}
	}

	return true
}

func getBookInCorrectOrder(book *[]int, rules []rule) {
	for i := 0; i < len(*book); i++ {
		for j := i + 1; j < len(*book); j++ {
			beforePage := (*book)[i]
			afterPage := (*book)[j]

			isCorrectOrder := checkIfOrderIsCorrect(beforePage, afterPage, rules)

			if isCorrectOrder {
				continue
			}

			(*book)[i], (*book)[j] = (*book)[j], (*book)[i]

		}
	}
}

func getPageToAdd(book []int, rules []rule) int {
	isBookInCorrectOrder := checkIfBookIsCorrectOrder(book, rules)
	if isBookInCorrectOrder {
		fmt.Println("correct", book)
		return 0
	}

	getBookInCorrectOrder(&book, rules)

	middleIndex := len(book) / 2
	return book[middleIndex]
}

func main() {
	rules := readRules("./rules.txt")
	books := readBooks("./pages.txt")

	middlePagesSum := 0

	for _, book := range books {
		pageToAdd := getPageToAdd(book, rules)
		middlePagesSum += pageToAdd
	}

	fmt.Println(middlePagesSum)
}
