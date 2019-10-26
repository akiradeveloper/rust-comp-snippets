use crate::sequence01::BinarySearch;

#[derive(Clone)]
struct FID {
    n: usize,
    n_blocks: usize,
    blocks: Vec<u64>,
    block_rank1: Vec<usize>,
}
impl std::fmt::Debug for FID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let n = self.n;
        for i in 0..n {
            if self.access(i) {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        write!(f, "{}", s)
    }
}
impl FID {
    // O(1)
    fn popcount(x: u64) -> usize {
        u64::count_ones(x) as usize
    }
    // O(64)
    fn kpopi(x: u64, k: usize) -> usize {
        let mut x = x;
        let mut k = k;
        let mut ans = 64;
        for i in 0..64 {
            if x & (1<<i) > 0 {
                k -= 1;
            }
            if k == 0 {
                ans = i;
                break;
            }
        }
        ans
    }
    pub fn new(n: usize) -> FID {
        let n_blocks = (n>>6)+1;
        FID {
            n: n,
            n_blocks: (n>>6)+1,
            blocks: vec![0; n_blocks],
            // blocks[0,i)の1の数
            block_rank1: vec![0; n_blocks],
        }
    }
    pub fn set(&mut self, k: usize) {
        self.blocks[k>>6] |= 1<<(k&0b111111);
    }
    pub fn build(&mut self) {
        self.block_rank1[0] = 0;
        // 累積和
        for i in 1..self.n_blocks {
            self.block_rank1[i] = self.block_rank1[i-1] + Self::popcount(self.blocks[i-1]);
        }
    }
    pub fn access(&self, k: usize) -> bool {
        let x = self.blocks[k>>6] & 1<<(k&0b111111);
        if x > 0 { true } else { false }
    }
    #[doc = "count 1s in [0,k)"]
    pub fn rank1(&self, k: usize) -> usize {
        let mask = (1<<(k&0b111111)) - 1;
        let rest = self.blocks[k>>6] & mask;
        self.block_rank1[k>>6] + Self::popcount(rest)
    }
    #[doc = "count 0s in [0,k)"]
    pub fn rank0(&self, k: usize) -> usize {
        k - self.rank1(k)
    }
    pub fn rank(&self, b: bool, k: usize) -> usize {
        if b {
            self.rank1(k)
        } else {
            self.rank0(k)
        }
    }
    #[doc = "query the index of k-th 1 (0-indexed)"]
    pub fn select1(&self, k: usize) -> usize {
        let mut remaining = k+1; // remaining
        let bs = BinarySearch {
            lower: 0,
            upper: (self.n_blocks-1) as i64,
            p: |i: i64| {
                let i = i as usize;
                self.block_rank1[i] >= remaining
            },
        };
        let l = bs.lower_bound() as usize - 1;
        let count1 = self.block_rank1[l];
        remaining -= count1;
        assert!(remaining>0);
        (l<<6) | Self::kpopi(self.blocks[l], remaining)
    }
    #[doc = "query the index of k-th 0 (0-indexed)"]
    pub fn select0(&self, k: usize) -> usize {
        let mut remaining = k+1;
        let bs = BinarySearch {
            lower: 0,
            upper: (self.n_blocks-1) as i64,
            p: |i: i64| {
                let i = i as usize;
                (i<<6) - self.block_rank1[i] >= remaining
            }
        };
        let l = bs.lower_bound() as usize - 1;
        let count0 = (l<<6) - self.block_rank1[l];
        remaining -= count0;
        assert!(remaining>0);
        (l<<6) | Self::kpopi(!self.blocks[l], remaining)
    }
    pub fn select(&self, b: bool, k: usize) -> usize {
        if b {
            self.select1(k)
        } else {
            self.select0(k)
        }
    }
}

#[test]
fn test_kpopi() {
    let tests = vec![
        (0b11111, 3, 2),
        (0b10101, 2, 2),
        (0b10101, 3, 4),
    ];
    for (x, k, expected) in tests {
        assert_eq!(FID::kpopi(x, k), expected);
    }
}

#[test]
fn test_fid_rank() {
    use crate::xorshift::Xorshift;
    use std::collections::HashSet;
    let mut rand = Xorshift::new();
    let mut hs = HashSet::new();
    for i in 0..500 {
        let j = rand.rand(1000) as usize;
        hs.insert(j);
    }
    let mut v = vec![false;1000];
    let mut fid = FID::new(1000);
    for i in hs {
        v[i] = true;
        fid.set(i);
    }
    fid.build();

    let ref_rank0 = |k: usize| {
        let mut cnt = 0;
        for i in 0..k {
            if !v[i] {
                cnt+=1;
            }
        }
        cnt
    };
    let ref_rank1 = |k: usize| {
        let mut cnt=0;
        for i in 0..k {
            if v[i] {
                cnt+=1;
            }
        }
        cnt
    };
    for _ in 0..1000 {
        let k = rand.rand(1000) as usize;
        assert_eq!(fid.rank0(k), ref_rank0(k));
        assert_eq!(fid.rank1(k), ref_rank1(k));
    }
}

#[test]
fn test_fid_simple_select() {
    let x = 0b1011010010;
    let mut fid = FID::new(10);
    for i in 0..10 {
        if x & (1<<i) > 0 {
            fid.set(i);
        }
    }
    fid.build();

    assert_eq!(fid.select0(0), 0);
    assert_eq!(fid.select0(1), 2);
    assert_eq!(fid.select0(2), 3);
    assert_eq!(fid.select0(3), 5);
    assert_eq!(fid.select0(4), 8);

    assert_eq!(fid.select1(0), 1);
    assert_eq!(fid.select1(1), 4);
    assert_eq!(fid.select1(2), 6);
    assert_eq!(fid.select1(3), 7);
    assert_eq!(fid.select1(4), 9);
}

// test with randomly generated u64s
#[test]
fn test_fid_select() {
    use crate::xorshift::Xorshift;
    let mut rand = Xorshift::new();
    for _ in 0..100000 {
        let x: u64 = rand.next();
        let mut fid = FID::new(64);
        for i in 0..64 {
            if x & (1<<i) > 0 {
                fid.set(i);
            }
        }
        fid.build();

        let j = rand.rand(63) as usize;
        let mask = (1<<(j+1)) - 1;
        let y = x & mask;
        let count1 = y.count_ones() as usize;
        let count0 = (j+1)-count1;
        if x & (1<<j) > 0 {
            assert_eq!(fid.select1(count1-1), j);
        } else {
            assert_eq!(fid.select0(count0-1), j);
        }
    }
}

#[test]
fn test_fid_select_many_blocks() {
    let mut fid = FID::new(10000);
    fid.set(7777);
    fid.build();
    assert_eq!(fid.select1(0),7777);
    assert_eq!(fid.select0(7777),7778);
}

struct WM {
    mat: Vec<FID>,
    nzeros: Vec<usize>,
}
impl WM {
    pub fn new(xs: Vec<u64>) -> WM {
        let n = xs.len();
        let mut mat = vec![];
        let mut nzeros = vec![];
        let mut cur = xs;
        for i in 0..64 {
            let mid = 1<<(63-i);
            let mask = mid - 1;
            let mut b = vec![];
            let mut left = vec![];
            let mut right = vec![];
            for i in 0..n {
                let x = cur[i];
                if x >= mid {
                    right.push(x & mask);
                    b.push(true);
                } else {
                    left.push(x & mask);
                    b.push(false);
                }
            }
            nzeros.push(left.len());

            left.append(&mut right);
            cur = left;
            let mut fid = FID::new(n);
            for i in 0..n {
                if b[i] {
                    fid.set(i);
                }
            }
            mat.push(fid);
        }
        dbg!(&nzeros);
        dbg!(&mat);

        WM {
            mat: mat,
            nzeros: nzeros,
        }
    }

    pub fn access(&self, i: usize) -> u64 {
        unimplemented!();
    }
    pub fn rank(&self, x: u64, i: usize) -> usize {
        let mut s = 0;
        let mut e = i;
        for d in 0..64 {
            let fid = &self.mat[d];
            let b = x & (1<<(63-d)) > 0;
            s = fid.rank(b, s);
            e = fid.rank(b, e);
            if b {
                s += self.nzeros[d];
                e += self.nzeros[d];
            }
        }
        e-s
    }
}

#[test]
fn test_wm_rank() {
    let xs = vec![0,7,2,1,4,3,6,7,2,5,0,4,7,2,6,3];
    let wm = WM::new(xs);
    assert_eq!(wm.rank(2, 12), 2);
    assert_eq!(wm.rank(7, 12), 2);
    assert_eq!(wm.rank(7, 13), 3);
    assert_eq!(wm.rank(7, 15), 3);
}