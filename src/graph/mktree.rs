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

#[snippet = "SubTree"]
struct SubTree {
    memo: Vec<Option<usize>>,
    g: Vec<Vec<usize>>,
}
#[snippet = "SubTree"]
impl SubTree {
    pub fn new(g: Vec<Vec<usize>>) -> SubTree {
        let n = g.len();
        SubTree {
            memo: vec![None;n],
            g: g,
        }
    }
    fn rec(&mut self, v: usize, pa: Option<usize>) -> usize {
        if let Some(x) = self.memo[v] {
            return x;
        }
        let mut tot = 1;
        for i in 0..self.g[v].len() {
            let u = self.g[v][i];
            if let Some(x) = pa {
                if x == u {
                    continue;
                }
            }
            tot += self.rec(u, Some(v));
        }
        self.memo[v] = Some(tot);
        tot
    }
    pub fn dfs(&mut self, root: usize) {
        self.rec(root, None);
    }
    pub fn get(&self, v: usize) -> usize {
        self.memo[v].unwrap()
    }
}
#[test]
fn test_subtree() {
    let e = vec![
        (0,1),
        (1,2),
        (2,3),
        (0,4),
    ];
    let mut g = vec![vec![];5];
    for (a,b) in e {
        g[a].push(b);
        g[b].push(a);
    }
    let mut st = SubTree::new(g);
    st.dfs(0);
    assert_eq!(st.get(3), 1);
    assert_eq!(st.get(4), 1);
    assert_eq!(st.get(0), 5);
    assert_eq!(st.get(1), 3);
    assert_eq!(st.get(2), 2);
}