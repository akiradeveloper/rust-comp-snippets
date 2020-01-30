#[snippet = "ConvexHullTrick"]
#[derive(Clone,Copy)]
struct Line {
    a: i64,
    b: i64
}
#[snippet = "ConvexHullTrick"]
impl Line {
    pub fn new(a: i64, b: i64) -> Line {
        Line {
            a: a,
            b: b,
        }
    }
    pub fn y(&self, x: i64) -> i64 {
        self.a*x + self.b
    }
}
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
    #[doc = "add a line f(x)=ax+b"]
    pub fn add(&mut self, line: Line) {
        let line = (line.a, line.b);
        while self.lines.len() >= 2 && Self::check(self.lines[self.lines.len()-2], self.lines[self.lines.len()-1], line) {
            self.lines.pop();
        }
        self.lines.push(line);
    }
    pub fn f(&self, i: usize, x: i64) -> i64 {
        let line = self.lines[i];
        line.0 * x + line.1
    }
    #[doc = "lower: l>=r, upper: l<=r"]
    fn get<F: Fn(i64,i64)->bool>(&self, x: i64, comp: F) -> Line {
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
        let (a,b) = self.lines[high as usize];
        Line { a: a, b: b }
    }
    pub fn get_upper(&self, x: i64) -> Line {
        self.get(x, |l,r| { l<=r })
    }
    pub fn get_lower(&self, x: i64) -> Line {
        self.get(x, |l,r| { l>=r })
    }
}

#[test]
fn test_convex_hull_trick() {
    let mut cht = ConvexHullTrick::new();
    cht.add(Line::new(2,0));
    assert_eq!(cht.get_lower(-2).y(-2), -4);
    assert_eq!(cht.get_lower(2).y(2), 4);
    cht.add(Line::new(0,-1));
    assert_eq!(cht.get_lower(-2).y(-2), -4);
    assert_eq!(cht.get_lower(2).y(2), -1);
    cht.add(Line::new(1,1));
    assert_eq!(cht.get_lower(-2).y(-2), -4);
    assert_eq!(cht.get_lower(2).y(2), -1);
    cht.add(Line::new(-1,0));
    assert_eq!(cht.get_lower(-2).y(-2), -4);
    assert_eq!(cht.get_lower(2).y(2), -2);
}