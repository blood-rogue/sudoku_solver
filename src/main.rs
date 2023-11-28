mod consts;
mod sudoku;

#[cfg(test)]
mod tests;

use std::io::BufRead;

use sudoku::Puzzle;

fn main() {
    let mut rows = Vec::new();

    for _ in 0..9 {
        let mut inp = String::new();
        std::io::stdin().lock().read_line(&mut inp).unwrap();

        rows.push(inp.trim().as_bytes().to_vec());
    }

    let mut puzzle = Puzzle::new(&rows);

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
