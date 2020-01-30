use std::collections::HashSet;

#[doc = "undirected. paint the vertices in two colors. if impossible return None."]
#[snippet = "is_bigraph"]
pub fn is_bigraph(g: &[Vec<usize>]) -> Option<Vec<bool>> {
    struct Rec<'a> {
        g: &'a [Vec<usize>],
        color: Vec<i8>,
    }
    impl <'a> Rec<'a> {
        fn solve(&mut self, u: usize, color: i8) -> bool {
            self.color[u] = color;

            let mut ok = true;
            for i in 0..self.g[u].len() {
                let v = self.g[u][i];
                if self.color[v] == 0 {
                    if !self.solve(v, -1*color) {
                        ok = false
                    }
                } else {
                    if self.color[v] == color {
                        ok = false
                    }
                }
            }
            ok
        }
    }

    let n = g.len();
    let mut rec = Rec {
        g: g,
        color: vec![0;n],
    };
    let ok = rec.solve(0, 1);
    if !ok {
        return None
    }
    let mut res = vec![];
    for i in 0..n {
        if rec.color[i] == 1 {
            res.push(true)
        } else {
            res.push(false)
        }
    }
    Some(res)
}


#[snippet = "bipartite_matching"]
#[doc = "O(V(V+E))"]
fn bipartite_matching(g_list: &[HashSet<usize>]) -> Vec<(usize,usize)> {
    fn dfs(v: usize, g_list: &[HashSet<usize>], used: &mut [bool], matching: &mut [Option<usize>]) -> bool {
        used[v] = true;
        for &u in &g_list[v] {
            let w = matching[u];
            if w.is_none() || (!used[w.unwrap()] && dfs(w.unwrap(), g_list, used, matching)) {
                matching[v] = Some(u);
                matching[u] = Some(v);
                return true
            }
        }
        false
    }
    let n = g_list.len();
    let mut matching = vec![None; n];
    for v in 0..n {
        if matching[v].is_none() {
            let mut used = vec![false; n];
            dfs(v, g_list, &mut used, &mut matching);
        }
    }
    let mut res = vec![];
    for u in 0..matching.len() {
        let v0 = matching[u];
        if v0.is_some() {
            let v = v0.unwrap();
            assert!(u != v);
            if u < v {
                res.push((u,v));
            }
        }
    }
    res
}

#[snippet = "bipartite_matching"]
struct BipartiteMatching {
    g: Vec<HashSet<usize>>,
}
#[doc = "find the pair of vertices which is maximum possible."]
#[snippet = "bipartite_matching"]
impl BipartiteMatching {
    pub fn new(n: usize) -> BipartiteMatching {
        BipartiteMatching {
            g: vec![HashSet::new(); n],
        }
    }
    pub fn connect(&mut self, u: usize, v: usize) {
        assert!(u != v);
        self.g[u].insert(v);
        self.g[v].insert(u);
    }
    pub fn solve(&self) -> Vec<(usize, usize)> {
        bipartite_matching(&self.g)
    }
}

#[test]
fn test_bipartite_matching() {
    let mut bpm = BipartiteMatching::new(4);
    bpm.connect(0,2);
    bpm.connect(0,3);
    bpm.connect(1,2);
    dbg!(bpm.solve());
}
#[test]
fn test_bipartite_matching_impossible() {
    let mut bpm = BipartiteMatching::new(3);
    bpm.connect(0,1);
    bpm.connect(1,2);
    bpm.connect(2,0);
    dbg!(bpm.solve());
}