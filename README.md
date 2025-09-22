# Sudoku Solver

A simple **Sudoku Solver** written in **Rust** that reads a Sudoku puzzle from a CSV file, solves it using a backtracking algorithm, and optionally saves the solved puzzle to a CSV file.

---

## Features

* Reads Sudoku puzzles from CSV files.
* Solves Sudoku using backtracking.
* Prints the original and solved Sudoku to the console.
* Optionally saves the solved Sudoku to a CSV file.
* Measures and displays the time taken to solve the puzzle.

---

## Installation

1. **Clone the repository**:

```bash
git clone <repository-url>
cd <repository-folder>
```

2. **Build the project using Cargo**:

```bash
cargo build --release
```

3. **Run the executable**:

```bash
cargo run -- --input path/to/sudoku.csv
```

---

## Usage

```bash
cargo run -- --input <input_csv> [--output <output_csv>]
```

### Arguments

| Argument | Short | Description                                   |
| -------- | ----- | --------------------------------------------- |
| `input`  | `-i`  | Path to the input Sudoku CSV file (required)  |
| `output` | `-o`  | Path to save the solved Sudoku CSV (optional) |

---

## Input CSV Format

* The input file must be a **9x9 grid** of numbers.
* Empty cells can be represented by `0` or left blank.
* Example:

```csv
5,3,0,0,7,0,0,0,0
6,0,0,1,9,5,0,0,0
0,9,8,0,0,0,0,6,0
8,0,0,0,6,0,0,0,3
4,0,0,8,0,3,0,0,1
7,0,0,0,2,0,0,0,6
0,6,0,0,0,0,2,8,0
0,0,0,4,1,9,0,0,5
0,0,0,0,8,0,0,7,9
```

---

## Example

Run the solver:

```bash
cargo run -- --input sudoku.csv --output solved_sudoku.csv
```

**Output (Console)**:

```
Original Sudoku:
5 3 . | . 7 . | . . .
6 . . | 1 9 5 | . . .
. 9 8 | . . . | . 6 .
------+-------+------
8 . . | . 6 . | . . 3
4 . . | 8 . 3 | . . 1
7 . . | . 2 . | . . 6
------+-------+------
. 6 . | . . . | 2 8 .
. . . | 4 1 9 | . . 5
. . . | . 8 . | . 7 9

Solved Sudoku:
5 3 4 | 6 7 8 | 9 1 2
6 7 2 | 1 9 5 | 3 4 8
1 9 8 | 3 4 2 | 5 6 7
------+-------+------
8 5 9 | 7 6 1 | 4 2 3
4 2 6 | 8 5 3 | 7 9 1
7 1 3 | 9 2 4 | 8 5 6
------+-------+------
9 6 1 | 5 3 7 | 2 8 4
2 8 7 | 4 1 9 | 6 3 5
3 4 5 | 2 8 6 | 1 7 9

Time taken: 0.001s
Solved Sudoku saved to CSV.
```

---

## Dependencies

* [clap](https://crates.io/crates/clap) — Command-line argument parser
* [csv](https://crates.io/crates/csv) — CSV reader and writer
