use pretty_assertions::assert_eq;
use serde::{Deserialize, Deserializer};

use crate::{
    bitset::{DigitSet, IndexSet},
    combination::index,
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
                Cell::Solved(c) => *c + b'0',
                Cell::Unsolved(_) => b'.',
            })
            .collect::<Vec<_>>(),
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
        let mut puzzle = Puzzle::new_from_string(problem);

        if puzzle.solve() {
            let sol = solution_str(&puzzle);
            if &sol == solution {
                solved += 1;
            } else {
                panic!("got: {sol}\nexpected: {solution}")
            }
        }
    }

    let n = test_data.len();
    assert_eq!(solved, n, "solved {solved}/{n}",);
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
    test_puzzle_with_solution("expert");
}

#[test]
fn test_evil() {
    test_puzzle_with_solution("evil");
}

#[test]
fn test_combined() {
    test_puzzle_with_solution("combined");
}

#[test]
fn test_digitset() {
    let digit_set = DigitSet::new(0b0000_0010_0000_1010);
    assert_eq!(digit_set.into_iter().collect::<Vec<_>>(), vec![1, 3, 9]);

    let other_set = DigitSet::from_iter(vec![1, 2, 3, 4]);

    assert_eq!(
        digit_set.union(other_set).into_iter().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 9]
    );
}

#[test]
fn test_indexset() {
    let mut index_set = IndexSet::new(0);

    index_set.insert(1);
    index_set.insert(3);
    index_set.insert(9);

    assert_eq!(index_set.into_iter().collect::<Vec<_>>(), vec![1, 3, 9]);

    index_set.remove(3);
    assert_eq!(index_set.into_iter().collect::<Vec<_>>(), vec![1, 9]);

    let other_set = IndexSet::from_iter(vec![1, 2, 3, 4]);

    assert_eq!(
        index_set
            .difference(other_set)
            .into_iter()
            .collect::<Vec<_>>(),
        vec![9]
    );

    assert_eq!(
        index_set
            .intersection(other_set)
            .into_iter()
            .collect::<Vec<_>>(),
        vec![1]
    );
}

#[test]
fn test_pdep() {
    let mut index_set = IndexSet::new(0);

    index_set.insert(1);
    index_set.insert(3);
    index_set.insert(9);
    index_set.insert(80);

    assert_eq!(index(index_set.0, 0), Some(1));
    assert_eq!(index(index_set.0, 1), Some(3));
    assert_eq!(index(index_set.0, 2), Some(9));
    assert_eq!(index(index_set.0, 3), Some(80));
    assert_eq!(index(index_set.0, 4), None);
}
