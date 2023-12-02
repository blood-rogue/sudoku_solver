use itertools::Itertools;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Deserializer};

use crate::{
    bitset::{DigitSet, IndexSet},
    sudoku::{Cell, Digit},
    Puzzle,
};

fn de<'de, D>(d: D) -> Result<Vec<Digit>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(d)?.as_bytes().to_vec())
}

#[derive(Deserialize)]
struct TestPuzzle {
    #[serde(deserialize_with = "de")]
    problem: Vec<Digit>,
    solution: String,
}

fn solution_str(puzzle: &Puzzle) -> String {
    let mut cells = vec![0; 81];

    for ((row, col), cell) in puzzle
        .0
        .iter()
        .enumerate()
        .map(|(i, t)| ((i / 9, i % 9), t.clone()))
    {
        cells[row * 9 + col] = match cell {
            Cell::Solved(c) => c + b'0',
            Cell::Unsolved(_) => b'_',
        }
    }

    String::from_utf8(cells).unwrap()
}

fn test_puzzle_with_solution(difficulty: &str) {
    let test_data = serde_json::from_str::<Vec<TestPuzzle>>(
        &std::fs::read_to_string(format!("test/puzzles-{difficulty}.json")).unwrap(),
    )
    .unwrap();

    let mut solved = 0;

    for test_puzzle in &test_data {
        let mut puzzle = Puzzle::new_from_string(&test_puzzle.problem);

        if puzzle.solve() && solution_str(&puzzle) == test_puzzle.solution {
            solved += 1
        }
    }

    if solved != test_data.len() {
        panic!("solved {solved}/{}", test_data.len())
    }
}

#[test]
fn test_easy() {
    test_puzzle_with_solution("easy");
}

#[test]
fn test_medium() {
    test_puzzle_with_solution("medium");
}

#[test]
fn test_hard() {
    test_puzzle_with_solution("hard");
}

#[test]
fn test_expert() {
    test_puzzle_with_solution("expert")
}

#[test]
fn test_evil() {
    test_puzzle_with_solution("evil")
}

#[test]
fn test_combined() {
    test_puzzle_with_solution("combined")
}

#[test]
fn test_digitset() {
    let mut digit_set = DigitSet::new();

    digit_set.insert(1);
    digit_set.insert(3);
    digit_set.insert(9);

    assert_eq!(digit_set.into_iter().collect_vec(), vec![1, 3, 9]);

    digit_set.remove(3);

    assert_eq!(digit_set.into_iter().collect_vec(), vec![1, 9]);

    let other_set = DigitSet::from_iter(vec![1, 2, 3, 4]);

    assert_eq!(
        digit_set.difference(other_set).into_iter().collect_vec(),
        vec![9]
    );

    assert_eq!(
        digit_set.union(other_set).into_iter().collect_vec(),
        vec![1, 2, 3, 4, 9]
    );

    assert!(digit_set.contains(1));
}

#[test]
fn test_indexset() {
    let mut digit_set = IndexSet::new();

    digit_set.insert(1);
    digit_set.insert(3);
    digit_set.insert(9);

    assert_eq!(digit_set.into_iter().collect_vec(), vec![1, 3, 9]);

    let other_set = IndexSet::from_iter(vec![1, 2, 3, 4]);

    assert_eq!(
        digit_set.difference(other_set).into_iter().collect_vec(),
        vec![9]
    );

    assert_eq!(
        digit_set.intersection(other_set).into_iter().collect_vec(),
        vec![1, 3]
    );

    assert!(digit_set.contains(1));
}
