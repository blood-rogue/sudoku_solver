use crate::Board;

fn test_board(difficulty: &str) {
    let mut solved = 0;

    let data = std::fs::read_to_string(format!("test/boards/{difficulty}/solutions.txt")).unwrap();
    let solutions = data.lines().collect::<Vec<_>>();

    for i in 0..100 {
        let board_path = format!("test/boards/{difficulty}/board-{i}.txt");

        let rows = std::fs::read_to_string(board_path)
            .unwrap()
            .lines()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        let mut board = Board::new(&rows);

        if !board.is_valid() {
            panic!("invalid board");
        }

        if board.solve() && format!("{board:?}") == solutions[i] {
            solved += 1;
        }
    }

    if solved != 100 {
        panic!("solved {solved}/100")
    }
}

#[test]
pub fn test_easy() {
    test_board("easy");
}

#[test]
pub fn test_medium() {
    test_board("medium");
}

#[test]
pub fn test_hard() {
    test_board("hard");
}

#[test]
pub fn test_expert() {
    test_board("expert")
}

#[test]
pub fn test_evil() {
    test_board("evil")
}
