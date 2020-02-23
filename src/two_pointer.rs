#[snippet = "TwoPointer"]
struct TwoPointer<F> {
    n: usize,
    f: F,
}
#[snippet = "TwoPointer"]
impl <F: FnMut(usize,usize) -> bool> TwoPointer<F> {
    pub fn new(n: usize, f: F) -> TwoPointer<F> {
        TwoPointer {
            n: n,
            f: f,
        }
    }
    pub fn run(&mut self) {
        let mut l = 0;
        let mut r = 0;
        loop {
            let mut ok = (self.f)(l,r);
            if r==self.n {
                ok = false;
            }
            if !ok {
                l += 1;
            } else {
                r += 1;
            }
            if l==self.n && r==self.n {
                break;
            }
        }
    }
}

#[test]
fn test_two_pointer() {
    let a = vec![4,6,7,8,1,2,110,2,4,12,3,9];
    let n = a.len();
    let mut cum = vec![0];
    for i in 0..n {
        cum.push(cum[i] + a[i]);
    }
    let mut maxv = 0;
    let mut f = |i:usize, j:usize| {
        let sum = cum[j] - cum[i];
        if sum <= 27 {
            maxv = std::cmp::max(maxv, sum);
            true
        } else {
            false
        }
    };
    let mut tp = TwoPointer::new(12, f);
    tp.run();
    assert_eq!(maxv, 26)
}