struct HLDecomposition {
    n: usize,
    g: Vec<Vec<usize>>,
    subcnt: Vec<usize>,
    depth: Vec<usize>,
    par: Vec<Option<usize>>,
    heavy_next: Vec<Option<usize>>,
    heavy_head: Vec<usize>,
    heavy_id: Vec<usize>,
    heavy_id_inv: Vec<usize>,
}

impl HLDecomposition {

    fn new(n: usize) -> Self {
        HLDecomposition {
            n: n,
            g: vec![vec![]; n],
            subcnt: vec![0; n],
            depth: vec![0; n],
            par: vec![None; n],
            heavy_next: vec![None; n],
            heavy_head: vec![n; n],
            heavy_id: vec![n; n],
            heavy_id_inv: vec![n; n],
        }
    }

    fn connect(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
    }

    fn build(&mut self, root: usize) {
        self.dfs1(root);
        self.dfs2(root);
        self.bfs(root);
    }

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
    
    fn dfs2(&mut self, root: usize) {

    }

    fn bfs(&mut self, root: usize) {

    }
}

#[test]
fn test_hl_decomposition() {
    let mut hl = HLDecomposition::new(13);
    let es = vec![(0,1),(0,2),(0,3),(1,4),(1,5),(4,8),(4,9),(2,6),(6,10),(6,11),(6,12),(3,7)];
    for (u,v) in es {
        hl.connect(u,v);
    }
    hl.build(0);
    dbg!(&hl.depth);
    dbg!(&hl.par);
    dbg!(&hl.subcnt);
}