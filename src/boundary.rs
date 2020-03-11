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
#[snippet = "valid4u"]
pub fn valid4u(h: usize, w: usize, ps: Vec<(Option<usize>, Option<usize>)>) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for (x, y) in ps {
        if x.is_none() || y.is_none() {
            continue;
        }
        let x = x.unwrap();
        let y = y.unwrap();
        if x >= h || y >= w {
            res.push((x,y));
        }
    }
    res
}
#[snippet = "incl"]
#[doc = "0..=n in old compilers"]
pub fn incl(n: usize) -> usize {
    n+1
}