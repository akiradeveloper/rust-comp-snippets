#[snippet = "BinarySearch"]
struct BinarySearch<F> {
    p: F,
    lower: usize,
    upper: usize,
}
#[snippet = "BinarySearch"]
impl <F: Fn(usize) -> bool> BinarySearch<F> {
    fn search(&self) -> (Option<usize>, usize) {
        let lower = self.lower as i32;
        let upper = self.upper as i32;

        let mut lb = lower - 1; 
        let mut ub = upper + 1;
        while ub - lb > 1 {
            let mid = (lb+ub)/2;
            if (self.p)(mid as usize) {
                ub = mid;
            } else {
                lb = mid;
            }
        }
        let former = if lb < 0 {
            None
        } else {
            Some(lb as usize)
        };
        let latter = ub as usize;
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

