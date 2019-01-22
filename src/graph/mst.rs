/// minimum spanning tree

mod prim {
    /// O(V^2)
fn prim(cost: &[Vec<u32>]) -> u32 {
    let n = cost.len();
    const INF: u32 = 2_000_000_1;
    let mut mincost = vec![INF; n];
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

mod kraskal {
    use crate::union_find;

struct Edge {
    u: usize,
    v: usize,
    cost: u32
}

/// O(E logV)
fn kraskal(n: usize, es: &mut [Edge]) -> u32 {
    es.sort_by(|a, b| {
        a.cost.cmp(&b.cost)
    });

    let mut uf = union_find::DisjointSet::new(n);

    let mut total_cost = 0;

    for e in es {
        if !uf.same(e.u, e.v) {
            uf.unite(e.u, e.v);
            total_cost += e.cost;
        }
    }

    total_cost
}
}