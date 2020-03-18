use cargo_snippet::snippet;

// Verified with GRA_5_A

#[snippet("tree_diameter")]
#[derive(Clone, Copy)]
struct Edge {
    dst: usize,
    weight: i64,
}
#[snippet("tree_diameter")]
struct FindFurthestPair {
    g: Vec<Vec<Edge>>
}
#[snippet("tree_diameter")]
impl FindFurthestPair {
    fn find(&self, v: usize) -> (usize, i64) {
        self.find_rec(None, v)
    }
    fn find_rec(&self, par: Option<usize>, v: usize) -> (usize, i64) {
        let mut r = (v, 0);
        for &e in &self.g[v] {
            if Some(e.dst) != par {
                let mut t = self.find_rec(Some(v), e.dst);
                t.1 += e.weight;
                if r.1 < t.1 {
                    r = t;
                }
            }
        }
        r
    }
}
#[snippet("tree_diameter")]
struct TreeDiameter {
    g: Vec<Vec<Edge>>,
}
#[snippet("tree_diameter")]
impl TreeDiameter {
    pub fn new(n: usize) -> TreeDiameter {
        TreeDiameter {
            g: vec![vec![];n]
        }
    }
    pub fn connect(&mut self, u: usize, v: usize, dist: i64) {
        self.g[u].push(Edge { dst: v, weight: dist });
        self.g[v].push(Edge { dst: u, weight: dist });
    }
    pub fn solve(&self) -> i64 {
        let ffp = FindFurthestPair {
            g: self.g.clone(),
        };
        let (v, _) = ffp.find(0);
        let (_, d) = ffp.find(v);
        d
    }
}