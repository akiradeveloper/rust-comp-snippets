use cargo_snippet::snippet;

#[snippet("AdjacencyMatrix")]
struct AdjacencyMatrix {
    a: Vec<Vec<i64>>,
}
#[snippet("AdjacencyMatrix")]
impl AdjacencyMatrix {
    pub fn new(n: usize, inf: i64) -> AdjacencyMatrix {
        AdjacencyMatrix {
            a: vec![vec![inf;n];n]
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, cost: i64) {
        self.a[u][v] = cost;
    }
    pub fn build(&self) -> Vec<Vec<i64>> {
        self.a.clone()
    }
}