use cargo_snippet::snippet;
use std::collections::VecDeque;

/// HL分解は、木構造をパスの集合に分解する。
/// こうして木構造をvid木上で一直線状にすることにより、
/// セグツリーなどの配列構造に対するアルゴリズムを適用可能になる。
/// そして、頂点や辺にvidをつける。ここで辺は子ノードの頂点vidで表される。
/// クエリ(u,v)に対して、木上のパスに含まれるパス集合（vidベース）を列挙する。
/// 
/// 構築 O(N)

#[snippet("HLDecomposition")]
struct HLDecomposition {
    n: usize,
    g: Vec<Vec<usize>>,
    subcnt: Vec<usize>,
    depth: Vec<usize>,
    pub par: Vec<Option<usize>>,
    heavy_next: Vec<Option<usize>>,
    heavy_head: Vec<usize>,
    real_to_virt: Vec<usize>,
    pub virt_to_real: Vec<usize>,
}

#[snippet("HLDecomposition")]
impl HLDecomposition {
    pub fn new(n: usize) -> Self {
        HLDecomposition {
            n: n,
            g: vec![vec![]; n],
            subcnt: vec![0; n],
            depth: vec![0; n],
            par: vec![None; n],
            heavy_next: vec![None; n],
            heavy_head: vec![n; n],
            real_to_virt: vec![n; n],
            virt_to_real: vec![n; n],
        }
    }
    pub fn connect(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
    }

    /// O(N)
    pub fn build(&mut self, root: usize) {
        self.dfs1(root);
        self.dfs2(root);
        self.bfs(root);
    }

    // 部分木の大きさを計算する
    fn dfs1(&mut self, root: usize) {
        self.depth[root] = 0;
        self.par[root] = None;
        self.dfs1_sub(root, None);
    }
    fn dfs1_sub(&mut self, u: usize, par: Option<usize>) -> usize {
        let mut cnt = 1;
        for v in self.g[u].clone() {
            if Some(v) == par { continue; }
            self.depth[v] = self.depth[u] + 1;
            self.par[v] = Some(u);
            cnt += self.dfs1_sub(v, Some(u));
        }
        self.subcnt[u] = cnt;
        cnt
    }
    
    // ヘビーパスを探す
    fn dfs2(&mut self, root: usize) {
        self.dfs2_sub(root, None);
    }
    fn dfs2_sub(&mut self, u: usize, par: Option<usize>) {
        let mut maxv = 0;
        let mut heavy_next = None;

        let cld = self.g[u].clone();

        // ヘビーパスを決める
        for &v in &cld {
            if Some(v) == par { continue; }
            if self.subcnt[v] > maxv { 
                maxv = self.subcnt[v];
                heavy_next = Some(v);
            }
        }
        // ヘビーパスがあるならそれを伸ばす
        if let Some(hn) = heavy_next {
            self.heavy_next[u] = Some(hn);
            self.dfs2_sub(hn, Some(u));
        }
        // ライトパスはまたルートからやり直し
        for &v in &cld {
            if Some(v) == par || Some(v) == heavy_next { continue; }
            self.dfs2_sub(v, Some(u));
        }
    }

    fn bfs(&mut self, root: usize) {
        let mut cur_virt_id = 0;
        let mut q = VecDeque::new();
        q.push_back(root);
        // ヘビーパスの先頭から下っていく
        while let Some(h) = q.pop_front() {
            let mut cur0 = Some(h);
            while cur0.is_some() {
                let cur = cur0.unwrap();
                self.real_to_virt[cur] = cur_virt_id;
                self.virt_to_real[cur_virt_id] = cur;
                cur_virt_id += 1;
                self.heavy_head[cur] = h;
                for v in self.g[cur].clone() {
                    if Some(v) == self.par[cur] || Some(v) == self.heavy_next[cur] { continue; }
                    q.push_back(v);
                }
                cur0 = self.heavy_next[cur];
            }
        }
    }

    /// O(log N)
    pub fn lca(&self, u: usize, v: usize) -> usize {
        let mut l = u;
        let mut r = v;
        loop {
            if self.real_to_virt[l] > self.real_to_virt[r] {
                std::mem::swap(&mut l, &mut r);
            }
            // 同じヘビーパスの上に乗っている
            // よって、vidの低い方がLCAと確定する
            if self.heavy_head[l] == self.heavy_head[r] {
                return l;
            }
            // 一つ上のヘビーパスに移る
            r = self.par[self.heavy_head[r]].unwrap();
        }
    }

    /// O(N)
    pub fn vertex_decomposition(&self) -> Vec<(usize, usize)> {
        let mut vhead = vec![self.n; self.n];
        for i in 0..self.n {
            vhead[i] = self.real_to_virt[self.heavy_head[i]];
        }
        let mut hs = std::collections::HashMap::new();
        for x in vhead {
            *hs.entry(x).or_insert(0) += 1;
        }
        let mut res = vec![];
        for (k,v) in hs {
            res.push((k,k+v-1));
        }
        res
    }

    /// O(N)
    pub fn edge_decomposition(&self) -> Vec<(usize,usize)> {
        let V = self.vertex_decomposition();
        let mut res = vec![];
        for (u,v) in V {
            let u = if u==0 {
                1
            } else {
                u
            };
            res.push((u,v));
        }
        res
    }

    pub fn vertex_decomposition_between(&self, u: usize, v: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];

        let mut l = u;
        let mut r = v;
        loop {
            if self.real_to_virt[l] > self.real_to_virt[r] {
                std::mem::swap(&mut l, &mut r);
            }
            let p = (std::cmp::max(self.real_to_virt[self.heavy_head[r]], self.real_to_virt[l]), self.real_to_virt[r]);
            res.push(p);
            if self.heavy_head[l] != self.heavy_head[r] {
                r = self.par[self.heavy_head[r]].unwrap();
            } else { break; }
        }

        res
    }

    pub fn edge_decomposition_between(&self, u: usize, v: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];

        let mut l = u;
        let mut r = v;
        loop {
            if self.real_to_virt[l] > self.real_to_virt[r] {
                std::mem::swap(&mut l, &mut r);
            }
            if self.heavy_head[l] != self.heavy_head[r] {
                let p = (self.real_to_virt[self.heavy_head[r]], self.real_to_virt[r]);
                res.push(p);
                r = self.par[self.heavy_head[r]].unwrap();
            } else {
                if l != r {
                    let p = (self.real_to_virt[l]+1, self.real_to_virt[r]);
                    res.push(p);
                }
                break;
            }
        }

        res
    }

    pub fn distance(&self, u: usize, v: usize) -> usize {
        self.depth[u] + self.depth[v] - 2 * self.depth[self.lca(u,v)]
    }
}

#[test]
fn test_hl_decomposition() {
    let mut hl = HLDecomposition::new(13);
    let es = vec![(0,1),(0,2),(0,3),(1,4),(1,5),(8,4),(4,9),(6,2),(6,10),(6,11),(6,12),(3,7)];
    for (u,v) in es {
        hl.connect(u,v);
    }
    hl.build(0);

    let lca_test = vec![
        (0,0,0),
        (8,10,0),
        (8,12,0),
        (4,5,1),
        (2,7,0),
        (3,7,3),
        (10,12,6),
        (2,12,2),
    ];
    for (u,v,lca) in lca_test {
        assert_eq!(hl.lca(u,v), lca);
        assert_eq!(hl.lca(v,u), lca);
    }

    dbg!(hl.vertex_decomposition());
    dbg!(hl.edge_decomposition());
    dbg!(hl.vertex_decomposition_between(8, 6));
    dbg!(hl.edge_decomposition_between(8, 6));
    dbg!(hl.vertex_decomposition_between(10, 7));
    dbg!(hl.edge_decomposition_between(10, 7));
}