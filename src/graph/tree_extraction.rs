#[snippet = "TreeExtraction"]
pub struct TreeExtraction {
    parent: Vec<Option<usize>>,
    is_leaf: Vec<bool>,
    depth: Vec<usize>,
}
#[doc = "Traverse a directed graph by dfs and extract a tree. O(N)"]
#[snippet = "TreeTraversal"]
impl TreeExtraction {
    #[doc = "directed / undirected"]
    pub fn new(g: &[Vec<usize>], root: usize) -> TreeExtraction {
        Self::build(g, root)
    }
    pub fn build(g: &[Vec<usize>], root: usize) -> TreeExtraction {
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
        TreeExtraction {
            depth: dp,
            is_leaf: is_leaf,
            parent: par,
        }
    }
}