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