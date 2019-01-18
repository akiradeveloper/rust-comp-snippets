pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>
}

impl DisjointSet {
    pub fn new(n: usize) -> DisjointSet {
        let mut x = DisjointSet {
            parent: vec![0; n],
            rank: vec![0; n]
        };
        for i in 0 .. n {
            x.parent[i] = i;
            x.rank[i] = 0;
        }
        x
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find_set(x) == self.find_set(y)
    }
    
    pub fn find_set(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find_set(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let a = self.find_set(x);
        let b = self.find_set(y);
        self.link_set(a, b);
    }

    fn link_set(&mut self, a: usize, b: usize) {
        if self.rank[a] > self.rank[b] {
            self.parent[b] = a;
        } else {
            self.parent[a] = b;
            if self.rank[a] == self.rank[b] {
                self.rank[b] += 1;
            }
        }
    }
}

#[test]
fn test_disjoint_set() {
    let mut s = DisjointSet::new(5);
    s.unite(1,4);
    s.unite(2,3);
    assert_eq!(s.same(1,2), false);
}