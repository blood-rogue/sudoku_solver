use serde::{Deserialize, Deserializer};

use crate::{sudoku::Cell, Puzzle};

fn de<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(d)?.as_bytes().to_vec())
}

#[derive(Deserialize)]
struct TestPuzzle {
    #[serde(deserialize_with = "de")]
    problem: Vec<u8>,
    solution: String,
}

fn solution_str(puzzle: &Puzzle) -> String {
    let mut cells = vec![0u8; 81];

    for (&(row, col), cell) in &puzzle.cells {
        cells[row * 9 + col] = match cell {
            Cell::Solved(c) => *c,
            Cell::Unsolved(_) => b'_',
        }
    }

    String::from_utf8(cells).unwrap()
}

fn test_puzzle(difficulty: &str) {
    let test_data = serde_json::from_str::<Vec<TestPuzzle>>(
        &std::fs::read_to_string(format!("test/puzzles-{difficulty}.json")).unwrap(),
    )
    .unwrap();

    let mut solved = 0;

    for test_puzzle in &test_data {
        let mut puzzle = Puzzle::new(
            &test_puzzle
                .problem
                .chunks_exact(9)
                .map(|chunk| chunk.to_vec())
                .collect::<Vec<_>>(),
        );

        if !puzzle.is_valid() {
            panic!("invalid puzzle");
        }

        if puzzle.solve() && solution_str(&puzzle) == test_puzzle.solution {
            solved += 1;
        }
    }

    if solved != test_data.len() {
        panic!("solved {solved}/{}", test_data.len())
    }
}

#[test]
pub fn test_easy() {
    test_puzzle("easy");
}

#[test]
pub fn test_medium() {
    test_puzzle("medium");
}

#[test]
pub fn test_hard() {
    test_puzzle("hard");
}

#[test]
pub fn test_expert() {
    test_puzzle("expert")
}

#[test]
pub fn test_evil() {
    test_puzzle("evil")
}
