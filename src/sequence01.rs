#[snippet = "BinarySearch"]
#[doc = "lower,upper are inclusive range"]
struct BinarySearch<F> {
    p: F,
    lower: i64,
    upper: i64,
}
#[snippet = "BinarySearch"]
impl <F: Fn(i64) -> bool> BinarySearch<F> {
    #[doc = "O(log(upper-lower))"]
    fn lower_bound(&self) -> i64 {
        let lower = self.lower;
        let upper = self.upper;

        let mut lb = lower - 1; 
        let mut ub = upper + 1;
        while ub - lb > 1 {
            let mid = (lb+ub)/2;
            if (self.p)(mid) {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        let latter = ub;
        latter
    }
}

#[test]
fn test_binary_search_generic_unique() {
    let always_false = |i: i64| { false };
    let bs0 = BinarySearch {
        p: always_false,
        lower: 0,
        upper: 100000,
    };
    assert_eq!(bs0.lower_bound(), 100001);

    let always_true = |i: i64| { true };
    let bs1 = BinarySearch {
        p: always_true,
        lower: 0,
        upper: 100000,
    };
    assert_eq!(bs1.lower_bound(), 0);
}

#[test]
fn test_binary_search_generic_array_ref() {
    let xs = vec![1,2,2,2,2,2,3,4,5];
    let p0 = |i: i64| { xs[i as usize] >= 2 };
    let bs0 = BinarySearch {
        p: p0,
        lower: 0,
        upper: xs.len() as i64 - 1,
    };
    assert_eq!(bs0.lower_bound(), 1);

    let p1 = |i: i64| { xs[i as usize] > 2 };
    let bs1 = BinarySearch {
        p: p1,
        lower: 0,
        upper: xs.len() as i64 - 1,
    };
    assert_eq!(bs1.lower_bound(), 6);

    let p2 = |i: i64| { xs[i as usize] >= 0 };
    let bs2 = BinarySearch {
        p: p2,
        lower: 0,
        upper: xs.len() as i64 - 1,
    };
    assert_eq!(bs2.lower_bound(), 0);

    let p3 = |i: i64| { xs[i as usize] >= 100 };
    let bs3 = BinarySearch {
        p: p3,
        lower: 0,
        upper: xs.len() as i64 - 1,
    };
    assert_eq!(bs3.lower_bound(), 9);
}

#[snippet = "FTSearch"]
struct FTSearch<F> {
    f_search: Vec<Option<usize>>,
    t_search: Vec<usize>,
    p: F,
    lower: usize,
}
#[snippet = "FTSearch"]
impl <F: Fn(usize) -> bool> FTSearch<F> {
    #[doc = "O(upper-lower)"]
    fn new(p: F, lower: usize, upper: usize) -> FTSearch<F> {
        let n = upper+1 - lower;
        let mut f_search = vec![None; n];
        let mut f_i = None;
        for i in 0..n {
            if p(i+lower) == false {
                f_i = Some(i);
            }
            f_search[i] = f_i;
        }
        let mut t_search = vec![n; n];
        let mut t_i = n;
        for i in (0..n).rev() {
            if p(i+lower) == true {
                t_i = i;
            }
            t_search[i] = t_i;
        }
        Self {
            p: p,
            f_search: f_search,
            t_search: t_search,
            lower: lower,
        }
    }
    #[doc = "including i and find the closest false in the left"]
    fn f_search(&self, i: usize) -> Option<usize> {
        self.f_search[i-self.lower].map(|x| x+self.lower)
    }
    #[doc = "including i and find the closest true in the right"]
    fn t_search(&self, i: usize) -> usize {
        self.t_search[i-self.lower] + self.lower
    }
}
#[test]
fn test_ft_search() {
    let xs = vec![true,false,false,true,false];
    let ft = FTSearch::new(
        |i: usize| { xs[i] },
        0,
        4,
    );
    assert_eq!(ft.f_search(0), None); assert_eq!(ft.t_search(0), 0);
    assert_eq!(ft.f_search(1), Some(1)); assert_eq!(ft.t_search(1), 3);
    assert_eq!(ft.f_search(2), Some(2)); assert_eq!(ft.t_search(2), 3);
    assert_eq!(ft.f_search(3), Some(2)); assert_eq!(ft.t_search(3), 3);
    assert_eq!(ft.f_search(4), Some(4)); assert_eq!(ft.t_search(4), 5);
}

