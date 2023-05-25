# Advent of Code 2015

### Day 1
Calculate on which floor santa ends up in. '(' means floor + 1 and ')' means floor - 1.
- ```(())``` and ```()()``` both result in floor 0.
- ```(((``` and ```(()(()(``` both result in floor 3.

### Day 2
Calculate the total wrapping paper required to wrap the given right rectangular prisms.
- Area is calculated using - 2*l*w + 2*w*h + 2*h*l.
- A little extra paper is required for each present: the area of the smallest side.

### Day 3
Delivering presents to an infinite two-dimensional grid of houses
- Input the program would be '>' 'v' '<' '>' that define the direction to travel.
- Return the number of houses that got at least one present.

### Day 4
Given input 'yzbqklnj', find out the smallest decimal number to append to this input which results in a md5 hash that starts with five 0s.

### Day 5
Count the number of strings that are "nice". A string is nice if --
- It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
- It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
- It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

### Day 6
In a two dimensional grid, turn on or off the cells based on given instructions. Instructions are in the following format --
- turn on 887,9 through 959,629
- turn off 539,243 through 559,965
- toggle 720,196 through 897,994

### Day 7
Perform bitwise operations based on instructions and find out what the final signal on the wire named "a" is.
- 127 -> x
- 456 -> y
- x AND y -> a
The wire "a" will contain contain signal 72