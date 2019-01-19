#[derive(Clone)]
struct Edge {
    to: usize,
    cap: u32,
    rev: usize, }

struct Network {
    g: Vec<Vec<Edge>>,
    used: Vec<bool>,
}

impl Network {
    fn new(n: usize) -> Network {
        Network {
            g: vec![vec![]; n],
            used: vec![false; n],
        }
    }

    fn n(&self) -> usize {
        self.g.len()
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: u32) {
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

    fn dfs(&mut self, v: usize, t: usize, f: u32) -> u32 {
        if v == t {
            return f;
        }
        self.used[v] = true;
        for i in 0..self.g[v].len() {
            let e = self.g[v][i].clone();
            if !self.used[e.to] && e.cap > 0 {
                let d = self.dfs(e.to, t, std::cmp::min(f, e.cap));
                if d > 0 {
                    {
                        let e = &mut self.g[v][i];
                        e.cap -= d;
                    }
                    self.g[e.to][e.rev].cap += d;
                    return d;
                }
            }
        }
        return 0;
    }

    fn max_flow(&mut self, s: usize, t: usize) -> u32 {
        let mut flow: u32 = 0;
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
fn test() {
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
