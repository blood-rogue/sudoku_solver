use std::fmt::Display;

use crate::{
    bitset::DigitSet,
    sudoku::{Cell, Index, Puzzle},
};

pub const ROWS: [[Index; 9]; 9] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],
];

pub const COLS: [[Index; 9]; 9] = [
    [0, 9, 18, 27, 36, 45, 54, 63, 72],
    [1, 10, 19, 28, 37, 46, 55, 64, 73],
    [2, 11, 20, 29, 38, 47, 56, 65, 74],
    [3, 12, 21, 30, 39, 48, 57, 66, 75],
    [4, 13, 22, 31, 40, 49, 58, 67, 76],
    [5, 14, 23, 32, 41, 50, 59, 68, 77],
    [6, 15, 24, 33, 42, 51, 60, 69, 78],
    [7, 16, 25, 34, 43, 52, 61, 70, 79],
    [8, 17, 26, 35, 44, 53, 62, 71, 80],
];

pub const BOXES: [[Index; 9]; 9] = [
    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

const BOX_MAPPING: [Index; 81] = [
    0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4,
    4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 6,
    6, 6, 7, 7, 7, 8, 8, 8, 6, 6, 6, 7, 7, 7, 8, 8, 8,
];

#[inline]
pub const fn row_of(idx: Index) -> [Index; 9] {
    ROWS[idx / 9]
}

#[inline]
pub const fn col_of(idx: Index) -> [Index; 9] {
    COLS[idx % 9]
}

#[inline]
pub const fn box_of(idx: Index) -> [Index; 9] {
    BOXES[BOX_MAPPING[idx]]
}

pub const FULL_SET: DigitSet = DigitSet::new(0b0000_0011_1111_1110);

pub const HORZ_BAR: &str = "───";
pub const VERT_BAR: char = '│';

pub const TOP_L_CORNER: char = '╭';
pub const BOT_L_CORNER: char = '╰';

pub const TOP_R_CORNER: char = '╮';
pub const BOT_R_CORNER: char = '╯';

pub const VERT_L_JOINT: char = '├';
pub const VERT_R_JOINT: char = '┤';

pub const HORZ_T_JOINT: char = '┬';
pub const HORZ_B_JOINT: char = '┴';

pub const INTERSECTION: char = '┼';

impl Display for Puzzle {
    fn fmt(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(buf, "{TOP_L_CORNER}")?;
        for _ in 0..8 {
            write!(buf, "{HORZ_BAR}{HORZ_T_JOINT}")?;
        }

        writeln!(buf, "{HORZ_BAR}{TOP_R_CORNER}")?;

        let mut puzzle = [[0; 9]; 9];

        for (i, cell) in self.0.iter().enumerate() {
            puzzle[i / 9][i % 9] = match cell {
                Cell::Solved(v) => *v,
                Cell::Unsolved(_) => b' ',
            };
        }

        for (i, row) in puzzle.iter().enumerate() {
            write!(buf, "{VERT_BAR}")?;
            for data in row {
                write!(buf, " {data} {VERT_BAR}")?;
            }
            writeln!(buf)?;
            if i != 8 {
                write!(buf, "{VERT_L_JOINT}")?;
                let row_len = row.len();
                for (j, _) in row.iter().enumerate() {
                    write!(buf, "{HORZ_BAR}")?;
                    if j != row_len - 1 {
                        write!(buf, "{INTERSECTION}")?;
                    }
                }
                writeln!(buf, "{VERT_R_JOINT}")?;
            }
        }

        write!(buf, "{BOT_L_CORNER}")?;
        for _ in 0..8 {
            write!(buf, "{HORZ_BAR}{HORZ_B_JOINT}")?;
        }

        writeln!(buf, "{HORZ_BAR}{BOT_R_CORNER}")?;

        Ok(())
    }
}
