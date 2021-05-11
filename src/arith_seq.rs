use cargo_snippet::snippet;

#[snippet("ArithSeq")]
pub struct ArithSeq {
    pub a: i64,
    pub b: i64,
}
#[snippet("ArithSeq")]
impl ArithSeq {
    #[doc = "y=ax+b"]
    pub fn new(a: i64, b: i64) -> Self {
        ArithSeq {
            a: a,
            b: b,
        }
    }
    /// (x0,y0)を通る傾きaの直線を求める
    pub fn from_point(x0: i64, y0: i64, a: i64) -> Self {
        let b = y0 - a * x0;
        Self::new(a,b)
    }
    pub fn y(&self, x: i64) -> i64 {
        self.a * x + self.b
    }
    /// ceil(水平線y=y0との交点)
    pub fn x(&self, y: i64) -> i64 {
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
            // (b+c)/2=y0
            let c = 2*y - b;
            // y方向にミラーする
            let mirror = Self::new(-a,c);
            mirror.x(y)
        }
    }
}
#[test]
fn test_arith_seq_lower_bound() {
    let f = ArithSeq::new(3,1);
    assert_eq!(f.x(-2), -1);
    assert_eq!(f.x(1), 0);
    assert_eq!(f.x(2), 1);
    assert_eq!(f.x(3), 1);
    assert_eq!(f.x(4), 1);
    assert_eq!(f.x(5), 2);

    let f = ArithSeq::new(-3,1);
    assert_eq!(f.x(4), -1);
    assert_eq!(f.x(3), 0);
    assert_eq!(f.x(2), 0);
    assert_eq!(f.x(1), 0);
    assert_eq!(f.x(0), 1);
    assert_eq!(f.x(-1), 1);
    assert_eq!(f.x(-2), 1);
    assert_eq!(f.x(-3), 2);
}