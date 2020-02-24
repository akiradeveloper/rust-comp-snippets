#[snippet = "valid4"]
pub fn valid4(h: usize, w: usize, ps: Vec<(i64,i64)>) -> Vec<(usize,usize)> {
    let mut res = vec![];
    for (i,j) in ps {
        if 0<=i && i<h as i64 && 0<=j && j<w as i64 {
            res.push((i as usize, j as usize))
        }
    }
    res
}