use cargo_snippet::snippet;

#[snippet("Rename")]
struct Rename {
    F: Vec<usize>,
    R: Vec<usize>,
    n: usize,
    cur: usize,
}
#[snippet("Rename")]
impl Rename {
    pub fn new(n: usize) -> Self {
        NodeRename {
            F: vec![n;n],
            R: vec![n;n],
            n: n,
            cur: 0,
        }
    }
    pub fn insert(&mut self, u: usize) {
        if self.F[u] == self.n {
            let i = self.cur;
            self.F[u] = i;
            self.R[i] = u;
            self.cur += 1;
        }
    }
    pub fn get(&mut self, u: usize) -> usize {
        self.F[u]
    }
    pub fn rev(&mut self, i: usize) -> usize {
        self.R[i]
    }
    pub fn len(&self) -> usize {
        self.cur
    }
}

#[test]
fn test_rename() {
    let mut rn = Rename::new(3);
    rn.insert(1);
    assert_eq!(rn.get(1),0);
    assert_eq!(rn.rev(0),1);
    rn.insert(1);
    assert_eq!(rn.get(1),0);
    assert_eq!(rn.rev(0),1);
}