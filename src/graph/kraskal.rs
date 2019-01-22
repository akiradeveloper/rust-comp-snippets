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