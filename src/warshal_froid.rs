fn warshal_froid(n: usize, d: &mut [Vec<i32>]) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = std::cmp::min(d[i][j], d[i][k] + d[k][j]);
            }
        }
    }
}
