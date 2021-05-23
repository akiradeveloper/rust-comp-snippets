use cargo_snippet::snippet;
use std::collections::HashSet;

/// 有向グラフに対してサイクルの存在判定を行う。
/// 
/// アイデア:
/// サイクルの周りにある辺を全部カットする。
/// 入次数0の頂点から辿っていき、その都度辺をカットしていく。
/// 最終的に、入次数が0にならないということは、その頂点がサイクルの一部だということである。
/// 
/// 計算量: O(E)
/// 最悪ケースで全辺を辿るため

// verified: GRL_4_A
#[snippet("CycleDetection")]
fn cycle_detection_directed(g: &[Vec<usize>]) -> bool {
    let n = g.len();
    let mut in_g = vec![HashSet::new();n];
    for v in 0..n {
        for &u in &g[v] {
            in_g[u].insert(v);
        }
    }
    let mut v_indegree0 = vec![];
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
#[snippet("CycleDetection")]
struct CycleDetection {
    g: Vec<Vec<usize>>,
}
#[snippet("CycleDetection")]
impl CycleDetection {
    pub fn new(n: usize) -> CycleDetection {
        CycleDetection {
            g: vec![vec![];n]
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
    }
    pub fn solve(&self) -> bool {
        cycle_detection_directed(&self.g)
    }
}

#[test]
fn test_detect_cycle_directed_0() {
    let e = vec![
        vec![1,2],
        vec![2],
        vec![],
    ];
    let mut g = CycleDetection::new(3);
    for u in 0..e.len() {
        let vs = e[u].clone();
        for v in vs {
            g.add_edge(u, v);
        }
    }
    assert_eq!(g.solve(), false);
}

#[test]
fn test_detect_cycle_directed_1() {
    let e = vec![
        vec![1,2],
        vec![2],
        vec![],
        vec![0,4],
        vec![5],
        vec![3],
    ];
    let mut g = CycleDetection::new(6);
    for u in 0..e.len() {
        let vs = e[u].clone();
        for v in vs {
            g.add_edge(u,v);
        }
    }
    assert_eq!(g.solve(), true);
}