mod bfs01 {
    // connected[i][i] == false
    #[doc = "shortest path from directed matrix graph with 0/1 cost. O(E)"]
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

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
#[snippet = "DijkstraHeap"]
struct DijkstraHeap<State: std::hash::Hash + std::cmp::Eq> {
    cur: usize,
    que: Vec<Vec<State>>,
    next: BinaryHeap<i64>,
}
#[snippet = "DijkstraHeap"]
impl <State: Default + Clone + std::hash::Hash + std::cmp::Eq> DijkstraHeap<State> {
    pub fn new(maxdist: usize) -> DijkstraHeap<State> {
        DijkstraHeap {
            cur: maxdist+1,
            que: vec![vec![]; maxdist+2],
            next: BinaryHeap::new(),
        }
    }
    fn pop_retry(&mut self) -> Option<(usize, State)> {
        self.forward_cur();
        let q = &mut self.que[self.cur];
        if q.is_empty() {
            None
        } else {
            let e = q.pop().unwrap();
            Some((self.cur, e))
        }
    }
    pub fn pop(&mut self) -> Option<(usize, State)> {
        let q = &mut self.que[self.cur];
        if q.is_empty() {
            self.pop_retry()
        } else {
            let e = q.pop().unwrap();
            Some((self.cur, e))
        }
    }
    pub fn push(&mut self, cost: usize, st: State) {
        if self.que[cost].is_empty() {
            self.next.push(-1 * cost as i64);
        }
        self.que[cost].push(st);
    }
    fn is_empty_retry(&mut self) -> bool {
        self.forward_cur();
        let q = &self.que[self.cur];
        q.is_empty()
    }
    pub fn is_empty(&mut self) -> bool {
        let q = &self.que[self.cur];
        if q.is_empty() {
            self.is_empty_retry()
        } else {
            false
        }
    }
    fn forward_cur(&mut self) {
        if let Some(nx) = self.next.pop() {
            self.cur = -nx as usize;
        } 
    }
}
#[test]
fn test_dijkstra_heap_struct() {
    let mut q: DijkstraHeap<char> = DijkstraHeap::new(10000);
    assert!(q.is_empty());
    assert_eq!(q.pop(), None);
    q.push(0,'x');
    q.push(10000,'a');
    assert_eq!(q.pop(), Some((0,'x')));
    assert!(!q.is_empty());
    assert_eq!(q.pop(), Some((10000,'a')));
    assert!(q.is_empty());
    assert_eq!(q.pop(), None);
    assert_eq!(q.pop(), None);
}
const SZ: usize = 10000;
#[bench]
fn bench_dijkstra_heap_push(b: &mut test::Bencher) {
    let mut s = DijkstraHeap::new(SZ);
    let mut data = vec![];
    for i in 0..SZ {
        data.push(i);
    }
    b.iter(||
        for &x in &data {
            s.push(x,0);
        }
    );
}
#[bench]
fn bench_dijkstra_heap_pop(b: &mut test::Bencher) {
    let mut s = DijkstraHeap::new(SZ);
    for i in 0..SZ {
        s.push(i, 0);
    }
    b.iter(||
        for _ in 0..SZ {
            s.pop();
        }
    );
}
#[bench]
fn bench_dijkstra_heap_is_empty(b: &mut test::Bencher) {
    let mut s = DijkstraHeap::new(SZ);
    s.push(1,0);
    b.iter(||
        for _ in 0..SZ {
            s.is_empty();
        }
    );
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
