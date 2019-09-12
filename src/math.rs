/// https://github.com/hatoo/competitive-rust-snippets

#[allow(dead_code)]
/// v[n][k] = nCk / 2^n
fn nck_prob(n: usize) -> Vec<Vec<f64>> {
    let mut res = vec![vec![1.0]];

    for _ in 1..n {
        let mut v = Vec::new();
        {
            let last = res.last().unwrap();
            v.push(last.first().unwrap() / 2.0);
            for i in 0..last.len() - 1 {
                v.push((last[i] + last[i + 1]) / 2.0);
            }
            v.push(last.last().unwrap() / 2.0);
        }
        res.push(v);
    }
    res
}

#[snippet = "convex_hull_check"]
#[allow(dead_code)]
/// A check function for convex hull trick
pub fn convex_hull_check((a1, b1): (i64, i64), (a2, b2): (i64, i64), (a3, b3): (i64, i64)) -> bool {
    // Convert to f64 due to overflow
    (a2 as f64 - a1 as f64) * (b3 as f64 - b2 as f64)
        >= (b2 as f64 - b1 as f64) * (a3 as f64 - a2 as f64)
}