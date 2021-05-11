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
    pub fn y(&self, x0: i64) -> i64 {
        self.a * x0 + self.b
    }
    /// ceil(y=y0との交点)
    pub fn x_ceil(&self, y: i64) -> i64 {
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
            mirror.x_ceil(y)
        }
    }
    /// floor(y=y0との交点)
    pub fn x_floor(&self, y0: i64) -> i64 {
        let x0 = self.x_ceil(y0);
        let y = self.y(x0);
        if y == y0 {
            return x0;
        }
        if self.a > 0 && y > y0 {
            x0 - 1
        } else if self.a < 0 && y < y0 {
            x0 - 1
        } else {
            x0
        }
    }
}
#[test]
fn test_arith_seq_lower_bound() {
    let f = ArithSeq::new(3,1);
    assert_eq!(f.x_ceil(-2), -1);
    assert_eq!(f.x_ceil(1), 0);
    assert_eq!(f.x_ceil(2), 1);
    assert_eq!(f.x_ceil(3), 1);
    assert_eq!(f.x_ceil(4), 1);
    assert_eq!(f.x_ceil(5), 2);

    let f = ArithSeq::new(-3,1);
    assert_eq!(f.x_ceil(4), -1);
    assert_eq!(f.x_ceil(3), 0);
    assert_eq!(f.x_ceil(2), 0);
    assert_eq!(f.x_ceil(1), 0);
    assert_eq!(f.x_ceil(0), 1);
    assert_eq!(f.x_ceil(-1), 1);
    assert_eq!(f.x_ceil(-2), 1);
    assert_eq!(f.x_ceil(-3), 2);
}