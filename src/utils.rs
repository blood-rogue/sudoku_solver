use std::fmt::Display;

use crate::{
    bitset::{BitSet, DigitSet, IndexSet},
    sudoku::{Cell, Index, Puzzle},
};

pub const RANGES: [IndexSet; 27] = [
    IndexSet::new(511),
    IndexSet::new(261632),
    IndexSet::new(133955584),
    IndexSet::new(68585259008),
    IndexSet::new(35115652612096),
    IndexSet::new(17979214137393152),
    IndexSet::new(9205357638345293824),
    IndexSet::new(4713143110832790437888),
    IndexSet::new(2413129272746388704198656),
    IndexSet::new(4731607904558235517441),
    IndexSet::new(9463215809116471034882),
    IndexSet::new(18926431618232942069764),
    IndexSet::new(37852863236465884139528),
    IndexSet::new(75705726472931768279056),
    IndexSet::new(151411452945863536558112),
    IndexSet::new(302822905891727073116224),
    IndexSet::new(605645811783454146232448),
    IndexSet::new(1211291623566908292464896),
    IndexSet::new(1838599),
    IndexSet::new(14708792),
    IndexSet::new(117670336),
    IndexSet::new(246772580483072),
    IndexSet::new(1974180643864576),
    IndexSet::new(15793445150916608),
    IndexSet::new(33121255085135066300416),
    IndexSet::new(264970040681080530403328),
    IndexSet::new(2119760325448644243226624),
];

pub const ROWS: [u128; 9] = [
    511,
    261632,
    133955584,
    68585259008,
    35115652612096,
    17979214137393152,
    9205357638345293824,
    4713143110832790437888,
    2413129272746388704198656,
];

pub const COLS: [u128; 9] = [
    4731607904558235517441,
    9463215809116471034882,
    18926431618232942069764,
    37852863236465884139528,
    75705726472931768279056,
    151411452945863536558112,
    302822905891727073116224,
    605645811783454146232448,
    1211291623566908292464896,
];

pub const BOXES: [u128; 9] = [
    1838599,
    14708792,
    117670336,
    246772580483072,
    1974180643864576,
    15793445150916608,
    33121255085135066300416,
    264970040681080530403328,
    2119760325448644243226624,
];

const BOX_MAPPING: [usize; 81] = [
    0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4,
    4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 6,
    6, 6, 7, 7, 7, 8, 8, 8, 6, 6, 6, 7, 7, 7, 8, 8, 8,
];

pub const fn ranges_of(idx: Index) -> IndexSet {
    IndexSet::new(ROWS[idx / 9] | COLS[idx % 9] | BOXES[BOX_MAPPING[idx]])
}

pub const FULL_SET: BitSet<u8> = DigitSet::new(0b0000_0011_1111_1110);
pub const UNSOLVED_CELL: Cell = Cell::Unsolved(FULL_SET);

impl Display for Puzzle {
    fn fmt(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(buf, "╭───┬───┬───┬───┬───┬───┬───┬───┬───╮")?;

        let mut puzzle = [[0; 9]; 9];
        for (i, cell) in self.cells.iter().enumerate() {
            puzzle[i / 9][i % 9] = match cell {
                Cell::Solved(v) => *v,
                Cell::Unsolved(_) => b' ',
            };
        }

        let [parts @ .., last] = puzzle;
        for row in parts {
            writeln!(
                buf,
                "│ {} │ {} │ {} │ {} │ {} │ {} │ {} │ {} │ {} │\n├───┼───┼───┼───┼───┼───┼───┼───┼───┤",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8]
            )?;
        }

        write!(
            buf,
            "│ {} │ {} │ {} │ {} │ {} │ {} │ {} │ {} │ {} │\n╰───┴───┴───┴───┴───┴───┴───┴───┴───╯",
            last[0], last[1], last[2], last[3], last[4], last[5], last[6], last[7], last[8]
        )?;

        Ok(())
    }
}
