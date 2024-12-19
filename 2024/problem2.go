package main 

import (
	"strconv"
	"fmt"
	"bufio"
	"strings"
	"os"
)

func main() {
	safeReports := 0
	input, err := os.Open("input/problem2_input.txt")
	if err != nil {
		panic(err)
	}
	defer input.Close()

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), " ")

		// find out whether the pattern is ascending or not
		ascending := false
		if convertToInt(line[0]) < convertToInt(line[1]) {
			ascending = true
		}

		for i:=1; i<len(line); i++ {
			prev := convertToInt(line[i-1])
			curr := convertToInt(line[i])
			if abs(curr, prev) > 3 || abs(curr, prev) == 0 {
				break
			}
			if ascending {
				if curr < prev {
					break
				}
			} else {
				if prev < curr {
					break
				}
			}
			if i == len(line) - 1 {
				safeReports += 1
			}
		}
	}
	fmt.Println(safeReports)
}


func convertToInt(input string) int {
	integer, _ := strconv.Atoi(input)
	return integer
}

func abs(x, y int) int {
	diff := x - y
	if diff > 0 {
		return diff
	}
	return -diff
}
