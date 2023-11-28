use hashbrown::{HashMap, HashSet};
use std::fmt::{Debug, Display};

use itertools::Itertools;
use owo_colors::OwoColorize;

use crate::consts::{box_of, col_of, row_of, BOXES, COLS, ROWS};

pub type Idx = (usize, usize);

const FULL_SET: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

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
enum Cell {
    Unsolved(HashSet<char>),
    Solved(char),
}

impl Cell {
    const fn is_empty(&self) -> bool {
        matches!(self, Self::Unsolved(_))
    }

    const fn as_option(&self) -> Option<char> {
        match self {
            Self::Solved(v) => Some(*v),
            Self::Unsolved(_) => None,
        }
    }

    fn cell_values(&self) -> HashSet<char> {
        match self {
            Self::Unsolved(set) => set.clone(),
            Self::Solved(_) => HashSet::new(),
        }
    }

    fn cell_values_mut(&mut self) -> &mut HashSet<char> {
        match self {
            Self::Solved(_) => unreachable!(),
            Self::Unsolved(set) => set,
        }
    }
}

#[derive(Clone)]
struct PreSet {
    numbers: HashSet<char>,
    cells: HashSet<Idx>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    cells: HashMap<Idx, Cell>,
    prefilled: Vec<Idx>,
}

#[inline]
fn remove_any(x: &HashSet<Idx>, xs: &[HashSet<Idx>]) -> Vec<HashSet<Idx>> {
    xs.iter()
        .filter(|y| y.intersection(x).count() == 0)
        .cloned()
        .collect()
}

impl Board {
    pub fn new(input: &[String]) -> Self {
        let mut cells = HashMap::new();
        let mut prefilled = Vec::new();

        for (row_number, row) in input.iter().enumerate() {
            for (col_number, ch) in row.char_indices() {
                cells.insert(
                    (row_number, col_number),
                    if FULL_SET.contains(&ch) {
                        prefilled.push((col_number, row_number));
                        Cell::Solved(ch)
                    } else {
                        Cell::Unsolved(HashSet::new())
                    },
                );
            }
        }

        Self { cells, prefilled }
    }

    pub fn is_valid(&self) -> bool {
        let cell_len_ok = || self.cells.len() == 81;

        let cols_len_ok = || {
            self.cells
                .keys()
                .map(|&(_, col)| col)
                .collect::<HashSet<_>>()
                == HashSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8])
        };

        let rows_len_ok = || {
            self.cells
                .keys()
                .map(|&(row, _)| row)
                .collect::<HashSet<_>>()
                == HashSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8])
        };

        let row_values_ok = || {
            ROWS.iter()
                .map(|row| {
                    let mut unique = HashSet::new();

                    row.iter()
                        .filter_map(|idx| self.cells[idx].as_option())
                        .all(|value| unique.insert(value))
                })
                .all(|v| v)
        };

        let col_values_ok = || {
            COLS.iter()
                .map(|col| {
                    let mut unique = HashSet::new();

                    col.iter()
                        .filter_map(|idx| self.cells[idx].as_option())
                        .all(|value| unique.insert(value))
                })
                .all(|v| v)
        };

        let box_values_ok = || {
            BOXES
                .iter()
                .map(|r#box| {
                    let mut unique = HashSet::new();

                    r#box
                        .iter()
                        .filter_map(|idx| self.cells[idx].as_option())
                        .all(|value| unique.insert(value))
                })
                .all(|v| v)
        };

        cell_len_ok()
            && rows_len_ok()
            && cols_len_ok()
            && row_values_ok()
            && col_values_ok()
            && box_values_ok()
    }

    fn list_empty_cells(&self) -> Vec<Idx> {
        let mut empty_cells = Vec::new();

        if self.has_empty_cell() {
            for row in 0..9 {
                for col in 0..9 {
                    if self.cells[&(row, col)].is_empty() {
                        empty_cells.push((row, col));
                    }
                }
            }
        }

        empty_cells
    }

    fn find_empty_cell(&self) -> Option<(Idx, Cell)> {
        self.cells
            .iter()
            .filter_map(|(idx, cell)| match &self.cells[idx] {
                Cell::Solved(_) => None,
                Cell::Unsolved(set) => Some((*idx, cell.clone(), set.clone())),
            })
            .min_by(|(_, _, set1), (_, _, set2)| set1.len().cmp(&set2.len()))
            .map(|(idx, cell, _)| (idx, cell))
    }

    fn has_empty_cell(&self) -> bool {
        self.cells.iter().any(|(_, cell)| cell.is_empty())
    }

    fn markup(&mut self) {
        let empty_cells = self.list_empty_cells();

        let full_set = HashSet::from(FULL_SET);

        for idx in empty_cells {
            let peer_markup = [row_of(idx), col_of(idx), box_of(idx)]
                .concat()
                .into_iter()
                .filter_map(|idx| match self.cells[&idx] {
                    Cell::Solved(v) => Some(v),
                    Cell::Unsolved(_) => None,
                })
                .collect::<HashSet<_>>();

            let values = full_set
                .difference(&peer_markup)
                .copied()
                .collect::<HashSet<_>>();

            if let Cell::Unsolved(markup) = self.cells.get_mut(&idx).unwrap() {
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

            for idx in self.list_empty_cells() {
                let empty_cell = self.cells.get_mut(&idx).unwrap();

                if empty_cell.cell_values().len() == 1 {
                    let value = *empty_cell.cell_values().iter().next().unwrap();
                    empty_cell.cell_values_mut().remove(&value);

                    self.cells.insert(idx, Cell::Solved(value));

                    cont = true;
                    modified = true;
                    self.markup();
                }
            }
        }

        modified
    }

    fn find_pre_sets(&mut self, range: &[Idx]) -> Vec<PreSet> {
        let pre_sets = |n: usize| -> Vec<PreSet> {
            fn go(sets: &[HashSet<Idx>], acc: Vec<PreSet>, board: &Board, n: usize) -> Vec<PreSet> {
                if sets.is_empty() {
                    return acc;
                }

                let (x, xs) = sets.split_first().unwrap();

                let nums = x.iter().fold(HashSet::new(), |acc, value| {
                    board.cells[value]
                        .cell_values()
                        .union(&acc)
                        .copied()
                        .collect()
                });

                if nums.len() == n {
                    let mut acc = acc;
                    acc.push(PreSet {
                        numbers: nums,
                        cells: x.clone(),
                    });

                    go(&remove_any(x, xs), acc, board, n)
                } else {
                    go(xs, acc, board, n)
                }
            }

            let combinations = range
                .iter()
                .filter(|&idx| self.cells[idx].is_empty())
                .copied()
                .combinations(n)
                .map(|comb| comb.into_iter().collect())
                .collect::<Vec<_>>();

            go(&combinations, Vec::new(), self, n)
        };

        [pre_sets(2), pre_sets(3), pre_sets(4)].concat()
    }

    fn apply_presets(&mut self) {
        for ranges in [ROWS, COLS, BOXES] {
            for indices in ranges {
                for ps in self.find_pre_sets(&indices) {
                    let other_cells = HashSet::from(indices)
                        .difference(&ps.cells)
                        .copied()
                        .collect::<HashSet<_>>();

                    for cell_idx in other_cells {
                        if let Some(Cell::Unsolved(set)) = self.cells.get_mut(&cell_idx) {
                            *set = set.difference(&ps.numbers).copied().collect();
                        }
                    }
                }
            }
        }
    }

    fn simplify(&mut self) {
        let mut modified_previous = true;
        while self.has_empty_cell() && modified_previous {
            self.markup();
            self.apply_presets();

            modified_previous = self.fill_forced_cells();
        }
    }

    pub fn solve(&mut self) -> bool {
        self.simplify();

        if let Some((idx, cell)) = self.find_empty_cell() {
            for possibility in cell.cell_values() {
                self.cells.insert(idx, Cell::Solved(possibility));
                let prev_cells = self.cells.clone();

                if self.solve() {
                    return true;
                }

                self.cells = prev_cells;
            }

            false
        } else {
            true
        }
    }
}

impl Display for Board {
    fn fmt(&self, buf: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(buf, "{TOP_L_CORNER}")?;
        for _ in 0..8 {
            write!(buf, "{}{}", HORZ_BAR.repeat(3), HORZ_T_JOINT)?;
        }

        writeln!(buf, "{}{}", HORZ_BAR.repeat(3), TOP_R_CORNER)?;

        let mut board = vec![vec!['a'; 9]; 9];

        for (&(r, c), cell) in &self.cells {
            board[r][c] = cell.as_option().unwrap_or('_');
        }

        for (i, row) in board.iter().enumerate() {
            write!(buf, "{VERT_BAR}")?;
            for (j, &data) in row.iter().enumerate() {
                write!(
                    buf,
                    " {} {VERT_BAR}",
                    if self.prefilled.contains(&(j, i)) {
                        data.bright_cyan().to_string()
                    } else {
                        data.bright_red().to_string()
                    }
                )?;
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
            write!(buf, "{}{}", HORZ_BAR.repeat(3), HORZ_B_JOINT)?;
        }

        writeln!(buf, "{}{}", HORZ_BAR.repeat(3), BOT_R_CORNER)?;

        Ok(())
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cells = vec!['\0'; 81];

        for (&(row, col), cell) in &self.cells {
            cells[row * 9 + col] = match cell {
                Cell::Solved(c) => *c,
                Cell::Unsolved(_) => '_',
            }
        }

        write!(f, "{}", cells.iter().collect::<String>())
    }
}
