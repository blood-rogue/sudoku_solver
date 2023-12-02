mod bitset;
mod sudoku;

#[cfg(test)]
mod tests;

use std::io::BufRead;
use sudoku::Puzzle;

fn main() {
    let mut inp = String::new();
    std::io::stdin().lock().read_line(&mut inp).unwrap();

    let mut puzzle = Puzzle::new_from_string(inp.trim().as_bytes());

    if puzzle.is_valid() {
        if puzzle.solve() {
            println!("{puzzle}");
        } else {
            println!("Couldn't solve puzzle");
        }
    } else {
        println!("invalid puzzle");
    }
}
