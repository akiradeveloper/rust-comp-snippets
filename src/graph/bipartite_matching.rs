use cargo_snippet::snippet;
use std::collections::HashSet;

/// 二部グラフというのは、
/// 頂点集合を２つに分割して、各集合の頂点はお互いに隣接しないようなグラフである。

/// 隣合うノードに白黒と色をつけていくことで二部グラフかどうかを判定する。

#[snippet("BipartiteMatching")]
pub fn isbipartite(g: &[HashSet<usize>]) -> bool {
    struct Rec<'a> {
        g: &'a [HashSet<usize>],
        color: Vec<i8>,
    }
    impl <'a> Rec<'a> {
        // uにcolorをつけることが出来るか
        fn solve(&mut self, u: usize, color: i8) -> bool {
            self.color[u] = color;

            let mut ok = true;
            // 隣り合うノードすべてに異なる色をつけられることを検証する
            for &v in &self.g[u] {
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
    rec.solve(0, 1)
}

/// 二部グラフの中から、最大マッチングを列挙する。
/// 
/// アイデア:
/// 増加路を探しまくる。
/// 増加路が存在しない <=> 最大マッチング
/// が言える。
/// 
/// 計算量:
/// O(V(V+E))

#[snippet("BipartiteMatching")]
fn find_max_bipartite_matching(g_list: &[HashSet<usize>]) -> Vec<(usize,usize)> {
    fn dfs(v: usize, g_list: &[HashSet<usize>], used: &mut [bool], matching: &mut [Option<usize>]) -> bool {
        used[v] = true;
        for &u in &g_list[v] {
            let w = matching[u];
            // 今からつなごうとしているuにマッチングwがすでにいる場合、
            // wに他のペアを探すようにお願いする。これが増加路を探すことに相当する。
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

#[snippet("BipartiteMatching")]
struct BipartiteMatching {
    g: Vec<HashSet<usize>>,
}
#[snippet("BipartiteMatching")]
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
    pub fn solve(&self) -> Option<Vec<(usize, usize)>> {
        if isbipartite(&self.g) {
            Some(find_max_bipartite_matching(&self.g))
        } else {
            None
        }
    }
}

#[test]
fn test_bipartite_matching() {
    let mut bpm = BipartiteMatching::new(6);
    bpm.connect(0,3);
    bpm.connect(0,4);
    bpm.connect(1,4);
    bpm.connect(2,5);
    assert!(bpm.solve().is_some());
    dbg!(bpm.solve());
}

#[test]
fn test_bipartite_matching_2() {
    let mut bpm = BipartiteMatching::new(3);
    bpm.connect(0, 1);
    bpm.connect(1, 2);
    bpm.connect(2, 0);
    assert!(bpm.solve().is_none());
}