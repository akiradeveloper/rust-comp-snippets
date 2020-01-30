use std::collections::VecDeque;

#[snippet = "SCC"]
pub struct SCC {
    g: Vec<Vec<usize>>,
    r_g: Vec<Vec<usize>>,
    post_order: VecDeque<usize>,
    used: Vec<bool>,
    pub order: Vec<usize>,
}

#[snippet = "SCC"]
#[doc = "nodes that communicates each others are contracted into one node"]
impl SCC {
    #[doc = "directed"]
    pub fn new(g: Vec<Vec<usize>>) -> Self {
        let n = g.len();
        let mut r_g = vec![vec![]; n];
        for u in 0..n {
            let conn = &g[u];
            for &v in conn {
                r_g[v].push(u);
            }
        }
        Self {
            g,
            r_g,
            post_order: VecDeque::new(),
            used: vec![false; n],
            order: vec![n; n],
        }
    }
    fn dfs(&mut self, u: usize) {
        self.used[u] = true;
        for i in 0 .. self.g[u].len() {
            let v = self.g[u][i];
            if !self.used[v] {
                self.dfs(v);
            }
        }
        self.post_order.push_front(u);
    }
    fn rdfs(&mut self, u: usize, k: usize) {
        self.used[u] = true;
        self.order[u] = k;
        for i in 0 .. self.r_g[u].len() {
            let v = self.r_g[u][i];
            if !self.used[v] {
                self.rdfs(v, k);
            }
        }
    }
    pub fn build(&mut self) {
        for v in 0 .. self.g.len() {
            if !self.used[v] {
                self.dfs(v);
            }
        }
        // dbg!(&self.post_order);
        self.used = vec![false; self.g.len()];
        let mut k = 0;
        for i in 0 .. self.post_order.len() {
            let v = self.post_order[i];
            if !self.used[v] {
                self.rdfs(v, k);
                k += 1;
            }
        }
    }
}

#[test]
fn test_scc() {
    let g = vec![
        vec![1],
        vec![2,3],
        vec![3],
        vec![4],
        vec![2,5],
        vec![6],
        vec![7,8,9],
        vec![5],
        vec![9,11],
        vec![10],
        vec![9],
        vec![],
    ];
    let mut scc = SCC::new(g);
    scc.build();

    assert_eq!(scc.order, [0,1,2,2,2,3,3,3,4,6,6,5]);
}