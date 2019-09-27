#[snippet = "ConvexHullTrick"]
struct ConvexHullTrick {
    lines: Vec<(i64, i64)>,
}
#[snippet = "ConvexHullTrick"]
impl ConvexHullTrick {
    pub fn new() -> ConvexHullTrick {
        ConvexHullTrick {
            lines: vec![],
        }
    }
    fn check(l1: (i64,i64), l2: (i64,i64), l3: (i64,i64)) -> bool {
        let mut l1=l1;
        let mut l3=l3;
        if l1<l3 {
            std::mem::swap(&mut l1, &mut l3);
        }
        (l3.1 - l2.1) * (l2.0 - l1.0) >= (l2.1 - l1.1) * (l3.0 - l2.0)
    }
    pub fn add(&mut self, a: i64, b: i64) {
        let line = (a,b);
        while self.lines.len() >= 2 && Self::check(self.lines[self.lines.len()-2], self.lines[self.lines.len()-1], line) {
            self.lines.pop();
        }
        self.lines.push(line);
    }
    fn f(&self, i: usize, x: i64) -> i64 {
        let line = self.lines[i];
        line.0 * x + line.1
    }
    #[doc = "min: l>=r, max: l<=r"]
    pub fn get<F: Fn(i64,i64)->bool>(&self, x: i64, comp: F) -> i64 {
        let mut low: i64 = -1;
        let mut high: i64 = (self.lines.len() - 1) as i64;
        while high - low > 1 {
            let mid = (high + low) / 2;
            if comp(self.f(mid as usize, x), self.f((mid+1) as usize, x)) {
                low = mid;
            } else {
                high = mid;
            }
        }
        self.f(high as usize, x)
    }
}

#[test]
fn test_convex_hull_trick() {
    let mut cht = ConvexHullTrick::new();
    let cmp = |l:i64,r:i64| {
        l>=r
    };
    cht.add(2,0);
    assert_eq!(cht.get(-2, cmp), -4);
    assert_eq!(cht.get(2, cmp), 4);
    cht.add(0,-1);
    assert_eq!(cht.get(-2, cmp), -4);
    assert_eq!(cht.get(2, cmp), -1);
    cht.add(1,1);
    assert_eq!(cht.get(-2, cmp), -4);
    assert_eq!(cht.get(2, cmp), -1);
    cht.add(-1,0);
    assert_eq!(cht.get(-2, cmp), -4);
    assert_eq!(cht.get(2, cmp), -2);
}