/// https://github.com/hatoo/competitive-rust-snippets

#[snippet = "SEG"]
#[allow(dead_code)]
pub trait Monoid {
    type T: Clone;
    fn id() -> Self::T;
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}

#[snippet = "SEG"]
#[allow(dead_code)]
pub struct SEG<M: Monoid> {
    n: usize,
    buf: Vec<M::T>,
}

#[snippet = "SEG"]
impl<M: Monoid> SEG<M> {
    #[allow(dead_code)]
    pub fn new(n: usize) -> SEG<M> {
        let mut m = 1;
        while m < n { m *= 2; }
        SEG {
            n: m,
            buf: vec![M::id().clone(); 2 * m - 1],
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, k: usize, a: M::T) {
        let mut k = k + self.n - 1;
        self.buf[k] = a;

        while k > 0 {
            k = (k-1) / 2;
            self.buf[k] = M::op(&self.buf[k*2+1], &self.buf[k*2+2]);
        }
    }
    
    #[allow(dead_code)]
    pub fn get(&self, k: usize) -> M::T {
        self.buf[k + self.n].clone()
    }

    pub fn do_query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> M::T {
        if r <= a || b <= l {
            return M::id();
        }

        if a <= l && r <= b {
            return self.buf[k].clone();
        } else {
            let vl = self.do_query(a,b,k*2+1,l,(l+r)/2);
            let vr = self.do_query(a,b,k*2+2,(l+r)/2,r);
            return M::op(&vl, &vr);
        }
    }

    #[allow(dead_code)]
    pub fn query(&self, a: usize, b: usize) -> M::T {
        self.do_query(a,b,0,0,self.n)
    }
}

#[snippet = "Monoid-SUM"]
#[allow(dead_code)]
struct SUM;
#[snippet = "Monoid-SUM"]
impl Monoid for SUM {
    type T = u64;
    fn id() -> Self::T {
        0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        *a + *b
    }
}

#[snippet = "Monoid-MIN"]
#[allow(dead_code)]
struct MIN;
#[snippet = "Monoid-MIN"]
impl Monoid for MIN {
    type T = usize;
    fn id() -> Self::T {
        (1 << 31) - 1
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        std::cmp::min(*a, *b)
    }
}