package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func contents(filename string) ([]int, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}

	s := bufio.NewScanner(f)
	numbers := []int{}
	for s.Scan() {
		t := strings.TrimSpace(s.Text())
		i, err := strconv.Atoi(t)

		if err != nil {
			fmt.Println("Error parsing: ", t)
		}
		numbers = append(numbers, i)
	}

	if err := s.Err(); err != nil {
		return nil, err
	}

	return numbers, nil
}

func valueToPosMap(inputNumbers []int) (valuesToPos map[int]int) {
	valuesToPos = make(map[int]int, len(inputNumbers))
	for index, el := range inputNumbers {
		valuesToPos[el] = index
	}
	return
}

func existsInRange(value int, start int, end int, valuesToPos map[int]int) bool {
	if pos, ok := valuesToPos[value]; ok && pos >= start && pos < end {
		return true
	}
	return false
}

func validateInput(inputNumbers []int, preambleLength int) (invalidNumbers []int) {
	invalidNumbers = make([]int, 0)
	var valuesToPos map[int]int = valueToPosMap(inputNumbers)

	for validateIndex := preambleLength; validateIndex < len(inputNumbers); validateIndex++ {
		var valid bool = false
		var start int = validateIndex - preambleLength
		var preamble []int = inputNumbers[start:validateIndex]
		var target int = inputNumbers[validateIndex]

		for _, el := range preamble {
			if el > target {
				continue
			}

			var pair = target - el

			if existsInRange(pair, start, validateIndex, valuesToPos) {
				valid = true
				break
			}
		}

		if !valid {
			invalidNumbers = append(invalidNumbers, target)
		}
	}

	return
}

func findTargetSum(targetSum int, inputNumbers []int) ([]int, bool) {
	for startIndex, num := range inputNumbers {
		var runningSum int = num

		for index, num := range inputNumbers[startIndex+1:] {
			runningSum += num

			if runningSum == targetSum {
				return inputNumbers[startIndex:(startIndex + index + 1)], true
			}

			if runningSum > targetSum {
				break
			}
		}
	}

	return nil, false
}

func minMax(inputNumbers []int) (min int, max int) {
	min = inputNumbers[0]
	max = inputNumbers[0]

	for _, el := range inputNumbers {
		if el < min {
			min = el
		}

		if el > max {
			max = el
		}
	}

	return
}

func main() {
	var inputFilename string = "input.txt"
	var preambleLength int = 25

	if len(os.Args) > 1 {
		inputFilename = os.Args[1]
	}

	if len(os.Args) > 2 {
		var err error
		preambleLength, err = strconv.Atoi(os.Args[2])
		if err != nil {
			log.Fatal("Invalid premble length: ", os.Args[2], err)
		}
	}

	fmt.Println("Using input file: ", inputFilename)

	inputContents, err := contents(inputFilename)
	if err != nil {
		log.Fatal("Error reading ", inputFilename, err)
	}

	invalidNumbers := validateInput(inputContents, preambleLength)

	for _, invalid := range invalidNumbers {
		fmt.Printf("Found invalid value %v.\n", invalid)

		elements, foundSum := findTargetSum(invalid, inputContents)

		if foundSum {
			fmt.Printf("\tFound sum with consective elements: %v\n", elements)

			min, max := minMax(elements)
			fmt.Printf("\tMin: %v, Max: %v, Sum: %v\n", min, max, (min + max))
		}
	}
}
