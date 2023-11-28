mod consts;
mod sudoku;

#[cfg(test)]
mod tests;

use std::io::BufRead;

use sudoku::Board;

fn main() {
    let mut rows = Vec::new();

    for _ in 0..9 {
        let mut inp = String::new();
        std::io::stdin().lock().read_line(&mut inp).unwrap();

        rows.push(inp.trim().to_string());
    }

    let mut board = Board::new(&rows);

    if board.is_valid() {
        if board.solve() {
            println!("{board}");
        } else {
            println!("Couldn't solve board");
        }
    } else {
        println!("invalid board");
    }
}
