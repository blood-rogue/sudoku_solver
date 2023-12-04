use crate::bitset::{DigitSet, IndexSet};
use crate::combination::Combinations;
use crate::unroll;
use crate::utils::{ranges_of, FULL_SET, RANGES, UNSOLVED_CELL};

pub type Index = usize;
pub type Digit = u8;

#[derive(Clone, Copy)]
pub enum Cell {
    Unsolved(DigitSet),
    Solved(Digit),
}

impl Cell {
    const fn markup(self) -> DigitSet {
        match self {
            Self::Unsolved(set) => set,
            Self::Solved(_) => DigitSet::new(0),
        }
    }

    const fn value(self) -> Digit {
        match self {
            Self::Solved(v) => v,
            Self::Unsolved(_) => 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Puzzle {
    pub cells: [Cell; 81],
    pub empty_cells: IndexSet,
}

impl Puzzle {
    #[allow(clippy::cognitive_complexity)]
    pub fn new_from_string(s: &[Digit]) -> Self {
        let mut cells = [UNSOLVED_CELL; 81];
        let mut empty_cells = IndexSet::new(0);

        unroll! {
            for I in 0..81 {
                if FULL_SET >> (s[I] - b'0') & 1 == 1 {
                    cells[I] = Cell::Solved(s[I] - b'0');
                } else {
                    empty_cells.insert(I);
                }
            }
        }

        Self { cells, empty_cells }
    }

    pub fn is_valid(&self) -> bool {
        RANGES
            .iter()
            .map(|range| {
                range
                    .difference(self.empty_cells)
                    .into_iter()
                    .map(|idx| self.cells[idx].value())
                    .collect::<DigitSet>()
                    .0
                    == FULL_SET
            })
            .all(|v| v)
    }

    fn find_empty_cell(&self) -> Option<(Index, DigitSet)> {
        self.empty_cells
            .into_iter()
            .map(|idx| (idx, self.cells[idx].markup()))
            .min_by(|(_, set1), (_, set2)| set1.len().cmp(&set2.len()))
    }

    fn cross_out(&mut self, indices: IndexSet, value: Digit) {
        for idx in indices {
            if let Cell::Unsolved(ref mut set) = self.cells[idx] {
                set.0 &= !(1 << value);
            }
        }
    }

    fn generate_markup(&mut self) {
        for idx in self.empty_cells {
            let peer_markup = ranges_of(idx)
                .difference(self.empty_cells)
                .into_iter()
                .map(|idx| self.cells[idx].value())
                .collect::<DigitSet>();

            if let Cell::Unsolved(ref mut markup) = self.cells[idx] {
                markup.difference_mut(peer_markup);
            }
        }

        self.fill_singles();
    }

    fn fill_singles(&mut self) -> bool {
        let mut cont = true;
        let mut modified = false;

        while cont {
            cont = false;

            for idx in self.empty_cells {
                let empty_cell_values = self.cells[idx].markup();

                if empty_cell_values.len() == 1 {
                    let value = empty_cell_values.pop();
                    self.cells[idx] = Cell::Solved(value);
                    self.empty_cells.remove(idx);

                    cont = true;
                    modified = true;

                    self.cross_out(ranges_of(idx), value);
                }
            }
        }

        modified
    }

    fn find_pre_sets(&mut self, range: IndexSet) -> Vec<LockedSet> {
        let pre_sets = |n: Index| -> Vec<LockedSet> {
            fn go(
                sets: &[IndexSet],
                mut acc: Vec<LockedSet>,
                puzzle: &Puzzle,
                n: Index,
            ) -> Vec<LockedSet> {
                let [x, xs @ ..] = sets else {
                    return acc;
                };

                let nums = x.into_iter().fold(DigitSet::new(0), |acc, idx| {
                    puzzle.cells[idx].markup().union(acc)
                });

                if nums.len() == n {
                    acc.push(LockedSet {
                        numbers: nums,
                        cells: *x,
                    });

                    go(&remove_any(x, xs), acc, puzzle, n)
                } else {
                    go(xs, acc, puzzle, n)
                }
            }

            let combinations =
                Combinations::new(range.intersection(self.empty_cells).into_iter(), n)
                    .collect::<Vec<_>>();

            go(&combinations, Vec::new(), self, n)
        };

        [pre_sets(2), pre_sets(3)].concat()
    }

    fn apply_locked_sets(&mut self) {
        for indices in RANGES {
            for ps in self.find_pre_sets(indices) {
                for cell_idx in indices.difference(ps.cells) {
                    if let Cell::Unsolved(ref mut set) = self.cells[cell_idx] {
                        set.difference_mut(ps.numbers);
                    }
                }
            }
        }
    }

    fn simplify(&mut self) {
        let mut modified_previous = true;
        self.generate_markup();

        while modified_previous {
            self.apply_locked_sets();

            modified_previous = self.fill_singles();
        }
    }

    pub fn solve(&mut self) -> bool {
        self.simplify();

        if let Some((idx, possibilities)) = self.find_empty_cell() {
            self.empty_cells.remove(idx);

            for possibility in possibilities {
                self.cells[idx] = Cell::Solved(possibility);
                let prev_state = *self;

                if self.solve() {
                    return true;
                }

                *self = prev_state;
            }

            false
        } else {
            true
        }
    }
}

#[derive(Clone, Copy)]
struct LockedSet {
    numbers: DigitSet,
    cells: IndexSet,
}

#[inline]
fn remove_any(x: &IndexSet, xs: &[IndexSet]) -> Vec<IndexSet> {
    xs.iter()
        .filter(|y| y.intersection(*x).0 == 0)
        .copied()
        .collect()
}
