use cargo_snippet::snippet;

#[snippet("AdjacencyMatrix")]
struct AdjacencyMatrix {
    a: Vec<Vec<i64>>,
}
#[snippet("AdjacencyMatrix")]
impl AdjacencyMatrix {
    pub fn new(n: usize, inf: i64) -> Self {
        let mut a = vec![vec![inf;n];n];
        for i in 0..n {
            a[i][i] = 0;
        }
        AdjacencyMatrix {
            a: a,
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, cost: i64) {
        self.a[u][v] = cost;
    }
    pub fn build(&self) -> Vec<Vec<i64>> {
        self.a.clone()
    }
}