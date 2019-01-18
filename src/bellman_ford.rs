struct Edge {
    from: usize,
    to: usize,
    cost: i32,
}

fn bellman_ford(n: usize, es: Vec<Edge>, source: usize) -> Vec<i32> {
    const INF: i32 = 2_000_000_001;
    let mut d = vec![INF; n];
    d[source] = 0;
    loop {
        let mut update = false;
        for e in &es {
            if d[e.from] != INF && d[e.to] > d[e.from] + e.cost {
                d[e.to] = d[e.from] + e.cost;
                update = true;
            }
        }
        if !update {
            break;
        }
    }
    d
}

#[test]
fn test_bellman_ford() {}

fn find_negative_loop(n: usize, es: Vec<Edge>) -> bool {
    let mut d = vec![0; n];
    for i in 0..n {
        for e in &es {
            if d[e.to] > d[e.from] + e.cost {
                d[e.to] = d[e.from] + e.cost;
                if i == n - 1 {
                    return true;
                }
            }
        }
    }
    false
}

#[test]
fn test_find_negative_loop() {}
