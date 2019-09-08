#[doc = "build_depth_table from adjacent graph. O(V)"]
#[snippet = "build_depth_table"]
fn build_depth_table(n: usize, root: usize, g: &[Vec<usize>]) -> Vec<usize> {
    let inval=n+1;
    let mut res = vec![n+1;n];
    let mut stack = vec![root];
    res[root]=0;
    while !stack.is_empty() {
        let mut r = stack.pop().unwrap();
        let cs = &g[r];
        for &c in cs {
            res[c]=res[r]+1;
            stack.push(c);
        }
    }
    res
}