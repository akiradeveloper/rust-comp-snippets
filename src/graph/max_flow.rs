// verified: GRL_6_A
mod ford_fulkerson {
    #[snippet = "ford_fulkerson"]
    #[derive(Clone,Copy,Debug)]
    struct Edge {
        to: usize,
        cap: i64,
        rev: usize,
    }
    #[snippet = "ford_fulkerson"]
    struct Network {
        g: Vec<Vec<Edge>>,
        used: Vec<bool>,
    }

    #[doc = "directed flow graph. O(FE)"]
    #[snippet = "ford_fulkerson"]
    impl Network {
        pub fn new(n: usize) -> Network {
            Network {
                g: vec![vec![]; n],
                used: vec![false; n],
            }
        }

        fn n(&self) -> usize {
            self.g.len()
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
            assert!(cap>=0);

            let from_rev = self.g[to].len();
            let to_rev = self.g[from].len();

            self.g[from].push(Edge {
                to: to,
                cap: cap,
                rev: from_rev,
            });
            self.g[to].push(Edge {
                to: from,
                cap: 0,
                rev: to_rev,
            });
        }

        fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
            if v == t {
                return f;
            }
            self.used[v] = true;
            for i in 0..self.g[v].len() {
                let e = self.g[v][i].clone();
                if !self.used[e.to] && e.cap > 0 {
                    let d = self.dfs(e.to, t, std::cmp::min(f, e.cap));
                    if d > 0 {
                        self.g[v][i].cap -= d;
                        self.g[e.to][e.rev].cap += d;
                        return d;
                    }
                }
            }
            return 0;
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
            let mut flow = 0;
            loop {
                self.used = vec![false; self.n()];
                let f = self.dfs(s, t, 2_000_000_001);
                if f == 0 {
                    return flow;
                }
                flow += f;
            }
        }
    }
    #[test]
    fn test_ford_fulkerson() {
        let mut nw = Network::new(5);

        let conns = [
            (0, 1, 10),
            (0, 2, 2),
            (1, 2, 6),
            (1, 3, 6),
            (3, 2, 3),
            (2, 4, 5),
            (3, 4, 8),
        ];

        for conn in &conns {
            nw.add_edge(conn.0, conn.1, conn.2);
        }

        assert_eq!(nw.max_flow(0, 4), 11);
    }
}

// verified: GRL_6_A
mod dinic {
    #[snippet = "dinic"]
    #[derive(Clone,Copy,Debug)]
    struct Edge {
        to: usize,
        cap: i64,
        rev: usize,
    }
    #[snippet = "dinic"]
    struct Network {
        g: Vec<Vec<Edge>>,
        level: Vec<Option<usize>>,
        iter: Vec<usize>,
    }
    #[doc = "direct flow graph. O(EV^2)"]
    #[snippet = "dinic"]
    impl Network {
        fn new(n: usize) -> Network {
            Network {
                g: vec![vec![]; n],
                level: vec![None; n],
                iter: vec![0; n],
            }
        }
        fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
            assert!(cap>=0);

            let from_rev = self.g[to].len();
            let to_rev = self.g[from].len();
            self.g[from].push( Edge { to: to, cap: cap, rev: from_rev } );
            self.g[to].push( Edge { to: from, cap: 0, rev: to_rev } );
        }
        fn n(&self) -> usize {
            self.g.len()
        }
        fn bfs(&mut self, s: usize) {
            self.level = vec![None; self.n()];
            let mut q = std::collections::VecDeque::new();
            q.push_back(s);
            self.level[s] = Some(0);
            while let Some(v) = q.pop_front() {
                for e in &self.g[v] {
                    if e.cap > 0 && self.level[e.to].is_none() {
                        self.level[e.to] = self.level[v].map(|x| x + 1);
                        q.push_back(e.to);
                    }
                }
            }
        }
        fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
            if v == t {
                return f;
            }
            let iter_v_cur = self.iter[v];
            for i in iter_v_cur .. self.g[v].len() {
                let e = self.g[v][i].clone();
                if e.cap > 0 && self.level[v] < self.level[e.to] {
                    let d = self.dfs(e.to, t, std::cmp::min(f, e.cap));
                    if d > 0 {
                        self.g[v][i].cap -= d;
                        self.g[e.to][e.rev].cap += d;
                        return d;
                    }
                }
                self.iter[v] += 1;
            }
            return 0;
        }
        pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
            let mut flow = 0;
            loop {
                self.bfs(s);
                // finally if we could not find any path to t then return flow
                if self.level[t].is_none() {
                    return flow;
                }

                let INF = 2_000_000_001;
                self.iter = vec![0; self.n()];
                let mut f = self.dfs(s, t, INF);
                while f > 0 {
                    flow += f;
                    f = self.dfs(s, t, INF);
                }
            }
        }
    }

    #[test]
    fn test_dinic() {
      let mut nw = Network::new(5);

        let conns = [
            (0, 1, 10),
            (0, 2, 2),
            (1, 2, 6),
            (1, 3, 6),
            (3, 2, 3),
            (2, 4, 5),
            (3, 4, 8),
        ];

        for conn in &conns {
            nw.add_edge(conn.0, conn.1, conn.2);
        }

        assert_eq!(nw.max_flow(0, 4), 11);
    }
}