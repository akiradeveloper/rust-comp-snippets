mod bellman_ford {
    #[derive(Clone)]
    struct Edge {
        to: usize,
        cap: i32,
        cost: i32,
        rev: usize,
    }

    struct Network {
        g: Vec<Vec<Edge>>,
    }

    impl Network {
        fn new(n: usize) -> Network {
            Network {
                g: vec![vec![]; n],
            }
        }
        /// allows negative costs
        fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: i32) {
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

        fn min_cost_flow(&mut self, s: usize, t: usize, f: i32) -> Option<i32> {
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
                    return None;
                }

                let mut actual_flow = f;

                // FIXME this does not include s
                // should be for (u=t; u!=s; u=prevv[u])
                let mut u = t;
                loop {
                    if u == s {
                        break;
                    }
                    u = prevv[u];
                    actual_flow = std::cmp::min(actual_flow, self.g[prevv[u]][preve[u]].cap);
                }

                f -= actual_flow;
                res += actual_flow as i32 * dist[t];

                // FIXME
                let mut u = t;
                loop {
                    if u == s {
                        break;
                    }
                    u = prevv[u];
                    let e = self.g[prevv[u]][preve[u]].clone();
                    self.g[prevv[u]][preve[u]].cap -= actual_flow;
                    self.g[u][e.rev].cap += actual_flow;
                }
            }
            
            Some(res)
        }
    }
    #[test]
    fn test_bellman_ford_min_cost_flow() {
        let mut g = Network::new(5);
        g.add_edge(0, 1, 10, 2);
        g.add_edge(1, 3, 6, 2);
        g.add_edge(1, 2, 6, 6);
        g.add_edge(0, 2, 2, 4);
        g.add_edge(3, 2, 3, 3);
        g.add_edge(2, 4, 5, 2);
        g.add_edge(3, 4, 8, 6);
        println!("{}", g.min_cost_flow(0, 4, 9).unwrap());
    }
}

mod dijkstra {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: u32,
        v: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &State) -> std::cmp::Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.v.cmp(&other.v))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    
    #[derive(Clone)]
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
        fn new(n: usize) -> Network {
            Network {
                g: vec![vec![]; n],
            }
        }

        fn add_edge(&mut self, from: usize, to: usize, cap: u32, cost: u32) {
            let from_rev = self.g[to].len();
            let to_rev = self.g[from].len();
            self.g[from].push(Edge {
                to: to,
                cap: cap,
                cost: cost as i32,
                rev: from_rev,
            });
            self.g[to].push(Edge {
                to: from,
                cap: 0,
                cost: -1 * cost as i32,
                rev: to_rev,
            });
        } 

        fn n(&self) -> usize {
            self.g.len()
        }

        fn min_cost_flow(&mut self, s: usize, t: usize, f: u32) -> Option<u32> {
            let mut res = 0;
            let mut total_flow = f;
            let mut prevv = vec![0; self.n()];
            let mut preve = vec![0; self.n()];

            let mut h = vec![0u32; self.n()];

            while total_flow > 0 {
                let INF: u32 = 2_000_000_001;
                let mut queue = std::collections::BinaryHeap::new();
                let mut dist = vec![INF; self.n()]; // for all >= 0
                dist[s] = 0;
                queue.push( State { cost: 0, v: s } );

                while let Some(State{ cost, v }) = queue.pop() {
                    if dist[v] < cost { continue; }
                    for i in 0 .. self.g[v].len() {
                        let e = &self.g[v][i];
                        let new_dist = (dist[v] as i32 + e.cost + h[v] as i32 - h[e.to] as i32) as u32;
                        assert!(new_dist >= 0);
                        if e.cap > 0 && dist[e.to] > new_dist {
                            dist[e.to] = new_dist;
                            prevv[e.to] = v;
                            preve[e.to] = i;
                            queue.push( State { cost: dist[e.to], v: e.to } );
                        }
                    }
                }
                if dist[t] == INF {
                    return None;
                }
                for v in 0 .. self.n() {
                    h[v] += dist[v];
                }
                
                let mut actual_flow = f;
                // FIXME
                let mut v = t;
                while v != s {
                    actual_flow = std::cmp::min(actual_flow, self.g[prevv[v]][preve[v]].cap);
                    v = prevv[v];
                }

                total_flow -= actual_flow;
                res += actual_flow * h[t];
                // FIXME
                let mut v = t;
                while v != s {
                    let e = self.g[prevv[v]][preve[v]].clone();
                    self.g[prevv[v]][preve[v]].cap -= actual_flow;
                    self.g[v][e.rev].cap += actual_flow;
                    v = prevv[v];
                }
            }

            Some(res)
        }
    }
    #[test]
    fn test_dijkstra_min_cost_flow() {
        let mut g = Network::new(5);
        g.add_edge(0, 1, 10, 2);
        g.add_edge(1, 3, 6, 2);
        g.add_edge(1, 2, 6, 6);
        g.add_edge(0, 2, 2, 4);
        g.add_edge(3, 2, 3, 3);
        g.add_edge(2, 4, 5, 2);
        g.add_edge(3, 4, 8, 6);
        println!("{}", g.min_cost_flow(0, 4, 9).unwrap());
    }
}