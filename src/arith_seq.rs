#[snippet = "ArithSeq"]
pub struct ArithSeq {
    pub a: i64,
    pub b: i64,
}
#[snippet = "ArithSeq"]
impl ArithSeq {
    #[doc = "y=ax+b"]
    pub fn new(a: i64, b: i64) -> Self {
        ArithSeq {
            a: a,
            b: b,
        }
    }
    pub fn from_point(x0: i64, y0: i64, a: i64) -> Self {
        let b = y0 - a * x0;
        Self::new(a,b)
    }
    pub fn y(&self, x: i64) -> i64 {
        self.a * x + self.b
    }
    #[doc = "x such that f(x) >= y and f(x-1) < y"]
    pub fn lower_bound(&self, y: i64) -> i64 {
        assert!(self.a != 0);
        let a = self.a;
        let b = self.b;
        if a > 0 {
            let mut x = y - b;
            if x >= 0 {
                x += a-1;
            } else {
            }
            let x = x / a;
            x
        } else {
            let y = 2*b - y;
            let mirror = Self::new(-a,b);
            mirror.lower_bound(y)
        }
    }
    #[doc = "x such that f(x) < y and f(x+1) >= y"]
    pub fn upper_bound(&self, y: i64) -> i64 {
        unimplemented!()
    }
}
#[test]
fn test_arith_seq_lower_bound() {
    let f = ArithSeq::new(3,1);
    assert_eq!(f.lower_bound(-2), -1);
    assert_eq!(f.lower_bound(1), 0);
    assert_eq!(f.lower_bound(2), 1);
    assert_eq!(f.lower_bound(3), 1);
    assert_eq!(f.lower_bound(4), 1);
    assert_eq!(f.lower_bound(5), 2);

    let f = ArithSeq::new(-3,1);
    assert_eq!(f.lower_bound(4), -1);
    assert_eq!(f.lower_bound(3), 0);
    assert_eq!(f.lower_bound(2), 0);
    assert_eq!(f.lower_bound(1), 0);
    assert_eq!(f.lower_bound(0), 1);
    assert_eq!(f.lower_bound(-1), 1);
    assert_eq!(f.lower_bound(-2), 1);
    assert_eq!(f.lower_bound(-3), 2);
}