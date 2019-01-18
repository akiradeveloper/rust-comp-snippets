fn dijkstra(n: usize, cost: &[Vec<u32>], s: usize) -> Vec<u32> {
    const INF: u32 = 2_000_000_001;
    let mut d = vec![INF; n];
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
            d[u] = std::cmp::min(d[u], d[v] + cost[v][u])
        }
    }
    d
}
#[test]
fn test_dijkstra() {}

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

struct Edge {
    to: usize,
    cost: u32,
}

fn dijkstra_heap(n: usize, g: &[Vec<Edge>], s: usize) -> Vec<u32> {
    let INF = 2_000_000_001;
    let mut queue = std::collections::BinaryHeap::new();
    let mut d = vec![INF; n];

    d[s] = 0;
    queue.push(State { cost: 0, v: s });

    while let Some(State { cost, v }) = queue.pop() {
        if d[v] < cost {
            continue;
        }
        for e in &g[v] {
            if d[e.to] > d[v] + e.cost {
                d[e.to] = d[v] + e.cost;
                queue.push(State {
                    cost: e.cost,
                    v: e.to,
                });
            }
        }
    }

    d
}

#[test]
fn test_dijkstra_heap() {}
