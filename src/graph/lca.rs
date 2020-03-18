use cargo_snippet::snippet;

#[snippet("LCA")]
struct LCA {
    tree: Vec<Vec<usize>>,
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}
#[snippet("LCA")]
impl LCA {
    pub fn new(n: usize) -> Self {
        let mut log_n = (n as f64).log2().ceil() as usize;
        if log_n == 0 {
            log_n = 1;
        }
        assert!(log_n > 0);
        LCA {
            tree: vec![vec![];n],
            parent: vec![vec![None; n]; log_n],
            depth: vec![0; n],
        }
    }
    pub fn connect(&mut self, u: usize, v: usize) {
        self.tree[u].push(v);
        self.tree[v].push(u);
    }
    // store direct parent and depth
    fn dfs(&mut self, u: usize, parent: Option<usize>, depth: usize) {
        self.parent[0][u] = parent;
        self.depth[u] = depth;
        for i in 0 .. self.tree[u].len() {
            let v = self.tree[u][i];
            if Some(v) != parent {
                self.dfs(v, Some(u), depth+1);
            }
        }
    }
    pub fn build(&mut self, root: usize) {
        self.dfs(root, None, 0);

        let mut k = 0;
        while k+1 < self.parent.len() {
            for u in 0 .. self.tree.len() {
                if self.parent[k][u].is_some() {
                    self.parent[k+1][u] = self.parent[k][self.parent[k][u].unwrap()]
                } 
            }
            k += 1;
        }
    }
    pub fn lca(&self, u: usize, v: usize) -> usize {
        let (mut v0, mut v1) = if self.depth[u] <= self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        assert!(self.depth[v1] >= self.depth[v0]);

        // move v1 up until depth of v0 and v1 gets equal.
        for k in 0 .. self.parent.len() {
            if (((self.depth[v1] - self.depth[v0]) >> k) & 1) > 0 {
                assert!(self.parent[k][v1].is_some());
                v1 = self.parent[k][v1].unwrap();
            }
        }
        assert!(self.depth[v1] >= self.depth[v0]);
        assert!(self.depth[v1] == self.depth[v0]);
        if (v0 == v1) {
            return v0;
        }
        for k in (0..self.parent.len()).rev() {
            // LCA's parent is LCA
            if self.parent[k][v0] != self.parent[k][v1] {
                assert!(self.parent[k][v0].is_some());
                assert!(self.parent[k][v1].is_some());
                v0 = self.parent[k][v0].unwrap();
                v1 = self.parent[k][v1].unwrap();
            }
        }
        return self.parent[0][v0].unwrap();
    }
    pub fn distance(&self, u: usize, v: usize) -> usize {
        self.depth[u] + self.depth[v] - 2 * self.depth[self.lca(u, v)]
    }
}

#[test]
fn test_lca() {
    let tree = vec![
        vec![1,2],
        vec![0,3,4],
        vec![0,5],
        vec![1],
        vec![1,6,7],
        vec![2],
        vec![4],
        vec![4],
    ];
    let mut e = vec![
        (0,1),(0,2),(1,3),(1,4),(2,5),
        (4,6),(4,7),
    ];
    let mut lca = LCA::new(8);
    for (u,v) in e {
        lca.connect(u,v);
    }
    lca.build(0);

    let Q = vec![
        (1,2,0,2),
        (3,7,1,3),
        (4,4,4,0),
        (3,5,0,4),
        (1,7,1,2),
        (2,5,2,1),
    ];
    for (u,v,p,d) in Q {
        assert_eq!(lca.lca(u, v), p);
        assert_eq!(lca.distance(u, v), d);
    }
}