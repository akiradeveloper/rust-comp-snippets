use std::collections::HashSet;
pub fn detect_cycle_directed(g: &[Vec<usize>]) -> bool {
    let mut white_set = HashSet::new();
    let mut gray_set = HashSet::new();
    let mut black_set = HashSet::new();

    for v in 0..g.len() {
        white_set.insert(v);
    }
    while !white_set.is_empty() {
        while let Some(cur) = white_set.iter().cloned().next() {
            if dfs(g, cur, &mut white_set, &mut gray_set, &mut black_set) {
                return true
            }
        }
    }

    return false;

    fn dfs(g: &[Vec<usize>], cur: usize, white_set: &mut HashSet<usize>, gray_set: &mut HashSet<usize>, black_set: &mut HashSet<usize>) -> bool {
        moveVertex(cur, white_set, gray_set);
        for neighbour in g[cur].iter().cloned() {
            if black_set.contains(&neighbour) {
                continue;
            }
            if gray_set.contains(&neighbour) {
                return true
            }
            if dfs(g, neighbour, white_set, gray_set, black_set) {
                return true
            }
        }
        moveVertex(cur, gray_set, black_set);
        false
    }

    fn moveVertex(v: usize, from: &mut HashSet<usize>, to: &mut HashSet<usize>) {
        from.remove(&v);
        to.insert(v);
    }
} 
#[test]
fn test_detect_cycle_directed_0() {
    let mut g = vec![
        vec![1,2],
        vec![2],
        vec![],
    ];
    assert_eq!(detect_cycle_directed(&g), false);
}
#[test]
fn test_detect_cycle_directed_1() {
    let mut g = vec![
        vec![1,2],
        vec![2],
        vec![],
        vec![0,4],
        vec![5],
        vec![3],
    ];
    assert_eq!(detect_cycle_directed(&g), true);
}