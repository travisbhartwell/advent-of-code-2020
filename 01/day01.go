package main

import (
	"bufio"
	"errors"
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

func findSum(targetSum int, operandCount int, numbers []int) ([]int, error) {
	// fmt.Printf("%v: Target Sum: %v, Operand Count: %v, Numbers: %v\n", operandCount, targetSum, operandCount, numbers)

	var operands []int = make([]int, 0, operandCount)
	var numberCount = len(numbers)

	for index, value := range numbers {
		if operandCount == 1 && value == targetSum {
			operands = append(operands, value)
			// fmt.Printf("%v: %v == %v and operandCount == 1; operands = %v\n", operandCount, value, targetSum, operands)
			break
		}

		if operandCount > 1 && index < (numberCount-1) {
			// fmt.Printf("%v: operandCount(%v) > 1 and index(%v) < (numberCount (%v) - 1)\n", operandCount, operandCount, index, numberCount)
			recOperands, err := findSum(targetSum-value, operandCount-1, numbers[index:])
			if recOperands != nil && err == nil {
				// fmt.Printf("%v: recOperands: %v\n", operandCount, recOperands)
				operands = append(operands, value)
				operands = append(operands, recOperands...)
				// fmt.Printf("%v: operands: %v\n", operandCount, operands)
				break
			}
		}
	}

	// fmt.Printf("%v: Returning, operands = %v\n", operandCount, operands)

	if len(operands) == 0 {
		return nil, errors.New("no operands matching sum found")
	}

	return operands, nil
}

func product(numbers []int) (result int) {
	result = 1

	for _, value := range numbers {
		result *= value
	}

	return
}

func main() {
	var inputFilename string = "input.txt"

	if len(os.Args) > 1 {
		inputFilename = os.Args[1]
	}

	inputNumbers, err := contents(inputFilename)

	if err != nil {
		log.Fatal("Error reading ", inputFilename, ":", err)
	}

	matching, err := findSum(2020, 2, inputNumbers)

	if err != nil {
		log.Fatal("No matching operands found.")
	}

	fmt.Printf("Part 1: \nFound operands adding up to 2020: %v with product %v.\n", matching, product(matching))

	matching, err = findSum(2020, 3, inputNumbers)

	if err != nil {
		log.Fatal("No matching operands found.")
	}

	fmt.Printf("Part 2: \nFound operands adding up to 2020: %v with product %v.\n", matching, product(matching))
}
