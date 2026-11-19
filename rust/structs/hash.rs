pub fn HashString(s: &str, size: usize) -> usize {
    let mut h: usize = 0;
    for b in s.bytes() {
        h = h.wrapping_mul(31).wrapping_add(b as usize);
    }
    h % size
}

pub fn HashNumber(m: i64, size: usize) -> usize {
    let m = if m < 0 { -m } else { m };
    (m as usize) % size
}

pub trait Hashable {
    fn hash(&self, cap: usize) -> usize;
}

impl Hashable for String {
    fn hash(&self, cap: usize) -> usize {
        HashString(self, cap)
    }
}

impl Hashable for i64 {
    fn hash(&self, cap: usize) -> usize {
        HashNumber(*self, cap)
    }
}