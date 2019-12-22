#[doc = "O(N)"]
#[snippet = "tree_depth_table"]
pub fn tree_depth(g: &[Vec<usize>], root: usize) -> (Vec<i64>, Vec<bool>) {
    let n = g.len();
    let mut dp = vec![0; n];
    let mut is_leaf = vec![false; n];
    let mut S = vec![];
    let mut visited = vec![false; n];
    visited[root] = true;
    S.push(root);
    while !S.is_empty() {
        let v = S.pop().unwrap();
        let mut found = false;
        for i in 0..g[v].len() {
            let u = g[v][i];
            if !visited[u] {
                dp[u] = dp[v] + 1;
                visited[u] = true;
                S.push(u);
                found = true;
            }
        }
        if !found {
            is_leaf[v] = true;
        }
    }
    (dp, is_leaf)
}