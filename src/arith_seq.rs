#[snippet = "ArithSeq"]
struct ArithSeq {
    a: i64,
    b: i64,
}
#[snippet = "ArithSeq"]
impl ArithSeq {
    #[doc = "a+bi >= x"]
    pub fn next(&self, x: i64) -> i64 {
        let d = x - self.a;
        let i = (d - 1 + self.b) / self.b;
        self.a + self.b * i
    }
    #[doc = "a+bi <= y"]
    pub fn prev(&self, x: i64) -> i64 {
        let next = self.next(x);
        if next == x {
            x
        } else {
            next - self.b
        }
    }
    #[doc = "[a+bi, a+bj] <= [l, r]"]
    pub fn range(&self, l: i64, u: i64) -> Option<(i64, i64, usize)> {
        None
    }
}

#[test]
fn test_arith_seq_next() {
    let x = ArithSeq {
        a: 0,
        b: 3,
    };
    assert_eq!(x.next(0), 0);
    assert_eq!(x.next(1), 3);
    assert_eq!(x.next(2), 3);
    assert_eq!(x.next(3), 3);
    assert_eq!(x.next(4), 6);

    let mut x = ArithSeq {
        a: 1,
        b: 3,
    };
    assert_eq!(x.next(0), 1);
    assert_eq!(x.next(1), 1);
    assert_eq!(x.next(2), 4);
    assert_eq!(x.next(3), 4);
    assert_eq!(x.next(4), 4);
    assert_eq!(x.next(5), 7);
}

#[test]
fn test_arith_seq_prev() {
    let x = ArithSeq {
        a: 1,
        b: 3,
    };
    assert_eq!(x.prev(4), 4);
    assert_eq!(x.prev(3), 1);
    assert_eq!(x.prev(2), 1);
    assert_eq!(x.prev(1), 1);
    assert_eq!(x.prev(0), -2);
}