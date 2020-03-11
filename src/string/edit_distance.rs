use std::cmp::min;

#[snippet = "edit_distance"]
pub fn edit_distance<T: Eq>(s: Vec<T>, t: Vec<T>) -> Vec<Vec<usize>> {
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![1<<30; m+1]; n+1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..m {
            let change = if s[i] == t[j] {
                dp[i][j]
            } else {
                dp[i][j] + 1
            };
            let delete = dp[i][j+1] + 1;
            let insert = dp[i+1][j] + 1;

            dp[i+1][j+1] = min(change, min(delete, insert));
        }
    }
    dp
}

#[test]
fn test_edit_distance() {
    let s = vec!['a','c','a','c'];
    let t = vec!['a','c','m'];
    let dp = edit_distance(s, t);
    dbg!(&dp);
    assert_eq!(dp[4][3], 2);
}