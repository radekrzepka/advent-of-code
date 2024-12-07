package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strings"
)

type Direction string

type Board [][]string

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

func (c *Cords) areCordsInBoard(board Board) bool {
	rowSize := len(board[0])
	colSize := len(board)

	if c.x < 0 || c.x >= rowSize || c.y < 0 || c.y >= colSize {
		return false
	}

	return true
}

type Guard struct {
	cords     Cords
	direction Direction
}

func (g *Guard) initPosition(board Board) {
	for i := range board {
		for j := range board[i] {
			if board[i][j] == "^" {
				g.cords.x = i
				g.cords.y = j
				return
			}
		}
	}
}

func (g *Guard) isGuardInBoard(board Board) bool {
	return g.cords.areCordsInBoard(board)
}

func (g *Guard) nextMove() Cords {
	var nextMoveCords Cords

	switch g.direction {
	case North:
		nextMoveCords = Cords{
			x: g.cords.x - 1,
			y: g.cords.y,
		}
	case East:
		nextMoveCords = Cords{
			x: g.cords.x,
			y: g.cords.y + 1,
		}
	case South:
		nextMoveCords = Cords{
			x: g.cords.x + 1,
			y: g.cords.y,
		}
	case West:
		nextMoveCords = Cords{
			x: g.cords.x,
			y: g.cords.y - 1,
		}
	}

	return nextMoveCords
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

func getFieldChar(cords Cords, board Board) string {
	if !cords.areCordsInBoard(board) {
		return "."
	}
	return board[cords.x][cords.y]
}

func (g *Guard) move(board Board, moves *int, movesHistory *[]Cords) {
	nextGuardMove := g.nextMove()
	nextMoveField := getFieldChar(nextGuardMove, board)

	if nextMoveField == "#" {
		g.changeDirection()
		// fmt.Println("Changed direction", g.cords.x, g.cords.y, getFieldChar(nextGuardMove, board))
		return
	}

	g.cords = nextGuardMove
	if slices.Contains(*movesHistory, nextGuardMove) {
		return
	}
	*movesHistory = append(*movesHistory, nextGuardMove)
	*moves++

	// fmt.Println(nextGuardMove, getFieldChar(nextGuardMove, board))
}

func readBoard(boardPath string) Board {
	file, _ := os.Open(boardPath)
	defer file.Close()

	var board Board

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		chars := strings.Split(line, "")
		board = append(board, chars)
	}

	return board
}

func main() {
	board := readBoard("./input.txt")

	guard := Guard{
		cords: Cords{
			x: 0,
			y: 0,
		},
		direction: North,
	}
	guard.initPosition(board)

	moves := 0
	var movesHistory []Cords

	for guard.isGuardInBoard(board) {
		guard.move(board, &moves, &movesHistory)
	}

	fmt.Println(moves - 1)
}
