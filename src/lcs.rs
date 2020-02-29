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
        let mut k = dp[n][m];
        let mut i = n;
        let mut j = m;
        let mut matching = vec![];
        while k>0 {
            while dp[i-1][j] == k {
                i -= 1;
            }
            while dp[i][j-1] == k {
                j -= 1;
            }
            matching.push((i-1,j-1));
            k -= 1;
            i -= 1;
            j -= 1;
        }
        matching.reverse();
        
        LCS {
            max_matching: dp,
            matching: matching,
            phantom: std::marker::PhantomData,
        }
    }
    fn lcs(xs: &[T], ys: &[T]) -> Vec<Vec<usize>> {
        let n = xs.len();
        let m = ys.len();
        let mut dp = vec![vec![0;m+1];n+1];
        dp[0][0] = 0;
        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = if xs[i-1] == ys[j-1] {
                    dp[i-1][j-1] + 1
                } else {
                    std::cmp::max(dp[i][j-1], dp[i-1][j])
                }
            }
        }
        dp
    }
}

#[test]
fn test_lcs0() {
    let xs = vec![6,9,2,8,3,7,4,6];
    let ys = vec![1,2,2,4,5,3,3,3,2,4];
    let lcs = LCS::new(&xs, &ys);
    assert_eq!(lcs.matching, vec![(2,1),(4,5),(6,9)]);
}
#[test]
fn test_lcs1() {
    let xs = vec![1,0,1,1,1,1,0,0];
    let ys = vec![1,0,0,1,1,1,1,1,0];
    let lcs = LCS::new(&xs, &ys);
    assert_eq!(lcs.matching.len(), 7);
    assert_eq!(lcs.matching, vec![(0,0),(1,1),(2,3),(3,4),(4,5),(5,6),(6,8)]);
}