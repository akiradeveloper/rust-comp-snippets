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