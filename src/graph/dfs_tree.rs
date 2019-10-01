#[doc = "dfs on adjacent graph and analyze the connectivity. O(V)"]
#[snippet = "dfs_tree"]
fn dfs_tree(n: usize, root: usize, g: &[Vec<usize>]) -> (Vec<usize>, Vec<Option<usize>>) {
    let mut depth = vec![n;n];
    let mut stack = vec![root];
    let mut visited = vec![false;n];
    let mut par = vec![None;n];
    depth[root]=0;
    par[root]=None;
    visited[root]=true;
    while !stack.is_empty() {
        let r = stack.pop().unwrap();
        let cs = &g[r];
        for &c in cs {
            if visited[c] { continue; }
            visited[c]=true;
            par[c]=Some(r);
            depth[c]=depth[r]+1;
            stack.push(c);
        }
    }
    (depth,par)
}

#[test]
fn test_dfs_tree() {
    let mut g = vec![vec![];5];
    let conn = vec![
        (0,1),(0,2),(1,3),(1,4),
    ];
    for (x,y) in conn {
        g[x].push(y);
        g[y].push(x);
    }
    dbg!(dfs_tree(5, 0, &g));
    dbg!(dfs_tree(5, 1, &g));
}