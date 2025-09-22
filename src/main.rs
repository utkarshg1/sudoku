use clap::Parser;
use csv::ReaderBuilder;
use csv::Writer;
use std::error::Error;
use std::time::Instant;

/// Simple Sudoku Solver
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Sudoku CSV file
    #[arg(short, long)]
    input: String,

    /// Optional output CSV file for the solved Sudoku
    #[arg(short, long)]
    output: Option<String>,
}

fn is_valid(board: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    // Row check
    for x in 0..9 {
        if board[row][x] == num {
            return false;
        }
    }

    // Column check
    for y in 0..9 {
        if board[y][col] == num {
            return false;
        }
    }

    // 3x3 box check
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[box_row + i][box_col + j] == num {
                return false;
            }
        }
    }

    true
}

fn solve(board: &mut [[u8; 9]; 9]) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                for num in 1..=9 {
                    if is_valid(board, row, col, num) {
                        board[row][col] = num;
                        if solve(board) {
                            return true;
                        }
                        board[row][col] = 0; // backtrack
                    }
                }
                return false;
            }
        }
    }
    true
}

fn load_sudoku_from_csv(filename: &str) -> Result<[[u8; 9]; 9], Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut board = [[0u8; 9]; 9];

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        for (j, val) in record.iter().enumerate() {
            board[i][j] = val.parse::<u8>().unwrap_or(0);
        }
    }
    Ok(board)
}

fn print_board(board: &[[u8; 9]; 9]) {
    for (i, row) in board.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            println!("------+-------+------");
        }
        for (j, &val) in row.iter().enumerate() {
            if j % 3 == 0 && j != 0 {
                print!("| ");
            }
            print!("{} ", if val == 0 { '.' } else { (val + b'0') as char });
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut board = load_sudoku_from_csv(&args.input)?;

    println!("Original Sudoku:");
    print_board(&board);

    let start = Instant::now();
    let solved = solve(&mut board);
    let duration = start.elapsed();

    if solved {
        println!("\nSolved Sudoku:");
        print_board(&board);

        // If output file is specified, write solved board to CSV
        if let Some(output_file) = args.output {
            let mut wtr = Writer::from_path(output_file)?;
            for row in &board {
                wtr.write_record(row.iter().map(|v| v.to_string()))?;
            }
            wtr.flush()?;
            println!("\nSolved Sudoku saved to CSV.");
        }
    } else {
        println!("No solution exists.");
    }

    println!("\nTime taken: {:.3?}", duration);

    Ok(())
}
