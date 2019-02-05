#[snippet = "SEG_LAZY"]
trait SEGImpl {
    type Elem: Clone;
    fn up(l: usize, r: usize, e: Self::Elem) -> Self::Elem;
    /// current value, lazy value -> new value, child's lazy value
    fn down(cur: Self::Elem, lazy_val: Self::Elem) -> (Self::Elem, Self::Elem); fn id() -> Self::Elem;
    fn op(x: &Self::Elem, y: &Self::Elem) -> Self::Elem;
}

#[snippet = "SEG_LAZY"]
struct SEG<T: SEGImpl> {
    n: usize,
    node: Vec<T::Elem>,
    lazy: Vec<Option<T::Elem>>,
}

#[snippet = "SEG_LAZY"]
impl <T: SEGImpl> SEG<T> {
    fn new(n: usize) -> SEG<T> {
        let mut m = 1;
        while m < n { m *= 2; }
        SEG {
            n: m,
            node: vec![T::id(); m*2-1],
            lazy: vec![None; m*2-1],
        }
    }
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if let Some(lzv) = self.lazy[k].clone() {
            let cur_val = self.node[k].clone();
            let (new_val, child_lzv) = T::down(cur_val, lzv);
            self.node[k] = new_val;
            if r - l > 1 {
                self.lazy[k*2+1] = match self.lazy[k*2+1].clone() {
                    Some(x) => Some(T::op(&x, &child_lzv)),
                    None => Some(child_lzv.clone()),
                };
                self.lazy[k*2+2] = match self.lazy[k*2+2].clone() {
                    Some(x) => Some(T::op(&x, &child_lzv)),
                    None => Some(child_lzv.clone()),
                };
            }
            self.lazy[k] = None;
        }
    }
    fn do_update(&mut self, a: usize, b: usize, x: T::Elem, k: usize, l: usize, r: usize) {
        self.eval(k,l,r);

        if b <= l || r <= a {
            return;
        }

        if a <= l && r <= b {
            self.lazy[k] = match self.lazy[k].clone() {
                Some(a) => Some(T::op(&a, &T::up(l,r,x))),
                None => Some(T::up(l,r,x)),
            };
            self.eval(k,l,r);
        }

        else {
            self.do_update(a,b,x.clone(),2*k+1,l,(l+r)/2);
            self.do_update(a,b,x.clone(),2*k+2,(l+r)/2,r);
            self.node[k] = T::op(&self.node[2*k+1],&self.node[2*k+2]);
        }
    }
    fn update(&mut self, a: usize, b: usize, x: T::Elem) {
        let n = self.n;
        self.do_update(a,b,x,0,0,n)
    }
    fn do_query(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T::Elem {
        if r <= a || b <= l {
            return T::id();
        }

        self.eval(k,l,r);

        if a <= l && r <= b {
            return self.node[k].clone();
        } else {
            let vl = self.do_query(a,b,k*2+1,l,(l+r)/2);
            let vr = self.do_query(a,b,k*2+2,(l+r)/2,r);
            return T::op(&vl,&vr);
        }
    }
    fn query(&mut self, a: usize, b: usize) -> T::Elem {
        let n = self.n;
        self.do_query(a,b,0,0,n)
    }
}

struct RangeUpdate;
impl SEGImpl for RangeUpdate {
    type Elem = i32;
    fn id() -> Self::Elem {
        0
    }
    fn op(x: &Self::Elem, y: &Self::Elem) -> Self::Elem {
        // we need this comparison so the id value loses
        std::cmp::max(x.clone(), y.clone())
    }
    fn up(l: usize, r: usize, e: Self::Elem) -> Self::Elem {
        e
    }
    fn down(cur: Self::Elem, lazy_val: Self::Elem) -> (Self::Elem, Self::Elem) {
        (lazy_val, lazy_val)
    }
}
#[test]
fn test_range_update() {
    let mut seg: SEG<RangeUpdate> = SEG::new(10);
    assert_eq!(seg.query(0, 3), 0);
    seg.update(0, 2, 10);
    assert_eq!(seg.query(0, 3), 10);
    assert_eq!(seg.query(2, 3), 0);
    seg.update(1, 5, 20);
    assert_eq!(seg.query(0, 3), 20);
    assert_eq!(seg.query(0, 1), 10);
}

struct RangeAdd;
impl SEGImpl for RangeAdd {
    type Elem = i32;
    fn id() -> Self::Elem {
        0
    }
    fn op(x: &Self::Elem, y: &Self::Elem) -> Self::Elem {
        x.clone() + y.clone()
    }
    fn up(l: usize, r: usize, e: Self::Elem) -> Self::Elem {
        e * (r - l) as i32
    }
    fn down(cur: Self::Elem, lazy_val: Self::Elem) -> (Self::Elem, Self::Elem) {
        (cur+lazy_val, lazy_val/2)
    }
}
#[test]
fn test_range_add() {
    let mut seg: SEG<RangeAdd> = SEG::new(10);
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