use std::collections::HashSet;

// verified: GRL_4_A
#[doc = "directed. O(E)"]
#[snippet = "cycle_detection_directed"]
pub fn cycle_detection_directed(g: &[Vec<usize>]) -> bool {
    let n = g.len();
    let mut in_g = vec![HashSet::new();n];
    // O(E)
    for v in 0..n {
        for &u in &g[v] {
            in_g[u].insert(v);
        }
    }
    let mut v_indegree0 = vec![];
    // O(V)
    for v in 0..n {
        if in_g[v].len() == 0 {
            v_indegree0.push(v);
        }
    }

    let mut m=0;
    while let Some(v) = v_indegree0.pop() {
        m += 1;
        for &to in &g[v] {
            if in_g[to].remove(&v) && in_g[to].is_empty() {
                v_indegree0.push(to);
            }
        }
    }

    m != n
}

#[test]
fn test_detect_cycle_directed_0() {
    let mut g = vec![
        vec![1,2],
        vec![2],
        vec![],
    ];
    assert_eq!(cycle_detection_directed(&g), false);
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
    assert_eq!(cycle_detection_directed(&g), true);
}