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
    String::from_utf8(
        puzzle
            .cells
            .iter()
            .map(|cell| match cell {
                Cell::Solved(c) => c + b'0',
                Cell::Unsolved(_) => b'.',
            })
            .collect_vec(),
    )
    .unwrap()
}

fn test_puzzle_with_solution(difficulty: &str) {
    let test_data = serde_json::from_str::<Vec<TestPuzzle>>(
        &std::fs::read_to_string(format!("test/puzzles-{difficulty}.json")).unwrap(),
    )
    .unwrap();

    let mut solved = 0;

    for TestPuzzle { problem, solution } in &test_data {
        let mut puzzle = Puzzle::new_from_string(&problem);

        if puzzle.solve() {
            let sol = solution_str(&puzzle);
            if &sol == solution {
                solved += 1
            } else {
                panic!("got: {sol}, expected: {solution}")
            }
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
fn test_17_clue() {
    for line in std::fs::read_to_string("test/puzzles-17.txt")
        .unwrap()
        .lines()
    {
        Puzzle::new_from_string(line.as_bytes()).solve();
    }
}

#[test]
fn test_digitset() {
    let mut digit_set = DigitSet::new(0);

    digit_set.insert(1);
    digit_set.insert(3);
    digit_set.insert(9);

    assert_eq!(digit_set.into_iter().collect_vec(), vec![1, 3, 9]);

    digit_set.remove(3);

    assert_eq!(digit_set.into_iter().collect_vec(), vec![1, 9]);

    let other_set = DigitSet::from_iter(vec![1, 2, 3, 4]);

    assert_eq!(
        digit_set.union(other_set).into_iter().collect_vec(),
        vec![1, 2, 3, 4, 9]
    );

    assert!(digit_set.contains(1));
}

#[test]
fn test_indexset() {
    let mut index_set = IndexSet::new(0);

    index_set.insert(1);
    index_set.insert(3);
    index_set.insert(9);

    assert_eq!(index_set.into_iter().collect_vec(), vec![1, 3, 9]);

    index_set.remove(3);
    assert_eq!(index_set.into_iter().collect_vec(), vec![1, 9]);

    let other_set = IndexSet::from_iter(vec![1, 2, 3, 4]);

    assert_eq!(
        index_set.difference(other_set).into_iter().collect_vec(),
        vec![9]
    );

    assert_eq!(
        index_set.intersection(other_set).into_iter().collect_vec(),
        vec![1]
    );
}
