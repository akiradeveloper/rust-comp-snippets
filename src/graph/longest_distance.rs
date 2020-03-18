use cargo_snippet::snippet;

#[snippet("LongestDistance")]
struct LongestDistance {
    ing: Vec<Vec<usize>>,
    dp: Vec<Option<usize>>,
}
#[snippet("LongestDistance")]
impl LongestDistance {
    #[doc = "DAG"]
    pub fn new(n: usize) -> LongestDistance {
        LongestDistance {
            ing: vec![vec![]; n],
            dp: vec![None; n],
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.ing[v].push(u)
    }
    pub fn solve(&mut self, i: usize) -> usize {
        if self.ing[i].len() == 0 {
            return 0
        }
        if let Some(x) = self.dp[i] {
            return x;
        }
        let mut maxv = 0;
        for j in 0..self.ing[i].len() {
            let from = self.ing[i][j];
            maxv = std::cmp::max(maxv, self.solve(from) + 1);
        }
        self.dp[i] = Some(maxv);
        return maxv;
    }
}
#[test]
fn test_longest_distance() {
    let mut g = LongestDistance::new(8);
    let e = vec![
        (0,1),(2,0),(1,3),(1,4),(5,2),(4,6),(4,7),
    ];
    for (u,v) in e {
        g.add_edge(u, v);
    }
    assert_eq!(g.solve(7), 5);
    assert_eq!(g.solve(2), 1);
    assert_eq!(g.solve(5), 0);
    assert_eq!(g.solve(3), 4);
}