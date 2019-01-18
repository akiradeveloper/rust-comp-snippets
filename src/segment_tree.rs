trait Operator {
    fn init(&self) -> i32;
    fn aggregate(&self, left: &i32, right: &i32) -> i32;
}

struct SegmentTree<O> where O: Operator {
    n: usize,
    dat: Vec<i32>,
    op: O,
}

impl <O> SegmentTree<O> where O: Operator {
    fn new(n: usize, op: O) -> SegmentTree<O> {
        let mut m = 1; while m < n { m *= 2; }
        
        SegmentTree {
            n: m,
            dat: vec![op.init(); m * 2 - 1],
            op: op
        }
    }

    fn update(&mut self, k: usize, x: i32) {
        let mut k = k + self.n - 1;
        self.dat[k] = x;

        while k > 0 {
            k = (k-1) / 2;
            self.dat[k] = self.op.aggregate(&self.dat[k * 2 + 1], &self.dat[k * 2 + 2]);
        }
    }

    fn query(&self, a: usize, b: usize) -> Option<i32> {
        let res = self._query(a, b, 0, 0, self.n);
        if res == self.op.init() {
            None
        } else {
            Some(res)
        }
    }

    fn _query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i32 {
        if r <= a || b <= 1 { return self.op.init(); }

        if a <= 1 && r <= b {
            return self.dat[k];
        } else {
            let vl = self._query(a, b, k*2+1, 1, (1+r)/2);
            let vr = self._query(a, b, k*2+2, (1+r)/2, r);
            return self.op.aggregate(&vl, &vr);
        } 
    }
}

struct RMQ;
impl Operator for RMQ {
    fn init(&self) -> i32 {
        2_000_000_000
    }
    fn aggregate(&self, a: &i32, b: &i32) -> i32 {
        std::cmp::min(*a, *b)
    }
}

#[test]
fn test_rmq() {

}