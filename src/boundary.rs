use cargo_snippet::snippet;

#[snippet("Boundary")]
struct Boundary {
    l: i64,
    r: i64,
}
#[snippet("Boundary")]
impl Boundary {
    pub fn new(n: usize) -> Self {
        Self {
            l: 0,
            r: n as i64, 
        }
    }
    pub fn add(&self, x: usize, diff: i64) -> Option<usize> {
        let x = x as i64;
        let y = x + diff;
        if y > self.r-1 {
            None
        } else if y < self.l {
            None
        } else {
            Some(y as usize)
        }
    }
}
#[snippet("Boundary")]
struct Boundary2d {
    h: Boundary,
    w: Boundary,
}
#[snippet("Boundary")]
impl Boundary2d {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h: Boundary::new(h),
            w: Boundary::new(w),
        }
    }
    pub fn add(&self, p: (usize,usize), diff: (i64,i64)) -> Option<(usize,usize)> {
        let x = self.h.add(p.0, diff.0);
        let y = self.w.add(p.1, diff.1);
        match (x, y) {
            (Some(a), Some(b)) => Some((a,b)),
            _ => None,
        }
    }
}