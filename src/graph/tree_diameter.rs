use cargo_snippet::snippet;

/// 木の直径
/// 
/// 重み付き木（無向）において、直径を求める。
/// アルゴリズムにはdouble-sweepという名前があるらしい。

#[snippet("tree_diameter")]
pub mod tree_diameter {
    #[derive(Clone, Copy)]
    struct Edge {
        dst: usize,
        weight: i64,
    }
    pub struct FindFurthestPair {
        g: Vec<Vec<Edge>>
    }
    impl FindFurthestPair {
        pub fn find(&self, v: usize) -> (usize, i64) {
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
    pub struct TreeDiameter {
        g: Vec<Vec<Edge>>,
    }
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
        pub fn solve(&self) -> (usize, usize, i64) {
            let ffp = FindFurthestPair {
                g: self.g.clone(),
            };
            let (v, _) = ffp.find(0);
            let (w, d) = ffp.find(v);
            (v, w, d)
        }
    }
}