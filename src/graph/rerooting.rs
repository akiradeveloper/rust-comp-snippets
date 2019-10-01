trait Foldable {
    type T: Clone + std::fmt::Debug;
    fn identity() -> Self::T;
    fn fold(acc: &Self::T, x: &Self::T) -> Self::T;
    fn merge(a: &Self::T, b: &Self::T) -> Self::T;
}
struct ReRooting<F: Foldable> {
    g: Vec<Vec<usize>>,
    dp: Vec<Vec<F::T>>,
}
impl<F: Foldable> ReRooting<F> {
    fn new(g: Vec<Vec<usize>>) -> Self {
        let mut dp = vec![];
        for u in 0..g.len() {
            dp.push(vec![F::identity(); g[u].len()]);
        }
        ReRooting {
            g: g,
            dp: dp,
        }
    }

    fn dfs1(&mut self, par: Option<usize>, u: usize) -> F::T {
        let mut ret = F::identity();
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            if Some(v) == par { continue; } 

            let r = self.dfs1(Some(u), v);
            self.dp[u][i] = r.clone();
            ret = F::fold(&ret, &r);
        }
        ret
    }

    fn dfs2(&mut self, par: Option<usize>, u: usize, u2par: F::T) {
        let n = self.g[u].len();
        let mut prefix = vec![F::identity(); n];
        let mut suffix = vec![F::identity(); n];
        for i in 0..n {
            let v = self.g[u][i];
            if Some(v) == par {
                self.dp[u][i] = u2par.clone();
            }
        }
        for i in 0..n {
            let l = if i>0 { prefix[i-1].clone() } else { F::identity() };
            prefix[i] = F::fold(&l, &self.dp[u][i]);
        }
        for i in (0..n).rev() {
            let r = if i < n-1 { suffix[i+1].clone() } else { F::identity() };
            suffix[i] = F::fold(&r, &self.dp[u][i]);
        }
        for i in 0..n {
            let v = self.g[u][i];
            if Some(v) == par { continue; }
            
            // 枝iを除いた値を計算する
            let l = if i>0 { prefix[i-1].clone() } else { F::identity() };
            let r = if i<n-1 { suffix[i+1].clone() } else { F::identity() };
            self.dfs2(Some(u), v, F::merge(&l, &r));
        }
    }

    pub fn dfs(&mut self, u: usize) {
        let p = self.dfs1(None, u);
        self.dfs2(None, u, p);
    }

    pub fn solve(&self, u: usize) -> F::T {
        let mut acc = F::identity();
        for d in &self.dp[u] {
            acc = F::fold(&acc, d);
        }
        acc
    }
}