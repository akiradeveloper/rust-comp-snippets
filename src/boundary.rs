#[snippet = "adj4"]
pub fn adj4(i: usize, j: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut r = vec![];
    if i>0 {
        r.push((i-1,j));
    }
    if i+1<h {
        r.push((i+1,j));
    }
    if j>0 {
        r.push((i,j-1));
    }
    if j+1<w {
        r.push((i,j+1));
    }
    r
}

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