use std::collections::VecDeque;

#[doc = "if there is a loop len(out) will be 0"]
#[snippet = "TopologicalSort"]
struct TopologicalSort {
    g: Vec<Vec<usize>>,
    colors: Vec<bool>,
    indeg: Vec<u32>,
    Q: VecDeque<usize>,
    out: Vec<usize>,
}

#[snippet = "TopologicalSort"]
impl TopologicalSort {
    #[doc = "g = directed"]
    pub fn new(g: Vec<Vec<usize>>) -> Self {
        let n = g.len();
        let mut colors = vec![false; n];
        let mut indeg = vec![0; n];
        for u in 0..n {
            let conn = &g[u];
            for &next in conn {
                indeg[next] += 1;
            }
        }
        TopologicalSort {
            g: g,
            Q: VecDeque::new(),
            colors: colors,
            indeg: indeg,
            out: Vec::new(),
        }
    }
    fn bfs(&mut self, s: usize) {
        self.Q.push_back(s);
        self.colors[s] = true;
        while !self.Q.is_empty() {
            let u = self.Q.pop_front().unwrap();
            self.out.push(u);
            for &v in &self.g[u] {
                self.indeg[v] -= 1;
                if self.indeg[v] == 0 && self.colors[v] == false {
                    self.colors[v] = true;
                    self.Q.push_back(v);
                }
            }

        }
    }
    #[doc = "O(V+E)"]
    pub fn tsort(&mut self) {
        let n = self.g.len();
        for u in 0..n {
            if self.indeg[u] == 0 && self.colors[u] == false {
                self.bfs(u)
            }
        }
    }
}

#[test]
fn test_tsort() {
    let mut conns = vec![
        vec![1],
        vec![2],
        vec![],
        vec![1,4],
        vec![5],
        vec![2],
    ];
    let mut tsort = TopologicalSort::new(conns);
    tsort.tsort();
    assert_eq!(tsort.out, [0,3,1,4,5,2]);
}

#[test]
fn test_tsort_loop() {
    let mut conns = vec![
        vec![1],
        vec![2],
        vec![0]
    ];
    let mut tsort = TopologicalSort::new(conns);
    tsort.tsort();
    assert_eq!(tsort.out.len(), 0);
}