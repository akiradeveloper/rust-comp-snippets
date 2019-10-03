#[doc = "traverse on an adjacent graph and analyze the connectivity. O(V)"]
#[snippet = "analyze_tree"]
fn analyze_tree(g: &[Vec<usize>], root: usize) -> (Vec<usize>, Vec<Option<usize>>) {
    let n = g.len();
    let mut depth = vec![0;n];
    let mut par = vec![None;n];
    dfs(g, root, &mut |conn, v| {
        if let Some((r,i)) = conn {
            par[v]=Some(r);
            depth[v]=depth[r]+1;
        }
    });
    (depth,par)
}

#[test]
fn test_analyze_tree() {
    let mut g = vec![vec![];5];
    let conn = vec![
        (0,1),(0,2),(1,3),(1,4),
    ];
    for (x,y) in conn {
        g[x].push(y);
        g[y].push(x);
    }
    dbg!(analyze_tree(&g, 0));
    dbg!(analyze_tree(&g, 1));
}

#[snippet = "bfs"]
fn bfs<F: FnMut(Option<(usize,usize)>,usize)>(g: &[Vec<usize>], root: usize, op: &mut F) {
    let n = g.len();
    let mut q = std::collections::VecDeque::new();
    q.push_back(root);
    let mut visited = vec![false; n];
    visited[root]=true;
    op(None,root);
    while !q.is_empty() {
        let r = q.pop_front().unwrap();
        let cs = &g[r];
        for i in 0..cs.len() {
            let c = cs[i];
            if visited[c] { continue; }
            visited[c]=true;
            op(Some((r,i)),c); 
            q.push_back(c);
        }
    }
}

#[snippet = "dfs"]
fn dfs<F: FnMut(Option<(usize, usize)>,usize)>(g: &[Vec<usize>], root: usize, op: &mut F) {
    let n = g.len();
    let mut stack = vec![(None,root)];
    let mut visited = vec![false;n];
    visited[root]=true;
    while !stack.is_empty() {
        let (conn,r) = stack.pop().unwrap();
        op(conn,r);
        let cs = &g[r];
        for i in 0..cs.len() {
            let c = cs[i];
            if visited[c] { continue; }
            visited[c]=true;
            stack.push((Some((r,i)),c));
        }
    }
}

#[test]
fn test_tree_bfs() {
    let mut g = vec![vec![];5];
    let conn = vec![
        (0,1),(0,2),(2,3),(2,4),
    ];
    for (x,y) in conn {
        g[x].push(y);
        g[y].push(x);
    }
    dbg!(bfs(&g, 0, &mut |conn, v| { println!("{:?},{}",conn,v)}));
}

#[test]
fn test_tree_dfs() {
    let mut g = vec![vec![];5];
    let conn = vec![
        (0,1),(0,2),(2,3),(2,4),
    ];
    for (x,y) in conn {
        g[x].push(y);
        g[y].push(x);
    }
    dbg!(dfs(&g, 0, &mut |conn, v| { println!("{:?},{}",conn,v)}));
}