#[snippet = "UnionFind"]
pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize> }

#[snippet = "UnionFind"]
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).collect::<Vec<usize>>(),
            rank: vec![1; n],
        }
    }

    pub fn size(&mut self, x: usize) -> usize {
        let y = self.root(x);
        self.rank[y]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        } else {
            let y = self.par[x];
            let z = self.root(y);
            self.par[x] = z;
            return z;
        }
    }

    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut a = self.root(x);
        let mut b = self.root(y);
        if a == b { return false; }

        if self.rank[a] < self.rank[b] {
            std::mem::swap(&mut a, &mut b);
        }
        assert!(self.rank[a] >= self.rank[b]);

        self.rank[a] += self.rank[b];
        self.par[b] = a;
        return true
    }
}

#[test]
fn test_union_find() {
    let mut s = UnionFind::new(5);
    s.merge(1,4);
    s.merge(2,3);
    assert_eq!(s.same(1,2), false);
    assert_eq!(s.same(3,4), false);
    assert_eq!(s.same(1,4), true);
    assert_eq!(s.same(3,2), true);
    assert_eq!(s.size(1), 2);
    assert_eq!(s.size(0), 1);

    s.merge(1,3);
    assert_eq!(s.same(2,4), true);
    assert_eq!(s.same(3,0), false);
    assert_eq!(s.size(0), 1);
    assert_eq!(s.size(1), 4);
    assert_eq!(s.size(2), 4);

    s.merge(0,4);
    assert_eq!(s.same(0,2), true);
    assert_eq!(s.same(3,0), true);
    assert_eq!(s.size(0), 5);
}

#[snippet = "WeighedUnionFind"]
struct WeightedUnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
    diff_weight: Vec<i32>,
}

#[snippet = "WeighedUnionFind"]
impl WeightedUnionFind {
    fn new(n: usize) -> WeightedUnionFind {
        WeightedUnionFind {
            par: (0..n).collect::<Vec<usize>>(),
            rank: vec![0; n],
            diff_weight: vec![0; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
             return x;
        } else {
            let y = self.par[x];
            let z = self.root(y);
            self.diff_weight[x] += self.diff_weight[y];
            self.par[x] = z;
            return z;
        }
    }

    fn weight(&mut self, x: usize) -> i32 {
        self.root(x);
        self.diff_weight[x]
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn merge(&mut self, x: usize, y: usize, w: i32) -> bool {
        let mut w = w;
        w += self.weight(x);
        w -= self.weight(y);

        let mut a = self.root(x);
        let mut b = self.root(y);

        if a == b {
            return false;
        }

        if self.rank[a] < self.rank[b] {
            std::mem::swap(&mut a, &mut b);
            w = -w;
        }
        assert!(self.rank[a] >= self.rank[b]);

        if self.rank[a] == self.rank[b] {
            self.rank[a] += 1;
        }

        self.par[b] = a;
        self.diff_weight[b] = w;

        return true
    }
}

#[test]
fn test_weighted_union_find() {
    let mut wuf = WeightedUnionFind::new(5);
    wuf.merge(0,2,5);
    wuf.merge(1,2,3);
    assert_eq!(wuf.weight(1) - wuf.weight(0), 2);
    assert!(wuf.root(3) != wuf.root(1));
    wuf.merge(1,4,8);
    assert_eq!(wuf.weight(4) - wuf.weight(0), 10);
}