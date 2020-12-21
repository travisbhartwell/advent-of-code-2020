package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

// TreeInLocation is whether a tree is in the map location
type TreeInLocation bool

// TreeLocationMap is the map of location data
type TreeLocationMap struct {
	MapData [][]TreeInLocation
	Height  int
	Width   int
}

func (treeMap TreeLocationMap) treeInLocation(x int, y int) (TreeInLocation, bool) {
	if y >= treeMap.Height {
		return false, true
	}
	actualX := x % treeMap.Width
	return treeMap.MapData[y][actualX], false
}

func (treeMap TreeLocationMap) countTreesInPath(stepX int, stepY int) int {
	var count = 0

	var x = 0
	var y = 0

	for {
		treeInLocation, reachedBottom := treeMap.treeInLocation(x, y)

		if treeInLocation {
			count++
		}

		if reachedBottom {
			return count
		}

		x += stepX
		y += stepY
	}

}

func contents(filename string) (*TreeLocationMap, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}

	s := bufio.NewScanner(f)
	mapData := [][]TreeInLocation{}

	for s.Scan() {
		rowData := []TreeInLocation{}
		t := strings.TrimSpace(s.Text())
		for _, c := range t {
			switch c {
			case '#':
				rowData = append(rowData, true)
			case '.':
				rowData = append(rowData, false)
			default:
				log.Fatal("Unknown character ", c)
			}
		}
		mapData = append(mapData, rowData)
	}

	if err := s.Err(); err != nil {
		return nil, err
	}

	height := len(mapData)
	width := len(mapData[0])

	treeMap := TreeLocationMap{mapData, height, width}
	return &treeMap, nil
}

func main() {
	var inputFilename string = "input.txt"

	if len(os.Args) > 1 {
		inputFilename = os.Args[1]
	}

	treeMap, err := contents(inputFilename)

	if err != nil {
		log.Fatal("Error reading ", inputFilename, ":", err)
	}

	treeCount := treeMap.countTreesInPath(3, 1)
	fmt.Printf("Part 1: There are %v trees in the path.\n", treeCount)

	slopes := [5][2]int{{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}}
	product := 1

	for _, slope := range slopes {
		count := treeMap.countTreesInPath(slope[0], slope[1])
		product *= count
	}

	fmt.Printf("Part 2: Product of trees found: %v\n", product)
}
