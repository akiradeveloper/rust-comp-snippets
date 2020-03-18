use cargo_snippet::snippet;

#[snippet("BinarySearchf64")]
struct BinarySearchf64<F> {
    lb: f64,
    ub: f64,
    f: F,
}
#[snippet("BinarySearchf64")]
impl <F: FnMut(f64) -> bool> BinarySearchf64<F> {
    const eps: f64 = 1e-9;
    fn new(lb: f64, ub: f64, f: F) -> BinarySearchf64<F> {
        BinarySearchf64 {
            lb: lb,
            ub: ub,
            f: f,
        }
    }
    fn search(&mut self) -> f64 {
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

#[snippet("BinarySearch")]
#[doc = "lower,upper are inclusive range"]
pub struct BinarySearch<F> {
    pub p: F,
    pub lower: i64,
    pub upper: i64,
}
#[snippet("BinarySearch")]
impl <F: FnMut(i64) -> bool> BinarySearch<F> {
    #[doc = "O(log(upper-lower))"]
    pub fn lower_bound(&mut self) -> i64 {
        let lower = self.lower;
        let upper = self.upper;
        assert!(lower<=upper);

        let mut lb = lower - 1; 
        let mut ub = upper + 1;
        while ub - lb > 1 {
            let mid = (lb+ub)/2;
            let ok = (self.p)(mid);
            if ok {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        ub
    }
}

#[test]
fn test_binary_search() {
    let xs = vec![1,2,2,2,2,2,3,4,5];
    let p0 = |i: i64| { xs[i as usize] >= 2 };
    let mut bs0 = BinarySearch {
        p: p0,
        lower: 0,
        upper: xs.len() as i64 - 1,
    };
    assert_eq!(bs0.lower_bound(), 1);

    let p1 = |i: i64| { xs[i as usize] > 2 };
    let mut bs1 = BinarySearch {
        p: p1,
        lower: 0,
        upper: xs.len() as i64 - 1,
    };
    assert_eq!(bs1.lower_bound(), 6);

    let p2 = |i: i64| { xs[i as usize] >= 0 };
    let mut bs2 = BinarySearch {
        p: p2,
        lower: 0,
        upper: xs.len() as i64 - 1,
    };
    assert_eq!(bs2.lower_bound(), 0);

    let mut extval = 0;
    let p3 = |i: i64| {
        extval += 1;
        xs[i as usize] >= 100
    };
    let mut bs3 = BinarySearch {
        p: p3,
        lower: 0,
        upper: xs.len() as i64 - 1,
    };
    assert_eq!(bs3.lower_bound(), 9);
}
