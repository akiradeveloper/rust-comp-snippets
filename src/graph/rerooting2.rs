trait Foldble {
    type T: Copy + std::fmt::Debug;
    type Sum: Copy + std::fmt::Debug;
    fn identity() -> Self::Sum;
    fn f(x: Self::Sum, y: Self::Sum) -> Self::Sum;
}
struct Edge<F: Foldable> {
    to: usize,
    subdp: F::Sum,
    rootdp: F::Sum,
    data: F::T,
}
struct ReRooting2<F: Foldable> {
    g: Vec<Vec<Edge>>,
    subldp: Vec<Vec<F::Sum>>,
    subrdp: Vec<Vec<F::Sum>>,
    rootldp: Vec<Vec<F::Sum>>,
    rootrdp: Vec<Vec<F::Sum>>,
}
impl <F> ReRooting2<F: Foldable> {
    pub fn new(n: usize) -> ReRooting<F> {
        ReRooting {
            g: vec![vec![]; n],
            subldp: vec![vec![]; n],
            subrdp: vec![vec![]; n],
            rootldp: vec![vec![]; n],
            rootrdp: vec![vec![]; n],
        }
    }
    pub fn connect(&mut self, u: usize, v: usize, d: F::T, e: F::T) {
        self.g[u].push(Edge {
            to: v,
            data: d,
            subdp: F::identity(),
            rootdp: F::identity(),
        });
        self.g[v].push(Edge {
            to: u,
            data: e,
            subdp: F::identity(),
            rootdp: F::identity(),
        });
    }
}