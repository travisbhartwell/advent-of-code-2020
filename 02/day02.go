package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

// PasswordData is the data for one password
type PasswordData struct {
	Min      int
	Max      int
	Letter   string
	Password string
}

func (pd PasswordData) isValidPart1() bool {
	letterCount := strings.Count(pd.Password, pd.Letter)
	return pd.Min <= letterCount && letterCount <= pd.Max
}

func (pd PasswordData) isValidPart2() bool {
	position1 := pd.Min - 1
	position2 := pd.Max - 1

	charInPos1 := pd.Password[position1] == pd.Letter[0]
	charInPos2 := pd.Password[position2] == pd.Letter[0]
	return charInPos1 != charInPos2
}

func contents(filename string) ([]PasswordData, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}

	s := bufio.NewScanner(f)
	passwordData := []PasswordData{}
	for s.Scan() {
		t := strings.TrimSpace(s.Text())
		parts := strings.Split(t, " ")
		minAndMax := strings.Split(parts[0], "-")

		min, err := strconv.Atoi(minAndMax[0])
		if err != nil {
			log.Fatal("Error parsing: \n", t)
		}

		max, err := strconv.Atoi(minAndMax[1])
		if err != nil {
			log.Fatal("Error parsing: \n", t)
		}

		passwordData = append(passwordData, PasswordData{min, max, parts[1][:1], parts[2]})
	}

	if err := s.Err(); err != nil {
		return nil, err
	}

	return passwordData, nil
}

func main() {
	var inputFilename string = "input.txt"

	if len(os.Args) > 1 {
		inputFilename = os.Args[1]
	}

	passwordData, err := contents(inputFilename)

	if err != nil {
		log.Fatal("Error reading ", inputFilename, ":", err)
	}

	var count = 0

	for _, datum := range passwordData {
		if datum.isValidPart1() {
			count++
		}
	}

	fmt.Printf("Part 1: Found %v valid passports.\n", count)

	count = 0

	for _, datum := range passwordData {
		if datum.isValidPart2() {
			count++
		}
	}

	fmt.Printf("Part 2: Found %v valid passports.\n", count)
}
