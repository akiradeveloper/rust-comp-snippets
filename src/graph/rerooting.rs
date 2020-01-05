#[snippet = "ReRooting"]
trait Foldable {
    type Sum: Copy + std::fmt::Debug;
    type T: Copy + std::fmt::Debug;
    fn identity() -> Self::Sum;
    fn f(x: Self::Sum, y: Self::Sum) -> Self::Sum;
    fn g(acc: Self::Sum, x: Self::T) -> Self::Sum;
}
#[snippet = "ReRooting"]
#[derive(Clone, Copy)]
struct Edge<T, Sum> {
    to: usize,
    data: T,
    dp: Sum,
    ndp: Sum,
}
#[snippet = "ReRooting"]
struct ReRooting<F: Foldable> {
    subdp: Vec<F::Sum>,
    dp: Vec<F::Sum>,
    g: Vec<Vec<Edge<F::T, F::Sum>>>,
}
#[snippet = "ReRooting"]
impl <F: Foldable> ReRooting<F> {
    pub fn new(n: usize) -> ReRooting<F> {
        ReRooting {
            subdp: vec![F::identity(); n],
            dp: vec![F::identity(); n],
            g: vec![vec![]; n],
        }
    }
    pub fn connect(&mut self, u: usize, v: usize, d: F::T, e: F::T) {
        self.g[u].push(Edge {
            to: v,
            data: d,
            dp: F::identity(),
            ndp: F::identity(),
        });
        self.g[v].push(Edge {
            to: u,
            data: e,
            dp: F::identity(),
            ndp: F::identity(),
        });
    }
    fn dfs_sub(&mut self, u: usize, par: Option<usize>) {
        for i in 0..self.g[u].len() {
            let e = self.g[u][i];
            if Some(e.to) == par { continue; }
            self.dfs_sub(e.to, Some(u));
            self.subdp[u] = F::f(self.subdp[u], F::g(self.subdp[e.to], e.data));
        }
    }
    fn dfs_all(&mut self, u: usize, par: Option<usize>, top: F::Sum) {
        let mut buf = F::identity();
        for i in 0..self.g[u].len() {
            let e = &mut self.g[u][i];
            // 左累積
            e.ndp = buf;
            e.dp = F::g(if Some(e.to) == par { top } else { self.subdp[e.to] }, e.data);
            buf = F::f(buf, e.dp);
        }
        self.dp[u] = buf;
        // 右累積
        buf = F::identity();
        for i in (0..self.g[u].len()).rev() {
            let e = self.g[u][i];
            if Some(e.to) != par {
                self.dfs_all(e.to, Some(u), F::f(e.ndp, buf));
            }
            let e = &mut self.g[u][i];
            e.ndp = F::f(e.ndp, buf);
            buf = F::f(buf, e.dp);
        }
    }
    #[doc = "O(n)"]
    pub fn build(&mut self) -> Vec<F::Sum> {
        self.dfs_sub(0, None);
        self.dfs_all(0, None, F::identity());
        self.dp.clone()
    }
}