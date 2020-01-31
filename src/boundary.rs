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
