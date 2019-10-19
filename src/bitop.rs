#[snippet = "submasks"]
struct SubMasks {
    mask: i64,
    smask: i64,
}
#[snippet = "submasks"]
impl Iterator for SubMasks {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        let old = self.smask;
        if old == 0 {
            return None
        }
        self.smask = (self.smask-1) & self.mask;
        return Some(old)
    }
}
#[snippet = "submasks"]
#[doc = "iterate all sub masks except 0. O(2^K) where K is the number of bit positions."]
fn submasks(mask: i64) -> SubMasks {
    SubMasks {
        mask: mask,
        smask: mask,
    }
}
#[test]
fn test_submasks() {
    let mut v = vec![];
    for mask in submasks(13) {
        v.push(mask);
    }
    v.sort();
    assert_eq!(v, [1,4,5,8,9,12,13]);
}

#[snippet = "bitpos"]
struct BitPos {
    curpos: usize,
    x: i64,
}
#[snippet = "bitpos"]
impl Iterator for BitPos {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            return None;
        }
        if self.x & 1 > 0 {
            let old = self.curpos;
            self.curpos += 1;
            self.x >>= 1;
            return Some(old);
        }
        let n = i64::trailing_zeros(self.x) as usize;
        self.curpos += n;
        self.x >>= n;
        let old = self.curpos;
        self.curpos += 1;
        self.x >>= 1;
        return Some(old);
    }
}
#[snippet = "bitpos"]
#[doc = "iterate the bit positions in an integer. O(K) where K is the number of bit positions."]
fn bitpos(x: i64) -> BitPos {
    BitPos {
        curpos: 0,
        x: x,
    }
}
#[test]
fn test_bitpos() {
    let mut v = vec![];
    for i in bitpos(0b11000001) {
        v.push(i);
    }
    assert_eq!(v, [0,6,7]);
}

#[snippet = "next_boundary"]
fn next_boundary(x: i64, p: i64) -> i64 {
    let i = (x+p-1) / p;
    p*i
}
#[test]
fn test_next_boundary() {
    assert_eq!(next_boundary(0, 3), 0);
    assert_eq!(next_boundary(1, 3), 3);
    assert_eq!(next_boundary(2, 3), 3);
    assert_eq!(next_boundary(3, 3), 3);
    assert_eq!(next_boundary(4, 3), 6);
}

#[snippet = "lsb"]
fn lsb(x: i64) -> i64 {
    x & (-x)
}
#[test]
fn test_lsb() {
    assert_eq!(lsb(3), 1);
    assert_eq!(lsb(5), 1);
    assert_eq!(lsb(6), 2);
    assert_eq!(lsb(4), 4);
}

#[snippet = "bin_digits"]
fn bin_digits(n: i64) -> Vec<bool> {
    if n == 0 { return vec![]; }
    let logN = (n as f64).log2().floor() as usize;
    // dbg!(logN);
    let mut res = vec![false; logN+1];
    let mut n = n;
    for k in (0..logN+1).rev() {
        // dbg!(n, 1<<k);
        if n >= 1<<k {
            // dbg!(k);
            res[k] = true;
            n -= (1<<k);
        }
    }
    res
}
#[test]
fn test_bin_digits() {
    assert_eq!(bin_digits(0), []);
    assert_eq!(bin_digits(3), [true,true]);
    assert_eq!(bin_digits(7), [true,true,true]);
    assert_eq!(bin_digits(6), [false,true,true]);
    assert_eq!(bin_digits(10), [false,true,false,true]);
    assert_eq!(bin_digits(16), [false,false,false,false,true]);
}


#[snippet = "range_decomposition"]
#[doc = "decompose a number into range of form [X000...,X111...]"]
fn range_decomposition(x: i64) -> Vec<(i64,i64)> {
    let mut res = vec![];
    res
}
#[test]
fn test_range_decomposition() {
    let mut res = range_decomposition(0b10101);
    res.sort();
    assert_eq!(res, [(0b00000,0b1111),(0b10000,0b10011),(0b10100,0b10100),(0b10101,0b10101)]);
}
