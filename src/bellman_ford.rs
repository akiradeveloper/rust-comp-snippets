struct Edge {
    from: usize,
    to: usize,
    cost: i32,
}

fn bellman_ford(V: usize, es: Vec<Edge>, source: usize) -> Vec<i32> {
    let INF = 2_000_000_001;
    let mut d = vec![INF; V];
    d[source] = 0;
    loop {
        let mut update = false;
        for e in &es {
            if d[e.from] != INF && d[e.to] > d[e.from] + e.cost {
                d[e.to] = d[e.from] + e.cost;
                update = true;
            }
        }
        if !update { break; }
    }
    d
}

fn find_negative_loop(V: usize, es: Vec<Edge>) -> bool {
    let mut d = vec![0; V];
    for i in 0 .. V {
        for e in &es {
            if d[e.to] > d[e.from] + e.cost { 
                d[e.to] = d[e.from] + e.cost;
                if (i == V-1) {
                    return true;
                }
            }
        }
    }
    false
}