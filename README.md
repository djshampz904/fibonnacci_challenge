# Fibonacci Sequence Challenge

A simple Rust program that calculates and displays the Fibonacci number at a user-specified position in the sequence.

## About the Fibonacci Sequence

The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones, usually starting with 0 and 1:

```
0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
```

## How the Program Works

This program:
1. Prompts the user to enter a position in the Fibonacci sequence
2. Validates the input to ensure it's an integer
3. Calculates the Fibonacci number at that position using an iterative approach
4. Displays the result to the user

## Usage

To run the program:

```bash
cargo run
```

When prompted, enter a positive integer representing the position in the Fibonacci sequence you want to find.

## Example

```
What position of the fabonacci sequence are you looking for? (enter number)
7
Position 7: 8
```

## Implementation Details

The program uses an iterative approach rather than recursion to efficiently calculate Fibonacci numbers even for larger positions. The algorithm has a time complexity of O(n) where n is the position in the sequence.

## Note

Position numbering in this implementation starts with position 1 being 0 and position 2 being 1.
