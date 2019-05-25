struct LCA <'a> {
    root: usize,
    tree: &'a [Vec<usize>],
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<u32>,
}

impl <'a> LCA<'a> {
    fn new(root: usize, tree: &'a [Vec<usize>]) -> Self {
        let n = tree.len();
        let log_n = (n as f64).log2().ceil() as usize;
        Self {
            root,
            tree,
            parent: vec![vec![None; n]; log_n],
            depth: vec![0; n],
        }
    }
    // store direct parent and depth
    fn dfs(&mut self, u: usize, parent: Option<usize>, depth: u32) {
        self.parent[0][u] = parent;
        self.depth[u] = depth;
        for i in 0 .. self.tree[u].len() {
            let v = self.tree[u][i];
            if Some(v) != parent {
                self.dfs(v, Some(u), depth+1);
            }
        }
    }
    fn build(&mut self) {
        self.dfs(self.root, None, 0);
        let mut k = 0;
        while k+1 < self.parent.len() {
            for u in 0 .. self.tree.len() {
                self.parent[k+1][u] = if self.parent[k][u].is_some() {
                    self.parent[k][self.parent[k][u].unwrap()]
                } else {
                    None
                }
            }
            k += 1;
        }
    }
    fn lca(&self, u: usize, v: usize) -> usize {
        let (mut v0, mut v1) = if self.depth[u] <= self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        assert!(self.depth[v1] >= self.depth[v0]);

        // move v1 up until depth of v0 and v1 gets equal.
        for k in 0 .. self.parent.len() {
            if (self.depth[v1] - self.depth[v0]) >> k & 1 > 0 {
                v1 = self.parent[k][v1].unwrap();
            }
        }
        if (v0 == v1) {
            return v0;
        }
        for k in (0..self.parent.len()).rev() {
            // LCA's parent is LCA
            if self.parent[k][v0] != self.parent[k][v1] {
                v0 = self.parent[k][v0].unwrap();
                v1 = self.parent[k][v1].unwrap();
            }
        }
        return self.parent[0][v0].unwrap();
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
    let mut lca = LCA::new(0, &tree);
    lca.build();

    let probs = [
        (1,2,0),
        (3,7,1),
        (4,4,4),
        (3,5,0),
        (1,7,1),
        (2,5,2),
    ];
    for &(u, v, p) in &probs {
        assert_eq!(lca.lca(u, v), p);
    }
}