package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Direction string

const (
	North Direction = "N"
	East  Direction = "E"
	South Direction = "S"
	West  Direction = "W"
)

type Cords struct {
	x int
	y int
}

func (c *Cords) areCordsInBoard(board [][]string) bool {
	rowSize := len(board[0])
	colSize := len(board)

	if c.x < 0 || c.x >= colSize || c.y < 0 || c.y >= rowSize {
		return false
	}
	return true
}

type Guard struct {
	cords     Cords
	direction Direction
}

func readBoard(boardPath string) [][]string {
	file, _ := os.Open(boardPath)
	defer file.Close()

	var board [][]string

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		chars := strings.Split(line, "")
		board = append(board, chars)
	}

	return board
}

func findGuard(board [][]string) (Cords, Direction) {
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[i]); j++ {
			if board[i][j] == "^" {
				return Cords{i, j}, North
			}
		}
	}
	panic("No guard found")
}

func (g *Guard) nextMove() Cords {
	switch g.direction {
	case North:
		return Cords{x: g.cords.x - 1, y: g.cords.y}
	case East:
		return Cords{x: g.cords.x, y: g.cords.y + 1}
	case South:
		return Cords{x: g.cords.x + 1, y: g.cords.y}
	case West:
		return Cords{x: g.cords.x, y: g.cords.y - 1}
	}
	panic("Invalid direction")
}

func (g *Guard) changeDirection() {
	switch g.direction {
	case North:
		g.direction = East
	case East:
		g.direction = South
	case South:
		g.direction = West
	case West:
		g.direction = North
	}
}

func getFieldChar(cords Cords, board [][]string) string {
	if !cords.areCordsInBoard(board) {
		return ""
	}
	return board[cords.x][cords.y]
}

func simulate(board [][]string, startGuard Guard) (leavesBoard bool, loops bool) {
	guard := startGuard

	visitedStates := make(map[string]bool)
	initialState := fmt.Sprintf("%d,%d,%s", guard.cords.x, guard.cords.y, guard.direction)
	visitedStates[initialState] = true

	for {
		if !guard.cords.areCordsInBoard(board) {
			return true, false
		}

		next := guard.nextMove()
		nextChar := getFieldChar(next, board)

		if nextChar == "#" {
			guard.changeDirection()
		} else {
			guard.cords = next
		}

		stateKey := fmt.Sprintf("%d,%d,%s", guard.cords.x, guard.cords.y, guard.direction)
		if visitedStates[stateKey] {
			return false, true
		}
		visitedStates[stateKey] = true
	}
}

func copyBoard(original [][]string) [][]string {
	newBoard := make([][]string, len(original))
	for i := range original {
		newBoard[i] = make([]string, len(original[i]))
		copy(newBoard[i], original[i])
	}
	return newBoard
}

func main() {
	board := readBoard("./input.txt")
	guardStart, dir := findGuard(board)

	startGuard := Guard{
		cords:     guardStart,
		direction: dir,
	}

	possiblePositions := 0

	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[i]); j++ {
			if i == guardStart.x && j == guardStart.y {
				continue
			}
			if board[i][j] == "." {
				newBoard := copyBoard(board)
				newBoard[i][j] = "#"

				leaves, loops := simulate(newBoard, startGuard)
				if loops && !leaves {
					possiblePositions++
				}
			}
		}
	}

	fmt.Println(possiblePositions)
}
