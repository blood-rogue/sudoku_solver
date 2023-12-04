use std::arch::x86_64::_pdep_u64;

use crate::bitset::{BitSetIter, IndexSet};

pub struct Combinations {
    iter: BitSetIter<usize>,
    indices: Vec<usize>,
    buffer: IndexSet,
    first: bool,
}

impl Combinations {
    pub fn new(iter: BitSetIter<usize>, k: usize) -> Self {
        Self {
            iter,
            indices: (0..k).collect(),
            buffer: IndexSet::new(0),
            first: true,
        }
    }
}

impl Iterator for Combinations {
    type Item = IndexSet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            let buffer_len = self.buffer.len();
            if self.indices.len() > buffer_len {
                let delta = self.indices.len() - buffer_len;
                self.buffer = self.buffer.union(self.iter.by_ref().take(delta).collect());
            }

            if self.indices.len() > self.buffer.len() {
                return None;
            }

            self.first = false;
        } else if self.indices.is_empty() {
            return None;
        } else {
            let mut i: usize = self.indices.len() - 1;
            if self.indices[i] == self.buffer.len() - 1 {
                if let Some(x) = self.iter.next() {
                    self.buffer.insert(x);
                }
            }

            while self.indices[i] == i + self.buffer.len() - self.indices.len() {
                if i > 0 {
                    i -= 1;
                } else {
                    return None;
                }
            }

            self.indices[i] += 1;
            for j in i + 1..self.indices.len() {
                self.indices[j] = self.indices[j - 1] + 1;
            }
        }

        Some(
            self.indices
                .iter()
                .map(|i| index(self.buffer.0, *i))
                .collect(),
        )
    }
}

pub fn index(num: u128, n: usize) -> usize {
    let [high, low] = [(num >> 64) as u64, num as u64];
    let set_in_low = low.count_ones() as usize;

    let pos = if n < set_in_low {
        unsafe { _pdep_u64(1 << n, low) }.trailing_zeros()
    } else {
        unsafe { _pdep_u64(1 << (n - set_in_low), high) }.trailing_zeros() + 64
    };

    pos as usize
}
