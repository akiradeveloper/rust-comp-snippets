#[snippet = "Counter"]
struct Counter<T> {
    h: std::collections::HashMap<T, usize>,
}
#[snippet = "Counter"]
impl <T: Eq + std::hash::Hash> Counter<T> {
    pub fn new() -> Counter<T> {
        Counter {
            h: std::collections::HashMap::new(),
        }
    }
    pub fn add(&mut self, x: T) {
        *self.h.entry(x).or_insert(0) += 1;
    }
    pub fn get(&self, x: &T) -> usize {
        self.h.get(x).cloned().unwrap_or(0)
    }
    pub fn entries(&self) -> Vec<(&T, usize)> {
        let mut r = vec![];
        for (x,y) in &self.h {
            r.push((x,*y));
        }
        r
    }
}

#[test]
fn test_counter() {
    let mut c = Counter::new();
    let mut v = vec![1,2,1,3,1,3];
    for x in v {
        c.add(x);
    }
    assert_eq!(c.get(&4), 0);
    assert_eq!(c.get(&1), 3);
    dbg!(c.entries());
}