use hashbrown::HashSet;
use std::fmt::Display;

use itertools::Itertools;

pub type Idx = usize;

pub const ROWS: [[usize; 9]; 9] = [
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

pub const COLS: [[usize; 9]; 9] = [
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

pub const BOXES: [[usize; 9]; 9] = [
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

const BOX_MAPPING: [usize; 81] = [
    0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4,
    4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 6,
    6, 6, 7, 7, 7, 8, 8, 8, 6, 6, 6, 7, 7, 7, 8, 8, 8,
];

#[inline]
pub const fn row_of(idx: Idx) -> [Idx; 9] {
    ROWS[idx / 9]
}

#[inline]
pub const fn col_of(idx: Idx) -> [Idx; 9] {
    COLS[idx % 9]
}

#[inline]
pub const fn box_of(idx: Idx) -> [Idx; 9] {
    BOXES[BOX_MAPPING[idx]]
}

const FULL_SET: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

const HORZ_BAR: &str = "─";
const VERT_BAR: &str = "│";

const TOP_L_CORNER: &str = "╭";
const BOT_L_CORNER: &str = "╰";

const TOP_R_CORNER: &str = "╮";
const BOT_R_CORNER: &str = "╯";

const VERT_L_JOINT: &str = "├";
const VERT_R_JOINT: &str = "┤";

const HORZ_T_JOINT: &str = "┬";
const HORZ_B_JOINT: &str = "┴";

const INTERSECTION: &str = "┼";

#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Unsolved(HashSet<u8>),
    Solved(u8),
}

impl Cell {
    const fn is_empty(&self) -> bool {
        matches!(self, Self::Unsolved(_))
    }

    fn cell_values(&self) -> HashSet<u8> {
        match self {
            Self::Unsolved(set) => set.clone(),
            Self::Solved(_) => HashSet::new(),
        }
    }
}

#[derive(Clone)]
struct PreSet {
    numbers: HashSet<u8>,
    cells: HashSet<Idx>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Puzzle(pub Vec<Cell>);

#[inline]
fn remove_any(x: &HashSet<Idx>, xs: &[HashSet<Idx>]) -> Vec<HashSet<Idx>> {
    xs.iter()
        .filter(|y| y.intersection(x).count() == 0)
        .cloned()
        .collect()
}

impl Puzzle {
    pub fn new_from_string(s: &[u8]) -> Self {
        Self(
            s.iter()
                .map(|ch| {
                    let ch = *ch - b'0';
                    if FULL_SET.contains(&ch) {
                        Cell::Solved(ch)
                    } else {
                        Cell::Unsolved(HashSet::from(FULL_SET))
                    }
                })
                .collect(),
        )
    }

    pub fn is_valid(&self) -> bool {
        [ROWS, COLS, BOXES]
            .concat()
            .iter()
            .map(|range| {
                let mut unique = HashSet::new();

                range
                    .iter()
                    .filter(|&&idx| self.0[idx].is_empty())
                    .all(|value| unique.insert(value))
            })
            .all(|v| v)
    }

    fn find_empty_cell(&self) -> Option<(Idx, HashSet<u8>)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| match cell {
                Cell::Solved(_) => None,
                Cell::Unsolved(set) => Some((i, set.clone())),
            })
            .min_by(|(_, set1), (_, set2)| set1.len().cmp(&set2.len()))
    }

    fn cross_out(&mut self, indices: Vec<Idx>, value: u8) {
        for idx in indices {
            match self.0.get_mut(idx).unwrap() {
                Cell::Solved(_) => {}
                Cell::Unsolved(set) => {
                    set.remove(&value);
                }
            }
        }
    }

    fn markup(&mut self) {
        for idx in (0..81).filter(|&idx| self.0[idx].is_empty()).collect_vec() {
            let peer_markup = [row_of(idx), col_of(idx), box_of(idx)]
                .concat()
                .into_iter()
                .filter_map(|idx| match self.0[idx] {
                    Cell::Solved(v) => Some(v),
                    Cell::Unsolved(_) => None,
                })
                .collect::<HashSet<_>>();

            let values = self.0[idx]
                .cell_values()
                .into_iter()
                .filter(|value| !peer_markup.contains(value))
                .collect::<HashSet<_>>();

            if let Cell::Unsolved(markup) = self.0.get_mut(idx).unwrap() {
                *markup = values;
            }
        }

        self.fill_forced_cells();
    }

    fn fill_forced_cells(&mut self) -> bool {
        let mut cont = true;
        let mut modified = false;

        while cont {
            cont = false;

            for idx in (0..81).filter(|&idx| self.0[idx].is_empty()).collect_vec() {
                let empty_cell_values = self.0[idx].cell_values();

                if empty_cell_values.len() == 1 {
                    let value = *empty_cell_values.iter().next().unwrap();
                    self.0[idx] = Cell::Solved(value);

                    cont = true;
                    modified = true;

                    self.cross_out([row_of(idx), col_of(idx), box_of(idx)].concat(), value);
                }
            }
        }

        modified
    }

    fn find_pre_sets(&mut self, range: &[Idx]) -> Vec<PreSet> {
        let pre_sets = |n: usize| -> Vec<PreSet> {
            fn go(
                sets: &[HashSet<Idx>],
                acc: Vec<PreSet>,
                puzzle: &Puzzle,
                n: usize,
            ) -> Vec<PreSet> {
                let [x, xs @ ..] = sets else {
                    return acc;
                };

                let nums = x.iter().fold(HashSet::new(), |acc, &value| {
                    puzzle.0[value].cell_values().union(&acc).copied().collect()
                });

                if nums.len() == n {
                    let mut acc = acc;
                    acc.push(PreSet {
                        numbers: nums,
                        cells: x.clone(),
                    });

                    go(&remove_any(x, xs), acc, puzzle, n)
                } else {
                    go(xs, acc, puzzle, n)
                }
            }

            let combinations = range
                .iter()
                .filter(|&&idx| self.0[idx].is_empty())
                .copied()
                .combinations(n)
                .map(HashSet::from_iter)
                .collect_vec();

            go(&combinations, Vec::new(), self, n)
        };

        [pre_sets(2), pre_sets(3), pre_sets(4)].concat()
    }

    fn apply_presets(&mut self) {
        for indices in [ROWS, COLS, BOXES].concat() {
            for ps in self.find_pre_sets(&indices) {
                let indices = HashSet::from(indices);

                for cell_idx in indices.difference(&ps.cells) {
                    if let Some(Cell::Unsolved(set)) = self.0.get_mut(*cell_idx) {
                        *set = set.difference(&ps.numbers).copied().collect();
                    }
                }
            }
        }
    }

    fn simplify(&mut self) {
        let mut modified_previous = true;

        while modified_previous {
            self.markup();
            self.apply_presets();

            modified_previous = self.fill_forced_cells();
        }
    }

    pub fn solve(&mut self) -> bool {
        self.simplify();

        if let Some((idx, cell_markup)) = self.find_empty_cell() {
            for possibility in cell_markup {
                self.0[idx] = Cell::Solved(possibility);
                let prev_cells = self.0.clone();

                if self.solve() {
                    return true;
                }

                self.0 = prev_cells;
            }

            false
        } else {
            true
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(buf, "{TOP_L_CORNER}")?;
        for _ in 0..8 {
            write!(buf, "{}{HORZ_T_JOINT}", HORZ_BAR.repeat(3))?;
        }

        writeln!(buf, "{}{TOP_R_CORNER}", HORZ_BAR.repeat(3))?;

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
                    write!(buf, "{}", HORZ_BAR.repeat(3))?;
                    if j != row_len - 1 {
                        write!(buf, "{INTERSECTION}")?;
                    }
                }
                writeln!(buf, "{VERT_R_JOINT}")?;
            }
        }

        write!(buf, "{BOT_L_CORNER}")?;
        for _ in 0..8 {
            write!(buf, "{}{HORZ_B_JOINT}", HORZ_BAR.repeat(3))?;
        }

        writeln!(buf, "{}{BOT_R_CORNER}", HORZ_BAR.repeat(3))?;

        Ok(())
    }
}
