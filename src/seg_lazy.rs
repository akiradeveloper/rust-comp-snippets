/// https://ei1333.github.io/luzhiled/snippets/structure/segment-tree.html

#[snippet = "SEG_LAZY"]
trait SEGImpl {
    type Monoid: Copy;
    type OperatorMonoid: Copy + PartialEq;
    fn m0() -> Self::Monoid;
    fn om0() -> Self::OperatorMonoid;
    fn f(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid;
    fn g(x: Self::Monoid, y: Self::OperatorMonoid, len: usize) -> Self::Monoid;
    fn h(x: Self::OperatorMonoid, y: Self::OperatorMonoid) -> Self::OperatorMonoid;
}

#[snippet = "SEG_LAZY"]
struct SEG<T: SEGImpl> {
    n: usize,
    data: Vec<T::Monoid>,
    lazy: Vec<T::OperatorMonoid>,
}

#[snippet = "SEG_LAZY"]
impl <T: SEGImpl> SEG<T> {
    fn new(init: T::Monoid, n: usize) -> SEG<T> {
        let mut m = 1;
        while m < n { m *= 2; }
        SEG {
            n: m,
            data: vec![init; m*2],
            lazy: vec![T::om0(); m*2],
        }
    }
    fn propagate(&mut self, k: usize, len: usize) {
        if self.lazy[k] != T::om0() {
            if k < self.n {
                self.lazy[2*k+0] = T::h(self.lazy[2*k+0], self.lazy[k]);
                self.lazy[2*k+1] = T::h(self.lazy[2*k+1], self.lazy[k]);
            }
            self.data[k] = T::g(self.data[k], self.lazy[k], len);
            self.lazy[k] = T::om0();
        }
    }
    fn do_update(&mut self, a: usize, b: usize, x: T::OperatorMonoid, k: usize, l: usize, r: usize) -> T::Monoid {
        self.propagate(k, r-l);
        if r <= a || b <= l {
            self.data[k]
        } else if a <= l && r <= b {
            self.lazy[k] = T::h(self.lazy[k], x);
            self.propagate(k, r-l);
            self.data[k]
        } else {
            self.data[k] = T::f(
                self.do_update(a, b, x, 2*k+0, l, (l+r)>>1),
                self.do_update(a, b, x, 2*k+1, (l+r)>>1, r)
            );
            self.data[k]
        }
    }
    fn update(&mut self, a: usize, b: usize, x: T::OperatorMonoid) -> T::Monoid {
        let n = self.n;
        self.do_update(a, b, x, 1, 0, n)
    }
    fn do_query(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T::Monoid {
        self.propagate(k, r-l);
        if r <= a || b <= l {
            T::m0()
        } else if a <= l && r <= b {
            self.data[k]
        } else {
            T::f(
                self.do_query(a, b, 2*k+0, l, (l+r)>>1),
                self.do_query(a, b, 2*k+1, (l+r)>>1, r)
            )
        }
    }
    // [a,b)
    fn query(&mut self, a: usize, b: usize) -> T::Monoid {
        let n = self.n;
        self.do_query(a, b, 1, 0, n)
    }
}

#[snippet = "SEG_LAZY_MAX_RUQ"]
struct MAX_RUQ;
#[snippet = "SEG_LAZY_MAX_RUQ"]
impl SEGImpl for MAX_RUQ {
    type Monoid = i64;
    type OperatorMonoid = i64;
    fn m0() -> Self::Monoid {
        0
    }
    fn om0() -> Self::OperatorMonoid {
        0
    }
    fn f(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        std::cmp::max(x, y)
    }
    fn g(x: Self::Monoid, y: Self::OperatorMonoid, len: usize) -> Self::Monoid {
        y
    }
    fn h(x: Self::OperatorMonoid, y: Self::OperatorMonoid) -> Self::OperatorMonoid {
        y
    }
}
#[test]
fn test_MAX_RUQ() {
    let mut seg: SEG<MAX_RUQ> = SEG::new(MAX_RUQ::m0(), 10);
    assert_eq!(seg.query(0, 3), 0);
    seg.update(0, 2, 10); // [10,10,0,...]
    assert_eq!(seg.query(0, 3), 10);
    assert_eq!(seg.query(2, 3), 0);
    seg.update(1, 5, 20);
    assert_eq!(seg.query(0, 3), 20);
    assert_eq!(seg.query(0, 1), 10);
}

#[snippet = "SEG_LAZY_MIN_RUQ"]
struct MIN_RUQ;
#[snippet = "SEG_LAZY_MIN_RUQ"]
impl SEGImpl for MIN_RUQ {
    type Monoid = i64;
    type OperatorMonoid = i64;
    fn m0() -> Self::Monoid {
        std::i64::MAX
    }
    fn om0() -> Self::OperatorMonoid {
        0
    }
    fn f(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        std::cmp::min(x, y)
    }
    fn g(x: Self::Monoid, y: Self::OperatorMonoid, len: usize) -> Self::Monoid {
        y
    }
    fn h(x: Self::OperatorMonoid, y: Self::OperatorMonoid) -> Self::OperatorMonoid {
        y
    }
}
#[test]
fn test_MIN_RUQ() { // DSL_2_D
    let mut seg: SEG<MIN_RUQ> = SEG::new(MIN_RUQ::m0(), 8);
    seg.update(1,7,5);
    seg.update(2,8,2);
    seg.update(2,6,7);
    assert_eq!(seg.query(3, 4),7);
    seg.update(4,7,6);
    assert_eq!(seg.query(0, 1),std::i64::MAX);
    seg.update(0,8,9);
    assert_eq!(seg.query(2, 3),9);
    assert_eq!(seg.query(3, 4),9);
    seg.update(1,8,2);
}

#[snippet = "SEG_LAZY_SUM_RUQ"]
struct SUM_RUQ;
#[snippet = "SEG_LAZY_SUM_RUQ"]
impl SEGImpl for SUM_RUQ { 
    type Monoid = i64;
    type OperatorMonoid = i64;
    fn m0() -> Self::Monoid {
        0
    }
    fn om0() -> Self::OperatorMonoid {
        std::i64::MAX
    }
    fn f(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        x + y
    }
    fn g(x: Self::Monoid, y: Self::OperatorMonoid, len: usize) -> Self::Monoid {
        len as i64 * y
    }
    fn h(x: Self::OperatorMonoid, y: Self::OperatorMonoid) -> Self::OperatorMonoid {
        y
    }
}
#[test]
fn test_SUM_RUQ() { // DSL_1_I
    let mut seg: SEG<SUM_RUQ> = SEG::new(SUM_RUQ::m0(), 8);
    seg.update(1,7,-5);
    seg.update(2,5,-9);
    assert_eq!(seg.query(2,4),-18);
    seg.update(3,7,0);
    assert_eq!(seg.query(0,4),-14); // this
    assert_eq!(seg.query(5,8),0);
    assert_eq!(seg.query(2,7),-9);
    seg.update(3,8,9);
    assert_eq!(seg.query(2,6),18);
    seg.update(0,2,1);
}

#[snippet = "SEG_LAZY_SUM_RAQ"]
struct SUM_RAQ;
#[snippet = "SEG_LAZY_SUM_RAQ"]
impl SEGImpl for SUM_RAQ {
    type Monoid = i64;
    type OperatorMonoid = i64;
    fn m0() -> Self::Monoid {
        0
    }
    fn om0() -> Self::OperatorMonoid {
        0
    }
    fn f(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        x + y
    }
    fn g(x: Self::Monoid, y: Self::OperatorMonoid, len: usize) -> Self::Monoid {
        x + (len as i64) * y
    }
    fn h(x: Self::OperatorMonoid, y: Self::OperatorMonoid) -> Self::OperatorMonoid {
        x + y
    }
}
#[test]
fn test_SUM_RAQ() {
    let mut seg: SEG<SUM_RAQ> = SEG::new(0, 10);
    assert_eq!(seg.query(0, 3), 0);
    seg.update(0,5,10);
    assert_eq!(seg.query(0, 1), 10);
    assert_eq!(seg.query(0, 2), 20);
    assert_eq!(seg.query(0, 5), 50);
    assert_eq!(seg.query(0, 6), 50);
    seg.update(3,6,5);
    assert_eq!(seg.query(0, 5), 60);
    assert_eq!(seg.query(0, 6), 65);
    assert_eq!(seg.query(4, 7), 20);
}

#[snippet = "SEG_LAZY_MIN_RAQ"]
struct MIN_RAQ;
#[snippet = "SEG_LAZY_MIN_RAQ"]
impl SEGImpl for MIN_RAQ { 
    type Monoid = i64;
    type OperatorMonoid = i64;
    fn m0() -> Self::Monoid {
        std::i64::MAX
    }
    fn om0() -> Self::OperatorMonoid {
        0
    }
    fn f(x: Self::Monoid, y: Self::Monoid) -> Self::Monoid {
        std::cmp::min(x, y)
    }
    fn g(x: Self::Monoid, y: Self::OperatorMonoid, len: usize) -> Self::Monoid {
        x + (len as i64) * y
    }
    fn h(x: Self::OperatorMonoid, y: Self::OperatorMonoid) -> Self::OperatorMonoid {
        x + y
    }
}
#[test]
fn test_rmq_raq() { // DSL_2_H
    let mut seg: SEG<MIN_RAQ> = SEG::new(0, 6);
    seg.update(1,4,1);
    seg.update(2,5,-2);
    assert_eq!(seg.query(0,6),-2);
    assert_eq!(seg.query(0,2),0);
    seg.update(3,6,3);
    assert_eq!(seg.query(3,5),1);
    assert_eq!(seg.query(0,6),-1);
}

