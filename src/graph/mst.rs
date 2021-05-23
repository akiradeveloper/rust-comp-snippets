/// minimum spanning tree

use cargo_snippet::snippet;

/// プリム法
/// 
/// 最小全域木に含まれる頂点集合Vとその他Wに分類し、
/// 毎回、Vに含まれる頂点から、Wに繋がる辺のうちコスト最小のものを選んで、
/// WからVに頂点を移動していく。
/// 
/// 計算量:
/// O(V^2)

#[snippet("prim")]
pub mod prim {
    pub fn prim(cost: &[Vec<u64>]) -> u64 {
        let n = cost.len();
        let inf = 1<<60;
        let mut mincost = vec![inf; n];
        let mut used = vec![false; n];

        mincost[0] = 0;
        let mut total_cost = 0;

        loop {
            let mut v = None;
            for u in 0 .. n {
                if !used[u] && (v.is_none() || mincost[u] < mincost[v.unwrap()]) {
                    v = Some(u)
                }
            }

            if v.is_none() {
                break;
            }

            let v = v.unwrap();
            used[v] = true;
            total_cost += mincost[v];

            for u in 0 .. n {
                mincost[u] = std::cmp::min(mincost[u], cost[v][u]);
            }
        }
        
        total_cost
    }
}

#[snippet("kraskal")]
mod kraskal {
    use crate::union_find::UnionFind;

    #[derive(Clone,Copy,Debug)]
    pub struct Edge {
        pub u: usize,
        pub v: usize,
        pub cost: i64
    }

    #[doc = "es: undirected edges. O(ElogV)"]
    pub fn kraskal(n: usize, es: Vec<Edge>) -> (Vec<Edge>, Vec<Edge>) {
        let mut used = vec![];
        let mut unused = vec![];

        let mut es = es;
        es.sort_by_key(|x| x.cost);

        let mut uf = UnionFind::new(n);

        for e in es {
            if !uf.same(e.u, e.v) {
                uf.merge(e.u, e.v);
                used.push(e);
            } else {
                unused.push(e);
            }
        }

        (used, unused)
    }
}

#[snippet("chu_liu_edmonds")]
mod chu_liu_edmonds {
    use crate::graph::scc::SCC;

    #[derive(Debug,Clone,Copy)]
    pub struct Edge(pub usize, pub u64);

    fn min_edge(edges: &[Edge]) -> &Edge {
        let mut r = &edges[0];
        for e in edges {
            if e.1 < r.1 {
                r = e;
            }
        }
        r
    }
    static NULL_EDGE: &'static Edge = &Edge(1<<40, 0);
    pub fn chu_liu_edmonds(in_g: &[Vec<Edge>], root: usize) -> u64 {
        // dbg!(&in_g);
        let n = in_g.len();
        let mut min_in_g: Vec<&Edge> = vec![];
        let mut min_out_g: Vec<Vec<usize>> = vec![vec![]; n];
        for to in 0..n {
            if to == root {
                min_in_g.push(NULL_EDGE);
                continue;
            }
            let e = min_edge(&in_g[to]);
            min_in_g.push(e);
            min_out_g[e.0].push(to);
        }

        let mut scc = SCC::new(n);
        for u in 0..n {
            for i in 0..min_out_g[u].len() {
                let v = min_out_g[u][i];
                scc.add_edge(u, v);
            }
        }
        scc.build();

        // dbg!(&min_in_g);
        // dbg!(&min_out_g);
        // dbg!(&scc.order);

        let mut max_cmp = 0;
        for &cmp in &scc.order {
            if cmp > max_cmp {
                max_cmp = cmp;
            }
        }

        let no_loop = max_cmp == scc.order.len()-1;
        if no_loop {
            let mut res = 0;
            for e in &min_in_g {
                res += e.1;
            }
            // dbg!(res);
            return res;
        }

        let mut groups = vec![vec![]; max_cmp+1];
        for v in 0..scc.order.len() {
            let cmp = scc.order[v];
            groups[cmp].push(v);
        }
        // dbg!(&groups);

        let mut contracted_cost = 0;
        let mut new_in_g = vec![vec![]; max_cmp+1];
        for group in groups {
            if group.len() > 1 { // loop
                let cmp_to = scc.order[group[0]];
                for &v in &group {
                    let cur_e = min_in_g[v];

                    contracted_cost += cur_e.1;

                    for e in &in_g[v] {
                        let in_group = group.contains(&e.0);
                        if !in_group {
                            let cmp_from = scc.order[e.0];
                            // dbg!((v, e.1, cur_e.1));
                            let diff_cost = e.1 - cur_e.1;
                            new_in_g[cmp_to].push(Edge(cmp_from, diff_cost));
                        }
                    }
                }
            } else {
                assert!(group.len() == 1);
                let v = group[0];
                for e in &in_g[v] {
                    let cmp_to = scc.order[v];
                    let cmp_from = scc.order[e.0];
                    new_in_g[cmp_to].push(Edge(cmp_from, e.1));
                }
            }
        }

        let new_root = scc.order[root];

        contracted_cost + chu_liu_edmonds(&new_in_g, new_root)
    }
}
#[test]
fn test_chu_liu_edmonds_0() {
    use chu_liu_edmonds::*;
    let in_g = vec![
        vec![],
        vec![Edge(0, 5)],
        vec![Edge(0, 6)],
        vec![Edge(1, 5), Edge(2, 1)],
    ];
    assert_eq!(chu_liu_edmonds(&in_g, 0), 12);
}

#[test]
fn test_chu_liu_edmonds_1() {
    use chu_liu_edmonds::*;
    let in_g = vec![
        vec![],
        vec![Edge(0,2),Edge(4,3)],
        vec![Edge(0,4),Edge(1,5),Edge(4,3)],
        vec![Edge(1,3),Edge(4,4)],
        vec![Edge(5,7),Edge(7,5)],
        vec![Edge(2,6)],
        vec![Edge(3,5),Edge(4,4)],
        vec![Edge(5,1),Edge(6,3)],
        vec![Edge(5,4),Edge(7,6)],
    ];
    assert_eq!(chu_liu_edmonds(&in_g, 0), 29);
}