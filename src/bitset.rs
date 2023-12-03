use std::usize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BitSet<T: SetElement>(T::Storage);

pub type DigitSet = BitSet<u8>;
pub type IndexSet = BitSet<usize>;

pub trait SetElement {
    type Storage;
}

pub struct BitSetIter<T: SetElement>(T::Storage);

impl SetElement for u8 {
    type Storage = u16;
}

impl BitSet<u8> {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn insert(&mut self, value: u8) {
        self.0 |= 1 << value;
    }

    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    pub const fn contains(self, value: u8) -> bool {
        self.0 >> value & 1 == 1
    }

    pub fn remove(&mut self, value: u8) {
        self.0 &= !(1 << value);
    }

    pub fn pop(self) -> u8 {
        self.into_iter().rightmost_one_pos()
    }

    pub fn difference_mut(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

impl IntoIterator for BitSet<u8>
where
    BitSetIter<u8>: Iterator,
{
    type Item = <BitSetIter<u8> as Iterator>::Item;
    type IntoIter = BitSetIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        BitSetIter(self.0)
    }
}

impl FromIterator<u8> for BitSet<u8> {
    fn from_iter<U: IntoIterator<Item = u8>>(iter: U) -> Self {
        let mut s = Self::new(0);
        for a in iter {
            s.insert(a);
        }

        s
    }
}

impl BitSetIter<u8> {
    const fn rightmost_one_pos(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    const fn leftmost_one_pos(&self) -> u8 {
        (u16::BITS - 1 - self.0.leading_zeros()) as u8
    }

    const fn count_ones(&self) -> usize {
        self.0.count_ones() as usize
    }

    fn clear_rightmost_one(&mut self) {
        self.0 &= self.0.wrapping_sub(1);
    }
}

impl Iterator for BitSetIter<u8> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 != 0 {
            let trailing = self.rightmost_one_pos();
            self.clear_rightmost_one();
            Some(trailing)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = self.count_ones();
        (sz, Some(sz))
    }

    fn count(self) -> usize {
        self.0.count_ones() as usize
    }

    fn last(self) -> Option<Self::Item> {
        if self.0 != 0 {
            Some(self.leftmost_one_pos())
        } else {
            None
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let mut i = 0;
        while self.0 != 0 && i < n {
            self.clear_rightmost_one();
            i += 1;
        }
        self.next()
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;
        while self.0 != 0 {
            accum = f(accum, self.rightmost_one_pos());
            self.clear_rightmost_one();
        }
        accum
    }

    fn max(self) -> Option<Self::Item> {
        self.last()
    }

    fn min(self) -> Option<Self::Item> {
        if self.0 != 0 {
            Some(self.rightmost_one_pos())
        } else {
            None
        }
    }
}

impl SetElement for usize {
    type Storage = u128;
}

impl BitSet<usize> {
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    pub fn insert(&mut self, value: usize) {
        self.0 |= 1 << value;
    }

    pub fn remove(&mut self, value: usize) {
        self.0 &= !(1 << value);
    }

    pub const fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl IntoIterator for BitSet<usize>
where
    BitSetIter<usize>: Iterator,
{
    type Item = <BitSetIter<usize> as Iterator>::Item;
    type IntoIter = BitSetIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        BitSetIter(self.0)
    }
}

impl FromIterator<usize> for BitSet<usize> {
    fn from_iter<U: IntoIterator<Item = usize>>(iter: U) -> Self {
        let mut s = Self::new(0);
        for a in iter {
            s.insert(a);
        }
        s
    }
}

impl BitSetIter<usize> {
    const fn rightmost_one_pos(&self) -> usize {
        self.0.trailing_zeros() as usize
    }

    const fn leftmost_one_pos(&self) -> usize {
        (u128::BITS - 1 - self.0.leading_zeros()) as usize
    }

    const fn count_ones(&self) -> usize {
        self.0.count_ones() as usize
    }

    fn clear_rightmost_one(&mut self) {
        self.0 &= self.0.wrapping_sub(1);
    }
}

impl Iterator for BitSetIter<usize> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 != 0 {
            let trailing = self.rightmost_one_pos();
            self.clear_rightmost_one();
            Some(trailing)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = self.count_ones();
        (sz, Some(sz))
    }

    fn count(self) -> usize {
        self.0.count_ones() as usize
    }

    fn last(self) -> Option<Self::Item> {
        if self.0 != 0 {
            Some(self.leftmost_one_pos())
        } else {
            None
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let mut i = 0;
        while self.0 != 0 && i < n {
            self.clear_rightmost_one();
            i += 1;
        }
        self.next()
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;
        while self.0 != 0 {
            accum = f(accum, self.rightmost_one_pos());
            self.clear_rightmost_one();
        }
        accum
    }

    fn max(self) -> Option<Self::Item> {
        self.last()
    }

    fn min(self) -> Option<Self::Item> {
        if self.0 != 0 {
            Some(self.rightmost_one_pos())
        } else {
            None
        }
    }
}
