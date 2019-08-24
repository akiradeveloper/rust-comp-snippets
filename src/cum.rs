fn cum1(init: &[i64]) -> Vec<i64> {
    let n = init.len();
    let mut dp = vec![0; n];
    let mut acc = 0;
    for i in 0..n {
        acc += init[i];
        dp[i] = acc;
    }
    dp
}
#[test]
fn test_cum1() {
    let x = vec![0,1,2,1];
    assert_eq!(cum1(&x), [0,1,3,4]);
}

struct Cum2 {
    dp: Vec<Vec<i64>>,
}
impl Cum2 {
    fn build(base: &[Vec<i64>]) -> Cum2 {
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
        Cum2 {
            dp: dp,
        }
    }
    fn sum(&self, i: usize, j: usize) -> i64 {
        self.dp[i+1][j+1]
    }
    // [i0,i1],[j0,j1]
    fn query(&self, i0: usize, i1: usize, j0: usize, j1: usize) -> i64 {
        let i0 = i0+1;
        let i1 = i1+1;
        let j0 = j0+1;
        let j1 = j1+1;
        self.dp[i1][j1] - (self.dp[i0-1][j1] + self.dp[i1][j0-1] - self.dp[i0-1][j0-1])
    }
}
#[test]
fn test_cum2() {
    let x = vec![vec![1,2],vec![3,4]];
    let cum2 = Cum2::build(&x);
    assert_eq!(cum2.query(0, 1, 0, 1), 10);
    assert_eq!(cum2.query(0, 0, 1, 1), 2);
    assert_eq!(cum2.query(1, 1, 1, 1), 4);
    assert_eq!(cum2.query(0, 0, 0, 1), 3);
}