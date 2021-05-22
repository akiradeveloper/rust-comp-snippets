use cargo_snippet::snippet;

/// 無向グラフにおいて関節点と橋を求める。
/// 
/// 関節点: その点を取り除くと連結成分が増える
/// 橋: その辺を取り除くと連結成分が増える
/// 
/// アイデア:
/// あるノードからDFSを行い、到達順ord[u]を記録する。
/// この時のたどり順をDFS-Treeと呼び、通らなかった辺を後退辺と呼ぶことにする。
/// lowlinkという値を定義する。
/// lowlink[u]は、DFS-Treeに沿って後退辺をたかだか1本通った時に到達可能な頂点集合のordのうち最小値である。
/// lowlinkはこんな性質がある。もし、後退辺が一本もないような木の場合、ord[u]=lowlink[u]となる。
/// 仮にDFS-Treeにおいて、uの子vについてord[u]<=low[v]が成り立ったとする。
/// この時、uを取り除いてしまうと、vはuの祖先から孤立してしまうことがわかる。 
/// なぜならば、vからuの祖先に行く方法がないからである。
/// つまり、uが関節点であることが言える。
/// 
/// 橋については、
/// まず後退辺については橋には成り得ない。
/// DFS-Treeのうちu->v辺についてord[u]<low[v]の時、u->vは橋となる。
/// なぜならば、u->vを切ってしまうとvからuにすら到達出来なくなってしまうため。
/// 
/// 計算量:
/// 構築 O(V+E)

#[snippet("Lowlink")]
struct LowLink {
    g: Vec<Vec<usize>>,
    used: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    pub articulation: Vec<usize>,
    pub bridge: Vec<(usize, usize)>,
}
#[snippet("Lowlink")]
#[doc = "find articulation points and bridges at the same time"]
impl LowLink {
    fn minmax(p: (usize, usize)) -> (usize, usize) {
        if p.0 <= p.1 {
            p
        } else {
            (p.1, p.0)
        }
    }
    pub fn new(n: usize) -> LowLink {
        let mut g = vec![vec![];n];
        let mut used = vec![false; n];
        let mut ord = vec![0; n];
        let mut low = vec![0; n];
        let articulation = vec![];
        let bridge = vec![];
        LowLink {
            g,
            used,
            ord,
            low,
            articulation,
            bridge,
        }
    }
    pub fn connect(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
    }
    pub fn build(&mut self) {
        self.do_build(0, 0, None);
        self.articulation.sort();
        self.bridge.sort();
    }
    fn do_build(&mut self, u: usize, k: usize, par: Option<usize>) -> usize {
        let mut k = k;
        self.used[u] = true;
        self.ord[u] = k;
        k += 1;
        self.low[u] = self.ord[u];
        let mut is_articulation = false;
        let mut cnt = 0;
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            if !self.used[v] {
               cnt += 1; 
               k = self.do_build(v, k, Some(u));
               self.low[u] = std::cmp::min(self.low[u], self.low[v]);
               is_articulation |= par.is_some() && self.low[v] >= self.ord[u];
               if self.ord[u] < self.low[v] {
                   self.bridge.push(Self::minmax((u, v)));
               }
            } else if Some(v) != par {
                self.low[u] = std::cmp::min(self.low[u], self.ord[v]);
            } else {}
        }
        is_articulation |= par.is_none() && cnt > 1;
        if is_articulation {
            self.articulation.push(u);
        }
        k
    }
}

#[test]
fn test_lowlink() {
    let e = vec![(0,1),(0,2),(1,2),(1,3),(2,3),(3,6),(4,6),(5,6),(5,7),(6,7)];
    let mut g = LowLink::new(8);
    for (u,v) in e {
        g.connect(u, v);
    }
    g.build();
    assert_eq!(g.articulation, [3,6]);
    assert_eq!(g.bridge, [(3,6),(4,6)]);
}