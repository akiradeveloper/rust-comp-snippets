// Verified with GRA_5_A

#[derive(Clone, Copy)]
#[snippet = "tree_diameter"]
pub struct Edge {
    dst: usize,
    weight: i64,
}
#[snippet = "tree_diameter"]
pub struct FindFurthestPair {
    g: Vec<Vec<Edge>>
}
#[snippet = "tree_diameter"]
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
#[snippet = "tree_diameter"]
#[doc = "undirected"]
pub fn tree_diameter(tree: Vec<Vec<Edge>>) -> i64 {
    let ffp = FindFurthestPair {
        g: tree,
    };
    let (v, _) = ffp.find(0);
    let (_, d) = ffp.find(v);
    d
}