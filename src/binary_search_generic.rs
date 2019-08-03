struct BinarySearch<F> {
    p: F,
    lower: usize,
    upper: usize,
}

impl <F: Fn(usize) -> bool> BinarySearch<F> {
    fn search(&self) -> (Option<usize>, usize) {
        let mut lb = self.lower;
        let mut ub = self.upper+2;
        while ub - lb > 1 {
            let mid = (lb+ub)/2;
            if (self.p)(mid-1) {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        let former = if lb == self.lower {
            None
        } else {
            Some(lb-1)
        };
        let latter = ub-1;
        (former, latter)
    }
}

#[test]
fn test_binary_search_generic_unique() {
    let always_false = |i: usize| { false };
    let bs0 = BinarySearch {
        p: always_false,
        lower: 0,
        upper: 100000,
    };
    dbg!(bs0.search());

    let always_true = |i: usize| { true };
    let bs1 = BinarySearch {
        p: always_true,
        lower: 0,
        upper: 100000,
    };
    dbg!(bs1.search());
}

#[test]
fn test_binary_search_generic_array_ref() {
    let xs = vec![1,2,2,2,2,2,3,4,5];
    let p0 = |i: usize| { xs[i] >= 2 };
    let bs0 = BinarySearch {
        p: p0,
        lower: 0,
        upper: xs.len()-1,
    };
    dbg!(bs0.search());

    let p1 = |i: usize| { xs[i] > 2 };
    let bs1 = BinarySearch {
        p: p1,
        lower: 0,
        upper: xs.len()-1,
    };
    dbg!(bs1.search());

    let p2 = |i: usize| { xs[i] >= 0 };
    let bs2 = BinarySearch {
        p: p2,
        lower: 0,
        upper: xs.len()-1,
    };
    dbg!(bs2.search());

    let p3 = |i: usize| { xs[i] >= 100 };
    let bs3 = BinarySearch {
        p: p3,
        lower: 0,
        upper: xs.len()-1,
    };
    dbg!(bs3.search());
}