use cargo_snippet::snippet;

#[snippet("submasks")]
fn submasks(mask: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut smask = mask;
    while smask > 0 {
        res.push(smask);
        smask = (smask-1) & mask;
    }
    res.reverse();
    return res
}
#[test]
fn test_submasks() {
    assert_eq!(submasks(13), [1,4,5,8,9,12,13]);
}

#[snippet("bitpos")]
fn bitpos(x: i64) -> Vec<usize> {
    let mut p = vec![];
    let mut cur = x;
    for i in 0.. {
        if x & (1<<i) > 0 {
            p.push(i)
        }
        cur >>= 1;
        if cur == 0 { break }
    }
    p
}
#[test]
fn test_bitpos() {
    assert_eq!(bitpos(0b11000001), [0,6,7]);
}

#[snippet("BitMask")]
struct BitMask {
    x: i64,
}
#[snippet("BitMask")]
impl BitMask {
    pub fn new(x: i64) -> Self {
        BitMask { x: x }
    }
    pub fn check(&self, k: usize) -> bool {
        self.x & (1<<k) > 0
    }
    pub fn on(&self, k: usize) -> i64 {
        self.x | (1<<k)
    }
    pub fn off(&self, k: usize) -> i64 {
        let mask = !(1<<k);
        self.x & mask
    }
    pub fn flip(&self, k: usize) -> i64 {
        self.x ^ (1<<k)
    }
    #[doc = "0b1110 -> 0b10"]
    pub fn lsb(&self) -> i64 {
        let x = self.x;
        x & -x
    }
}

#[test]
fn test_bitmask() {
    let x = BitMask::new(0b1101);
    assert_eq!(x.flip(2), 0b1001);
    assert_eq!(x.flip(1), 0b1111);
    assert_eq!(x.off(0), 0b1100);
    assert_eq!(x.on(0), 0b1101);
    assert_eq!(x.on(1), 0b1111);
    assert_eq!(x.on(2), 0b1101);
}

#[test]
fn test_lsb() {
    assert_eq!(BitMask::new(3).lsb(), 1);
    assert_eq!(BitMask::new(5).lsb(), 1);
    assert_eq!(BitMask::new(6).lsb(), 2);
    assert_eq!(BitMask::new(4).lsb(), 4);
}