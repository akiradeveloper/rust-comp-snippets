#[snippet = "ArithSeq"]
struct ArithSeq {
    a: i64,
    b: i64,
}
#[snippet = "ArithSeq"]
impl ArithSeq {
    #[doc = "y=ai+b"]
    pub fn new(a: i64, b: i64) -> ArithSeq {
        assert!(a>0);
        ArithSeq {
            a: a,
            b: b,
        }
    }
    #[doc = "ai+b >= x"]
    pub fn next(&self, x: i64) -> i64 {
        if x >= self.b {
            let d = x - self.b;
            let i = (d - 1 + self.a) / self.a;
            self.a * i + self.b
        } else {
            let d = self.b - x;
            let i = d / self.a;
            self.b - self.a * i
        }
    }
    #[doc = "ai+b <= x"]
    pub fn prev(&self, x: i64) -> i64 {
        let next = self.next(x);
        if next == x {
            x
        } else {
            next - self.a
        }
    }
    #[doc = "[ai+b, n] <= [l, u]"]
    pub fn range(&self, l: i64, u: i64) -> Option<(i64, i64)> {
        if l > u {
            return None
        }
        let x = self.next(l);
        let y = self.prev(u);
        if x > y {
            return None
        }
        assert!(x <= y);
        if l<=x && x<=u {
            let cnt = (y-x) / self.a;
            Some((x, cnt+1))
        } else {
            None
        }
    }
}

#[test]
fn test_arith_seq_next() {
    let x = ArithSeq {
        a: 3,
        b: 0,
    };
    assert_eq!(x.next(-6), -6);
    assert_eq!(x.next(-5), -3);
    assert_eq!(x.next(-4), -3);
    assert_eq!(x.next(-3), -3);
    assert_eq!(x.next(-2), 0);
    assert_eq!(x.next(-1), 0);
    assert_eq!(x.next(0), 0);
    assert_eq!(x.next(1), 3);
    assert_eq!(x.next(2), 3);
    assert_eq!(x.next(3), 3);
    assert_eq!(x.next(4), 6);

    let mut x = ArithSeq {
        a: 3,
        b: 1,
    };
    assert_eq!(x.next(-5), -5);
    assert_eq!(x.next(-4), -2);
    assert_eq!(x.next(-3), -2);
    assert_eq!(x.next(-2), -2);
    assert_eq!(x.next(-1), 1);
    assert_eq!(x.next(0), 1);
    assert_eq!(x.next(1), 1);
    assert_eq!(x.next(2), 4);
    assert_eq!(x.next(3), 4);
    assert_eq!(x.next(4), 4);
    assert_eq!(x.next(5), 7);

    let x = ArithSeq {
        a: 1,
        b: 8,
    };
    assert_eq!(x.next(1), 1);
}

#[test]
fn test_arith_seq_prev() {
    let x = ArithSeq {
        a: 3,
        b: 0,
    };
    assert_eq!(x.prev(6), 6);
    assert_eq!(x.prev(5), 3);
    assert_eq!(x.prev(4), 3);
    assert_eq!(x.prev(3), 3);
    assert_eq!(x.prev(2), 0);
    assert_eq!(x.prev(1), 0);
    assert_eq!(x.prev(0), 0);
    assert_eq!(x.prev(-1), -3);
    assert_eq!(x.prev(-2), -3);
    assert_eq!(x.prev(-3), -3);

    let x = ArithSeq {
        a: 3,
        b: 1,
    };
    assert_eq!(x.prev(6), 4);
    assert_eq!(x.prev(5), 4);
    assert_eq!(x.prev(4), 4);
    assert_eq!(x.prev(3), 1);
    assert_eq!(x.prev(2), 1);
    assert_eq!(x.prev(1), 1);
    assert_eq!(x.prev(0), -2);
    assert_eq!(x.prev(-1), -2);
    assert_eq!(x.prev(-2), -2);
    assert_eq!(x.prev(-3), -5);
    assert_eq!(x.prev(-4), -5);
    assert_eq!(x.prev(-5), -5);
}

#[test]
fn test_arith_seq_range() {
    let x = ArithSeq {
        a: 3,
        b: 1,
    };
    assert_eq!(x.range(1,4), Some((1,2)));
    assert_eq!(x.range(1,3), Some((1,1)));
    assert_eq!(x.range(2,3), None);
    assert_eq!(x.range(2,4), Some((4,1)));
    assert_eq!(x.range(0,5), Some((1,2)));
    assert_eq!(x.range(0,4), Some((1,2)));
    assert_eq!(x.range(-3,5), Some((-2,3)));

    let x = ArithSeq {
        a: 1,
        b: 8,
    };
    assert_eq!(x.range(1,9), Some((1,9)));
}