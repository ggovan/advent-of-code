#[derive(PartialEq, Eq, Copy, Clone, std::fmt::Debug, Hash)]
pub struct Bitset(u64);

impl Bitset {
    pub fn empty() -> Self {
        Bitset(0)
    }

    pub fn set(&self, position: usize) -> Self {
        Self(self.0 | (1 << position))
    }

    pub fn clear(&self, position: usize) -> Self {
        Self(self.0 & (!0 ^ (1 << position)))
    }

    pub fn count(&self) -> u64 {
        self.0.count_ones() as u64
    }

    pub fn contains(&self, position: usize) -> bool {
        (self.0 & (1 << position)) != 0
    }

    pub fn contains_all(&self, Bitset(other): Bitset) -> bool {
        (self.0 & other) == other
    }
}
