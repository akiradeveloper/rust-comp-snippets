use cargo_snippet::snippet;

#[snippet("SqrtDecomposition")]
#[derive(Debug, Copy, Clone)]
enum Bucket {
    #[doc = "b-idx"]
    Filled { bucket_index: usize },
    #[doc = "b-idx, [to, from)"]
    Partial { bucket_index: usize, l: usize, r: usize },
}
#[snippet("SqrtDecomposition")]
struct Buckets {
    pub d: usize,
}
#[snippet("SqrtDecomposition")]
impl Buckets {
    pub fn new(n: usize) -> Self {
        let mut d = 1;
        while d*d < n {
            d += 1;
        }
        Self {
            d,
        }
    }
    pub fn n(&self) -> usize {
        self.d * self.d
    }
    pub fn bucket(&self, i: usize) -> usize {
        i / self.d
    }
    #[doc = "[l, r)"]
    pub fn buckets(&self, l: usize, r: usize) -> Vec<Bucket> {
        if l == r {
            return vec![]
        }
        assert!(r>l);

        let mut res = vec![];
        let bl = l / self.d;
        let br = (r-1) / self.d;
        if bl == br {
            if l % self.d == 0 && r % self.d == 0 {
                res.push(Bucket::Filled { bucket_index: bl });
            } else {
                res.push(Bucket::Partial { bucket_index: bl, l, r });
            }
        } else {
            if l % self.d == 0 {
                res.push(Bucket::Filled { bucket_index: bl });
            } else {
                let bnext = bl+1;
                res.push(Bucket::Partial { bucket_index: bl, l, r: bnext * self.d });
            }
            for i in bl+1..br {
                res.push(Bucket::Filled { bucket_index: i });
            }
            if r % self.d == 0 {
                res.push(Bucket::Filled { bucket_index: br });
            } else {
                res.push(Bucket::Partial { bucket_index: br, l: br * self.d, r });
            }
        }
        res
    }
}
#[test]
fn test_buckets() {
    let sc = Buckets::new(10);
    assert_eq!(sc.d, 4);
    assert_eq!(sc.bucket(0), 0);
    assert_eq!(sc.bucket(4), 1);
    dbg!(sc.buckets(0, 3));
    dbg!(sc.buckets(0, 4));
    dbg!(sc.buckets(1, 4));
    dbg!(sc.buckets(2, 6));
    dbg!(sc.buckets(2, 9));
    dbg!(sc.buckets(2, 10));
    dbg!(sc.buckets(2, 12));
}