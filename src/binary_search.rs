struct BinarySearch<F> {
    lb: f64,
    ub: f64,
    f: F,
}
impl <F: Fn(f64) -> bool> BinarySearch<F> {
    const eps: f64 = 1e-9;
    fn new(lb: f64, ub: f64, f: F) -> BinarySearch<F> {
        BinarySearch {
            lb: lb,
            ub: ub,
            f: f,
        }
    }
    fn search(&self) -> f64 {
        let mut lb = self.lb;
        let mut ub = self.ub;
        while ub > lb + Self::eps {
            let mid = (lb+ub)/2.0;
            if (self.f)(mid) {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        lb
    }
}
