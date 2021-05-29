use std::collections::HashMap;

struct Bag<T> {
    h: HashMap<T, usize>,
}
impl <T: Eq + std::hash::Hash> Bag<T> {
    pub fn new() -> Self {
        Self {
            h: HashMap::new(),
        }
    }
    pub fn add(&mut self, x: T) {
        *self.h.entry(x).or_insert(0) += 1;
    }
    pub fn remove(&mut self, x: &T) -> bool {
        let y = *self.h.get(&x).unwrap_or(&0);
        if y == 0 {
            false
        } else {
            if y == 1 {
                self.h.remove_entry(&x);
            } else {
                *self.h.get_mut(&x).unwrap() -= 1;
            }
            true
        }
    }
    pub fn count(&self, x: &T) -> usize {
        match self.h.get(&x) {
            Some(y) => *y,
            None => 0,
        }
    }
    pub fn uniq_count(&self) -> usize {
        self.h.len()
    }
}
#[test]
fn test_bag() {
    let mut h = Bag::new();
    assert_eq!(h.uniq_count(), 0);
    h.add(1);
    assert_eq!(h.uniq_count(), 1);
    h.remove(&1);
    assert_eq!(h.uniq_count(), 0);

    h.add(1);
    h.add(1);
    assert_eq!(h.uniq_count(), 1);
    h.add(2);
    assert_eq!(h.uniq_count(), 2);
    h.remove(&1);
    assert_eq!(h.uniq_count(), 2);
    h.remove(&1);
    assert_eq!(h.uniq_count(), 1);
    h.remove(&2);
    assert_eq!(h.uniq_count(), 0);
}