#[snippet = "Lowlink"]
fn minmax(p: (usize, usize)) -> (usize, usize) {
    if p.0 <= p.1 {
        p
    } else {
        (p.1, p.0)
    }
}

#[snippet = "Lowlink"]
struct LowLink<'a> {
    g: &'a [Vec<usize>],
    used: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    pub articulation: Vec<usize>,
    pub bridge: Vec<(usize, usize)>,
}

#[snippet = "Lowlink"]
#[doc = "find articulation points and bridges at the same time"]
impl <'a> LowLink<'a> {
    #[doc = "g: undirected adjacency graph"]
    fn new(g: &'a [Vec<usize>]) -> LowLink {
        let n = g.len();
        let mut used = vec![false; n];
        let mut ord = vec![0; n];
        let mut low = vec![0; n];
        let articulation = vec![];
        let bridge = vec![];
        LowLink {
            g,
            used,
            ord,
            low,
            articulation,
            bridge,
        }
    }
    fn build(&mut self) {
        self.do_build(0, 0, None);
        self.articulation.sort();
        self.bridge.sort();
    }
    fn do_build(&mut self, u: usize, k: usize, par: Option<usize>) -> usize {
        let mut k = k;
        self.used[u] = true;
        self.ord[u] = k;
        k += 1;
        self.low[u] = self.ord[u];
        let mut is_articulation = false;
        let mut cnt = 0;
        for &v in &self.g[u] {
            if !self.used[v] {
               cnt += 1; 
               k = self.do_build(v, k, Some(u));
               self.low[u] = std::cmp::min(self.low[u], self.low[v]);
               is_articulation |= par.is_some() && self.low[v] >= self.ord[u];
               if self.ord[u] < self.low[v] {
                   self.bridge.push(minmax((u, v)));
               }
            } else if Some(v) != par {
                self.low[u] = std::cmp::min(self.low[u], self.ord[v]);
            } else {}
        }
        is_articulation |= par.is_none() && cnt > 1;
        if is_articulation {
            self.articulation.push(u);
        }
        k
    }
}

#[test]
fn test_lowlink() {
    let g = vec![
        vec![1,2],
        vec![0,2,3],
        vec![0,1,3],
        vec![1,2,6],
        vec![6],
        vec![6,7],
        vec![3,4,5,7],
        vec![5,6],
    ];
    let mut lowlink = LowLink::new(&g);
    lowlink.build();
    assert_eq!(lowlink.articulation, [3,6]);
    assert_eq!(lowlink.bridge, [(3,6),(4,6)]);
}