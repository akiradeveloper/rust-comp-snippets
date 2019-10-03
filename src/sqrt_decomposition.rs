#[snippet = "SqrtDecomposition"]
#[derive(Debug, Copy, Clone)]
enum Bucket {
    #[doc = "b-idx"]
    Filled(usize),
    #[doc = "b-idx, [to, from)"]
    Partial(usize, usize, usize),
}
#[snippet = "SqrtDecomposition"]
struct Buckets {
    n: usize,
    d: usize,
}
#[snippet = "SqrtDecomposition"]
impl Buckets {
    pub fn bucket(&self, i: usize) -> usize {
        i / self.d
    }
    
    #[doc = "[from, to)"]
    pub fn buckets(&self, from: usize, to: usize) -> Vec<Bucket> {
        let mut res = vec![];
        let mut i = 0;
        while to - i > 0 {
            let b = i / self.d;
            let next_boundary = (b+1) * self.d;
            if i % self.d == 0 {
                if next_boundary <= to {
                    // fill
                    res.push(Bucket::Filled(b));
                    i = next_boundary;
                } else {
                    // partial
                    res.push(Bucket::Partial(b, i, to));
                    i = to;
                }
            } else {
                // partial
                if next_boundary <= to {
                    res.push(Bucket::Partial(b, i, next_boundary));
                    i = next_boundary;
                } else {
                    res.push(Bucket::Partial(b, i, to));
                    i = to
                }
            }
        }
        res
    }
}
#[test]
fn test_buckets() {
    let sc = Buckets {
        n: 10,
        d: 3,
    };
    assert_eq!(sc.bucket((0)), 0);
    assert_eq!(sc.bucket((4)), 1);
    dbg!(sc.buckets(0, 3));
    dbg!(sc.buckets(0, 4));
    dbg!(sc.buckets(1, 4));
    dbg!(sc.buckets(2, 6));
    dbg!(sc.buckets(2, 9));
    dbg!(sc.buckets(2, 10));
}