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
	total_similarity := 0
	
	for i:=0; i<len(left); i++ {
		if left[i] == -1 {
			continue
		}
		currentNumber := left[i]
		leftCount := 0
		rightCount := 0

		for j:=0; j<len(right); j++ {
			if currentNumber == right[j] {
				rightCount += 1
				right[j] = -1
			}
		}

		for k:=i; k<len(left); k++ {
			if currentNumber == left[k] {
				leftCount += 1
				left[k] = -1
			}
		}

		total_similarity += currentNumber * leftCount * rightCount
	}
	fmt.Println(total_similarity)
}
