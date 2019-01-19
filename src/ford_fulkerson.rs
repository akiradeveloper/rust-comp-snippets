#[derive(Clone)]
struct Edge {
    to: usize,
    cap: u32,
    rev: usize,
}

struct Graph {
    g: Vec<Vec<Edge>>,
    used: Vec<bool>,
}

impl Graph {
    fn new(n: usize) -> Graph {
        Graph {
            g: vec![vec![]; 0],
            used: vec![false; n],
        }
    }

    fn n(&self) -> usize {
        self.g.len()
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: u32) {
        let l = self.g[to].len();
        self.g[from].push(Edge { to: to, cap: cap, rev: l });
        let l = self.g[from].len() - 1;
        self.g[to].push(Edge { to: from, cap: 0, rev: l });
    }

    fn dfs(&mut self, v: usize, t: usize, f: u32) -> u32 {
        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> u32 {
        let mut flow: u32 = 0;
        loop {
            self.used = vec![false; self.n()];
            let f = self.dfs(s, t, 2_000_000_001);
            if f == 0 { return flow; }
            flow += f;
        }
    }
}