mod bfs01 {
    // connected[i][i] == false
    #[doc = "shortest path from directed matrix graph with 0/1 cost. O(V)"]
    #[snippet = "bfs01"]
    fn bfs01(g: &[Vec<i64>], s: usize, inf: i64) -> Vec<i64> {
        use std::collections::VecDeque;
        let n = g.len();
        let mut dp = vec![inf; n];
        let mut deque = VecDeque::new();
        dp[s] = 0;
        deque.push_back(s);
        while !deque.is_empty() {
            let i = deque.pop_front().unwrap();
            let cur_min_cost = dp[i];
            for j in 0..n {
                if g[i][j] < inf {
                    if g[i][j] == 1 {
                        let new_cost = cur_min_cost + 1;
                        if new_cost < dp[j] {
                            dp[j] = new_cost;
                            deque.push_back(j);
                        }
                    } else {
                        let new_cost = cur_min_cost;
                        if new_cost < dp[j] {
                            dp[j] = new_cost;
                            deque.push_front(j);
                        }
                    }
                }
            }
        }
        dp
    }
    #[test]
    fn test_bfs01() {
        let map = [
            ['.','.','.'],
            ['.','#','.'],
            ['.','.','.'],
        ];
        let mut g = vec![vec![1<<30; 9]; 9];
        let pos = |i, j| {
            i*3 + j
        };
        for i in 0..3 {
            for j in 0..3 {
                let u = pos(i, j);
                if map[i][j] == '#' {
                    continue;
                }
                if i>0 && map[i-1][j] == '.' {
                    let v = pos(i-1, j);
                    g[u][v] = 1;
                }
                if i<3-1 && map[i+1][j] == '.' {
                    let v = pos(i+1, j);
                    g[u][v] = 1;
                }
                if j>0 && map[i][j-1] == '.' {
                    let v = pos(i, j-1);
                    g[u][v] = 1;
                }
                if j<3-1 && map[i][j+1] == '.' {
                    let v = pos(i, j+1);
                    g[u][v] = 1;
                }
            }
        }
        dbg!(&g);
        let dp = bfs01(&g, 0, 1<<30); 
        dbg!(&dp);
    }
}

mod dijkstra {
    // self = 0
    // not connected = inf;
    #[doc = "shortest path from directed matrix graph"]
    fn dijkstra(g: &[Vec<i64>], s: usize, inf: i64) -> Vec<i64> {
        let n = g.len();
        let mut d = vec![inf; n];

        d[s] = 0;
        let mut used = vec![false; n];
        loop {
            let mut v = None;
            for u in 0..n {
                if !used[u] && (v.is_none() || d[u] < d[v.unwrap()]) {
                    v = Some(u)
                }
            }
            if v.is_none() {
                break;
            }
            let v = v.unwrap();
            used[v] = true;

            for u in 0..n {
                d[u] = std::cmp::min(d[u], d[v] + g[v][u])
            }
        }
        d
    }
    #[test]
    fn test_dijkstra() {}
}

mod djikstra_heap {
    #[snippet = "dijkstra"]
    #[derive(Clone,Copy,Debug)]
    struct Edge {
        to: usize,
        cost: i64,
    }

    #[doc = "g: directed adjacent graph with non-negative costs. O(ElogV)"]
    #[snippet = "dijkstra"]
    fn dijkstra_heap(g: &[Vec<Edge>], s: usize, inf: i64) -> Vec<i64> {
        let n = g.len();
        let mut queue = std::collections::BinaryHeap::new(); // max-heap
        let mut d = vec![inf; n];

        d[s] = 0;
        queue.push((0, s));

        while let Some((cost, v)) = queue.pop() {
            let cost = -cost;

            if d[v] < cost {
                continue;
            }
            for e in &g[v] {
                let new_cost = cost + e.cost;
                if d[e.to] > new_cost {
                    d[e.to] = new_cost;
                    queue.push((-new_cost, e.to));
                }
            }
        }

        d
    }

    #[test]
    fn test_dijkstra_heap() {}
}

mod bellman_ford {
    #[snippet = "bellman_ford"]
    #[derive(Clone,Copy,Debug)]
    struct Edge {
        from: usize,
        to: usize,
        cost: i64,
    }

    #[doc = "es: directed edges. negative costs allowed. O(V^2)"]
    #[snippet = "bellman_ford"]
    fn bellman_ford(n: usize, es: &[Edge], source: usize) -> Vec<i64> {
        const INF: i64 = 1<<60;
        let mut d = vec![INF; n];
        d[source] = 0;
        loop {
            let mut update = false;
            for e in es {
                if d[e.from] != INF && d[e.to] > d[e.from] + e.cost {
                    d[e.to] = d[e.from] + e.cost;
                    update = true;
                }
            }
            if !update {
                break;
            }
        }
        d
    }

    #[snippet = "bellman_ford"]
    fn find_negative_loop(n: usize, es: &[Edge]) -> bool {
        let mut d = vec![0; n];
        for i in 0..n {
            for e in es {
                if d[e.to] > d[e.from] + e.cost {
                    d[e.to] = d[e.from] + e.cost;
                    if i == n - 1 {
                        return true;
                    }
                }
            }
        }
        false
    }
}
