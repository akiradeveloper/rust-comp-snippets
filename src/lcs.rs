#[derive(Debug)]
struct LCS<T> {
    max_matching: Vec<Vec<usize>>,
    matching: Vec<(usize,usize)>,
    phantom: std::marker::PhantomData<T>,
}
impl <T: Eq> LCS<T> {
    pub fn new(xs: &[T], ys: &[T]) -> LCS<T> {
        let n = xs.len();
        let m = ys.len();
        let dp = Self::lcs(xs, ys);
        let k = dp[n-1][m-1];
        let mut matching = vec![(0,0);k];
        let mut cur = 0;
        for i in 0..xs.len() {
            for j in 0..ys.len() {
                if dp[i][j] > cur {
                    matching[cur] = (i,j);
                    cur += 1;
                }
            }
        }
        let mut tmp = vec![vec![0;m+1];n+1];
        for i in 0..n {
            for j in 0..m {
                tmp[i+1][j+1] = dp[i][j];
            }
        }
        LCS {
            max_matching: tmp,
            matching: matching,
            phantom: std::marker::PhantomData,
        }
    }
    fn lcs(xs: &[T], ys: &[T]) -> Vec<Vec<usize>> {
        let n = xs.len();
        let m = ys.len();
        let mut dp = vec![vec![0; m]; n];
        for i in 0..n {
            let x = &xs[i];
            let mut found = false;
            for j in 0..m {
                if &ys[j] == x {
                    found = true;
                }
                dp[i][j] = if found {
                    1
                } else { 0 };
            }
        }
        for j in 0..m {
            let y = &ys[j];
            let mut found = false;
            for i in 0..n {
                if &xs[i] == y {
                    found = true;
                }
                dp[i][j] = if found {
                    1
                } else { 0 };
            }
        }
        for i in 1..n {
            for j in 1..m {
                dp[i][j] = if xs[i] == ys[j] {
                    dp[i-1][j-1] + 1
                } else {
                    std::cmp::max(dp[i][j-1], dp[i-1][j])
                };
            }
        }
        return dp;
    }
}

#[test]
fn test_lcs() {
    let xs = vec![6,9,2,8,3,7,4,6];
    let ys = vec![1,2,2,4,5,3,3,3,2,4];
    let lcs = LCS::new(&xs, &ys);
    dbg!(lcs);
}