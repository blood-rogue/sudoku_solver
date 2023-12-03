use crate::bitset::{DigitSet, IndexSet};
use crate::utils::{box_of, col_of, row_of, BOXES, COLS, FULL_SET, ROWS};

use crunchy::unroll;

use itertools::Itertools;

pub type Index = usize;
pub type Digit = u8;

#[derive(Clone, Copy)]
pub enum Cell {
    Unsolved(DigitSet),
    Solved(Digit),
}

impl Cell {
    const fn is_empty(self) -> bool {
        matches!(self, Self::Unsolved(_))
    }

    const fn cell_values(self) -> DigitSet {
        match self {
            Self::Unsolved(set) => set,
            Self::Solved(_) => DigitSet::new(0),
        }
    }
}

#[derive(Clone, Copy)]
struct PreSet {
    numbers: DigitSet,
    cells: IndexSet,
}

pub struct Puzzle(pub [Cell; 81]);

#[inline]
fn remove_any(x: &IndexSet, xs: &[IndexSet]) -> Vec<IndexSet> {
    xs.iter()
        .filter(|y| y.intersection(*x).len() == 0)
        .copied()
        .collect()
}

impl Puzzle {
    #[allow(clippy::cognitive_complexity)]
    pub const fn new_from_string(s: &[Digit]) -> Self {
        let mut puzzle = [Cell::Unsolved(FULL_SET); 81];

        unroll! {
            for i in 0..81 {
                let ch = s[i] - b'0';
                if FULL_SET.contains(ch) {
                    puzzle[i] = Cell::Solved(ch);
                }
            }
        }

        Self(puzzle)
    }

    pub fn is_valid(&self) -> bool {
        [ROWS, COLS, BOXES]
            .concat()
            .iter()
            .map(|range| {
                let mut unique = DigitSet::new(0);

                range
                    .iter()
                    .filter_map(|&idx| match self.0[idx] {
                        Cell::Solved(v) => Some(v),
                        Cell::Unsolved(_) => None,
                    })
                    .all(|value| unique.insert(value))
            })
            .all(|v| v)
    }

    fn find_empty_cell(&self) -> Option<(Index, DigitSet)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| match cell {
                Cell::Solved(_) => None,
                Cell::Unsolved(set) => Some((i, *set)),
            })
            .min_by(|(_, set1), (_, set2)| set1.len().cmp(&set2.len()))
    }

    fn cross_out(&mut self, indices: Vec<Index>, value: Digit) {
        for idx in indices {
            match self.0.get_mut(idx).unwrap() {
                Cell::Solved(_) => {}
                Cell::Unsolved(set) => {
                    set.remove(value);
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
                .collect::<DigitSet>();

            let values = self.0[idx].cell_values().difference(peer_markup);

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
                    let value = empty_cell_values.pop();
                    self.0[idx] = Cell::Solved(value);

                    cont = true;
                    modified = true;

                    self.cross_out([row_of(idx), col_of(idx), box_of(idx)].concat(), value);
                }
            }
        }

        modified
    }

    fn find_pre_sets(&mut self, range: &[Index]) -> Vec<PreSet> {
        let pre_sets = |n: Index| -> Vec<PreSet> {
            fn go(sets: &[IndexSet], acc: Vec<PreSet>, puzzle: &Puzzle, n: Index) -> Vec<PreSet> {
                let [x, xs @ ..] = sets else {
                    return acc;
                };

                let nums = x.into_iter().fold(DigitSet::new(0), |acc, value| {
                    puzzle.0[value].cell_values().union(acc)
                });

                if nums.len() == n {
                    let mut acc = acc;
                    acc.push(PreSet {
                        numbers: nums,
                        cells: *x,
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
                .map(IndexSet::from_iter)
                .collect_vec();

            go(&combinations, Vec::new(), self, n)
        };

        [pre_sets(2), pre_sets(3), pre_sets(4)].concat()
    }

    fn apply_presets(&mut self) {
        for indices in [ROWS, COLS, BOXES].concat() {
            for ps in self.find_pre_sets(&indices) {
                let indices = IndexSet::from_iter(indices);

                for cell_idx in indices.difference(ps.cells) {
                    if let Some(Cell::Unsolved(set)) = self.0.get_mut(cell_idx) {
                        *set = set.difference(ps.numbers);
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
                let prev_cells = self.0;

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
