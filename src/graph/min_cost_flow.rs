mod bellman_ford {
    struct Edge {
        to: usize,
        cap: u32,
        cost: i32,
        rev: usize,
    }

    struct Network {
        g: Vec<Vec<Edge>>,
    }

    impl Network {
        fn add_edge(&mut self, from: usize, to: usize, cap: u32, cost: i32) {
            let from_rev = self.g[to].len();
            let to_rev = self.g[from].len();
            self.g[from].push(Edge {
                to: to,
                cap: cap,
                cost: cost,
                rev: from_rev,
            });
            self.g[to].push(Edge {
                to: from,
                cap: 0,
                cost: -cost,
                rev: to_rev,
            });
        }

        fn n(&self) -> usize {
            self.g.len()
        }

        fn min_cost_flow(&mut self, s: usize, t: usize, f: u32) -> i32 {
            let mut res: i32 = 0;
            let mut prevv = vec![0; self.n()];
            let mut preve = vec![0; self.n()];
            let mut f = f;
            const INF: i32 = 2_000_000_001;

            while f > 0 {
                let mut dist = vec![INF; self.n()];
                dist[s] = 0;
                let mut update = true;
                while update {
                    update = false;
                    for v in 0..self.n() {
                        if dist[v] == INF {
                            continue;
                        }
                        for i in 0..self.g[v].len() {
                            let e = &self.g[v][i];
                            if e.cap > 0 && dist[e.to] > dist[v] + e.cost {
                                dist[e.to] = dist[v] + e.cost;
                                prevv[e.to] = v;
                                preve[e.to] = i;
                                update = true;
                            }
                        }
                    }
                }

                if dist[t] == INF {
                    return -1;
                }

                let mut actual_flow = f;

                let mut v = t;
                while v != s {
                    actual_flow = std::cmp::min(actual_flow, self.g[prevv[v]][preve[v]].cap);
                    v = prevv[v];
                }

                f -= actual_flow;
                res += actual_flow as i32 * dist[t];

                let mut v = t;
                while v != s {
                    let e = &mut self.g[prevv[v]][preve[v]];
                    e.cap -= actual_flow;
                    v = prevv[v];
                }
            }
            return res;
        }

      
    }
    #[test]
    fn test() {}
}