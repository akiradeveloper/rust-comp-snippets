#[snippet = "LongestDistance"]
struct LongestDistance {
    ing: Vec<Vec<usize>>,
    dp: Vec<Option<usize>>,
}
#[snippet = "LongestDistance"]
impl LongestDistance {
    #[doc = "DAG"]
    fn new(ing: Vec<Vec<usize>>) -> LongestDistance {
        let n = ing.len();
        LongestDistance {
            ing: ing,
            dp: vec![None; n],
        }
    }
    fn solve(&mut self, i: usize) -> usize {
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