mod bench;
mod bitset;
mod combination;
mod sudoku;
mod utils;

#[cfg(test)]
mod tests;

use std::io::BufRead;
use sudoku::Puzzle;

fn main() {
    if Some("bench") == std::env::args().nth(1).as_deref() {
        bench::puzzle();
    } else {
        let mut inp = String::with_capacity(85);
        std::io::stdin().lock().read_line(&mut inp).unwrap();

        if inp.trim().len() != 81 {
            println!("Invalid input");
            return;
        }

        let mut puzzle = Puzzle::new_from_string(inp.trim().as_bytes());

        if puzzle.solve() {
            println!("{puzzle}");
        } else {
            println!("Couldn't solve puzzle");
        }
    }
}
