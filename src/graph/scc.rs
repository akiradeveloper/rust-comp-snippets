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
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![vec![];n],
            r_g: vec![vec![];n],
            post_order: VecDeque::new(),
            used: vec![false; n],
            order: vec![n; n],
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.r_g[v].push(u);
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
    let e = vec![
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
    let n = e.len();
    let mut g = SCC::new(n);
    for u in 0..n {
        for v in e[u].clone() {
            g.add_edge(u,v);
        }
    }
    g.build();

    assert_eq!(g.order, [0,1,2,2,2,3,3,3,4,6,6,5]);
}