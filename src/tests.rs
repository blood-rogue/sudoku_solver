use crate::Puzzle;

fn test_puzzle(difficulty: &str) {
    let mut solved = 0;

    let data = std::fs::read_to_string(format!("test/puzzles/{difficulty}/solutions.txt")).unwrap();
    let solutions = data.lines().collect::<Vec<_>>();

    for i in 0..100 {
        let puzzle_path = format!("test/puzzles/{difficulty}/puzzle-{i}.txt");

        let rows = std::fs::read_to_string(puzzle_path)
            .unwrap()
            .lines()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        let mut puzzle = Puzzle::new(&rows);

        if !puzzle.is_valid() {
            panic!("invalid puzzle");
        }

        if puzzle.solve() && format!("{puzzle:?}") == solutions[i] {
            solved += 1;
        }
    }

    if solved != 100 {
        panic!("solved {solved}/100")
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
