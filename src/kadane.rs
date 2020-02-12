#[snippet = "Kadane"]
struct Kadane {
    lmax_table: Vec<(i64,usize)>,
    rmax_table: Vec<(i64,usize)>,
}
#[snippet = "Kadane"]
impl Kadane {
    fn new(a: Vec<i64>) -> Kadane {
        let L = Self::build_lmax(a.clone());
        let mut a = a;
        a.reverse();
        let mut R = Self::build_lmax(a);
        R.reverse();
        Kadane {
            lmax_table: L,
            rmax_table: R,
        }
    }
    fn lmax(&self, r: usize) -> (i64, usize) {
        self.lmax_table[r]
    }
    fn rmax(&self, l: usize) -> (i64, usize) {
        self.rmax_table[l]
    }
    fn build_lmax(a: Vec<i64>) -> Vec<(i64, usize)> {
        let n = a.len();
        let mut res = vec![(0,0)];
        for r in 1..n+1 {
            let (ma, L) = res[r-1];
            let i = r-1;
            let x = a[i];
            let y = ma+a[i];
            let z = 0;
            if z > x && z > y {
                res.push((0,0));
            } else if x > y {
                res.push((x,1));
            } else {
                res.push((y,L+1));
            }
        }
        res
    }
}