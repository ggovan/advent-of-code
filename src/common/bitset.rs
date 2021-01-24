#[derive(PartialEq, Eq, Copy, Clone, std::fmt::Debug, Hash)]
pub struct Bitset(u64);

#[allow(unused)]
impl Bitset {
    pub fn empty() -> Self {
        Bitset(0)
    }

    pub fn from(raw: u64) -> Self {
        Bitset(raw)
    }

    #[must_use]
    pub fn set(&self, position: usize) -> Self {
        Self(self.0 | (1 << position))
    }

    #[must_use]
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

    #[must_use]
    pub fn flip_start(&self, len: usize) -> Self {
        let mut new = Bitset::empty();
        for i in 0..len {
            if self.contains(i) {
                new = new.set(len - i - 1);
            }
        }
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_start() {
        assert_eq!(0b0, Bitset::from(0).flip_start(1).0);
        assert_eq!(0b1, Bitset::from(0b1).flip_start(1).0);
        assert_eq!(0b01, Bitset::from(0b10).flip_start(2).0);
        assert_eq!(0b01, Bitset::from(0b10).flip_start(2).0);
    }
}
