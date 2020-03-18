use cargo_snippet::snippet;

use crate::seg::{SEG, Monoid};

#[snippet("MaxCover")]
struct MAX;
#[snippet("MaxCover")]
impl Monoid for MAX {
    type T = i64;
    fn id() -> Self::T {
        0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        std::cmp::max(*a, *b)
    }
}

#[snippet("MaxCover")]
struct MaxCover {
    n: usize,
    dp: SEG<MAX>,
    segs: Vec<(usize,usize,i64)>,
}
#[snippet("MaxCover")]
impl MaxCover {
    pub fn new(n: usize) -> Self {
        MaxCover {
            n: n,
            dp: SEG::new(n),
            segs: Vec::new(),
        }
    }
    #[doc = "[x,y]"]
    pub fn add(&mut self, x: usize, y: usize, a: i64) {
        self.segs.push((x,y,a));
    }
    #[doc = "O(mlogm + mlogn). m is the # of segments"]
    pub fn query(&mut self) -> i64 {
        let m = self.segs.len();
        self.segs.sort_by_key(|a| a.0);
        for i in 0..m {
            let (x,y,a) = self.segs[i];
            let curmax = self.dp.query(0, x);
            let newval = curmax + a;
            if self.dp.get(y) < newval {
                self.dp.update(y, newval);
            }
        }
        self.dp.query(0, self.n)
    }
}

#[test]
fn test_max_cover() {
    let mut mc = MaxCover::new(5);
    mc.add(1,2,10);
    mc.add(3,4,20);
    mc.add(2,3,40);
    assert_eq!(mc.query(), 40);
}