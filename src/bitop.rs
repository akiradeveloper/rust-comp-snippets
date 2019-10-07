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