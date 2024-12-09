package main 

import (
	"os"
	"bufio"
	"strings"
	"strconv"	
	"fmt"
)

func main() {
	input, err := os.Open("input/problem1_input.txt")
	if err != nil {
		panic(err)
	}
	defer input.Close()

	left := make([]int, 0)
	right:= make([]int, 0)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		temp := strings.Split(scanner.Text(), " ")
		left_num, _ := strconv.Atoi(temp[0])
		right_num, _ := strconv.Atoi(temp[3])
		left = append(left, left_num)
		right = append(right, right_num)
	}

	total_distance := 0

	// find the minimum numbers in both arrays, find the distance and append to total_distance
	for i:=0; i<len(left); i++ {
		leftMin := -1
		rightMin := -1
		leftMinIndex := 0
		rightMinIndex := 0

		for j:=0; j<len(left); j++ {
			if leftMin < left[j] && left[j] != -1 {
				leftMin = left[j]
				leftMinIndex = j
			}
			if rightMin < right[j] && right[j] != -1 {
				rightMin = right[j]
				rightMinIndex = j
			}
		}
		// get absolute difference
		diff := leftMin - rightMin
		if diff < 0 {
			diff = -diff
		}

		total_distance += diff 
		left[leftMinIndex] = -1
		right[rightMinIndex] = -1
	}

	fmt.Println(total_distance)
}
	
