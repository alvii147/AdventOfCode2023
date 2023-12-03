package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var ReGame *regexp.Regexp = regexp.MustCompile(`^Game (\d+):\s*(.*)\s*$`)
var ReCube *regexp.Regexp = regexp.MustCompile(`^\s*(\d+)\s+([a-z]+)\s*$`)

type GameDraw struct {
	Red   int
	Green int
	Blue  int
}

type GameRecord struct {
	ID    int
	Draws []*GameDraw
}

const MaxRed = 12
const MaxGreen = 13
const MaxBlue = 14

func ParseGameRecord(gameStr string) (*GameRecord, error) {
	matches := ReGame.FindAllStringSubmatch(gameStr, -1)
	if len(matches) < 1 || len(matches[0]) < 2 {
		return nil, fmt.Errorf("ParseGameInfo failed to ReGame.FindAllStringSubmatch '%s'", gameStr)
	}

	gameID, err := strconv.Atoi(matches[0][1])
	if err != nil {
		return nil, fmt.Errorf("ParseGameInfo failed to strconv.Atoi '%s': %w", matches[0][1], err)
	}

	gameRecord := &GameRecord{
		ID:    gameID,
		Draws: make([]*GameDraw, 0),
	}

	cubesDrawsStr := strings.Split(matches[0][2], ";")
	for _, cubesDrawStr := range cubesDrawsStr {
		cubesStr := strings.Split(cubesDrawStr, ",")
		gameDraw := &GameDraw{}
		for _, cubeStr := range cubesStr {
			matches := ReCube.FindAllStringSubmatch(cubeStr, -1)
			if len(matches) < 1 || len(matches[0]) < 2 {
				return nil, fmt.Errorf("ParseGameInfo failed to ReCube.FindAllStringSubmatch '%s'", cubeStr)
			}

			cubeCount, err := strconv.Atoi(matches[0][1])
			if err != nil {
				return nil, fmt.Errorf("ParseGameInfo failed to strconv.Atoi '%s': %w", matches[0][1], err)
			}

			cubeColor := matches[0][2]
			switch cubeColor {
			case "red":
				gameDraw.Red = cubeCount
			case "green":
				gameDraw.Green = cubeCount
			case "blue":
				gameDraw.Blue = cubeCount
			default:
				return nil, fmt.Errorf("ParseGameInfo failed, unknown cube color '%s'", cubeColor)
			}
		}

		gameRecord.Draws = append(gameRecord.Draws, gameDraw)
	}

	return gameRecord, nil
}

func main() {
	filePath := "../games.txt"
	f, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("failed to Open file '%s': %v\n", filePath, err)
		return
	}

	gameIDSum := 0
	gamePowerSum := 0

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		gameRecord, err := ParseGameRecord(line)
		if err != nil {
			fmt.Printf("failed to ParseGameRecord line '%s': %v\n", line, err)
			return
		}

		fewestRed := 0
		fewestGreen := 0
		fewestBlue := 0
		impossible := false
		for _, draw := range gameRecord.Draws {
			if draw.Red > MaxRed || draw.Green > MaxGreen || draw.Blue > MaxBlue {
				impossible = true
			}

			if draw.Red > fewestRed {
				fewestRed = draw.Red
			}

			if draw.Green > fewestGreen {
				fewestGreen = draw.Green
			}

			if draw.Blue > fewestBlue {
				fewestBlue = draw.Blue
			}
		}

		if !impossible {
			gameIDSum += gameRecord.ID
		}
		gamePowerSum += fewestRed * fewestGreen * fewestBlue
	}

	fmt.Println(gameIDSum)
	fmt.Println(gamePowerSum)
}
