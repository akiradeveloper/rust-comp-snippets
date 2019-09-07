#[snippet = "cumsum1"]
fn cumsum1(init: &[i64]) -> Vec<i64> {
    let n = init.len();
    let mut dp = vec![0; n+1];
    let mut acc = 0;
    for i in 0..n {
        acc += init[i];
        dp[i+1] = acc;
    }
    dp
}
#[test]
fn test_cumsum1() {
    let x = vec![0,1,2,1];
    assert_eq!(cumsum1(&x), [0,0,1,3,4]);
}

#[snippet = "cumsum2"]
struct CumSum2 {
    dp: Vec<Vec<i64>>,
}
#[snippet = "cumsum2"]
impl CumSum2 {
    fn build(base: &[Vec<i64>]) -> CumSum2 {
        let n = base.len();
        let m = base[0].len();
        let mut dp = vec![vec![0; m+1]; n+1];
        for i in 0..n {
            for j in 0..m {
                dp[i+1][j+1] = base[i][j];
            }
        }
        for i in 1..n+1 {
            for j in 1..m+1 {
                dp[i][j] += dp[i-1][j] + dp[i][j-1] - dp[i-1][j-1];
            }
        }
        CumSum2 {
            dp: dp,
        }
    }
    fn sum(&self, i: usize, j: usize) -> i64 {
        self.dp[i+1][j+1]
    }
    // [i0,i1),[j0,j1)
    fn query(&self, i0: usize, i1_: usize, j0: usize, j1_: usize) -> i64 {
        self.dp[i1_][j1_] - (self.dp[i0][j1_] + self.dp[i1_][j0] - self.dp[i0][j0])
    }
}
#[test]
fn test_cum2() {
    let x = vec![vec![1,2],vec![3,4]];
    let cum2 = CumSum2::build(&x);
    assert_eq!(cum2.query(0, 2, 0, 2), 10);
    assert_eq!(cum2.query(0, 1, 1, 2), 2);
    assert_eq!(cum2.query(1, 2, 1, 2), 4);
    assert_eq!(cum2.query(0, 1, 0, 2), 3);
}