#[snippet = "mktree"]
pub struct Tree {
    parent: Vec<Option<usize>>,
    is_leaf: Vec<bool>,
    depth: Vec<usize>,
}
#[doc = "O(N)"]
#[snippet = "mktree"]
pub fn mktree(g: &[Vec<usize>], root: usize) -> Tree {
    let n = g.len();
    let mut par = vec![None; n];
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
                par[u] = Some(v);
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
    Tree {
        depth: dp,
        is_leaf: is_leaf,
        parent: par,
    }
}