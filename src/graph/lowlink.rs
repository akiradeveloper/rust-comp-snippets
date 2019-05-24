struct LowLink<'a> {
    g: &'a [Vec<usize>],
    used: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    articulation: Vec<usize>,
    bridge: Vec<(usize, usize)>,
}

impl <'a> LowLink<'a> {
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
    }
    fn do_build(&mut self, u: usize, k: usize, par: Option<usize>) -> usize {
        let k = k + 1;
        self.used[u] = true;
        self.ord[u] = k;
        self.low[u] = self.ord[u];
        let mut is_articulation = false;
        let mut cnt = 0;
        for &v in &self.g[u] {
            if self.used[v] == false {
               cnt += 1; 
               let k = self.do_build(v, k, Some(u));
               self.low[u] = std::cmp::min(self.low[u], self.low[v]);
               is_articulation |= par.is_some() && self.low[v] >= self.ord[u];
               if self.ord[u] < self.low[v] {
                   self.bridge.push((u, v));
               }
            } else if Some(v) == par {
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