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



#[snippet("bin_digits")]
#[doc = "O(|A|)"]
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
    assert_eq!(bin_digits(0).len(), 0);
    assert_eq!(bin_digits(3), [true,true]);
    assert_eq!(bin_digits(7), [true,true,true]);
    assert_eq!(bin_digits(6), [false,true,true]);
    assert_eq!(bin_digits(10), [false,true,false,true]);
    assert_eq!(bin_digits(16), [false,false,false,false,true]);
}


#[snippet("range_decomposition")]
#[doc = "decompose a number into range of form [X000...,X111...]"]
fn range_decomposition(x: i64) -> Vec<(i64,i64)> {
    let mut res = vec![(x,x)];
    let mut cur = x;
    let bd = bin_digits(x);
    for i in 0..bd.len() {
        if bd[i] {
            let last = cur-1;
            cur -= (1<<i);
            res.push((cur,last));
        }
    }
    res.sort();
    res
}
#[test]
fn test_range_decomposition() {
    let mut res = range_decomposition(0b10101);
    assert_eq!(res, [(0b00000,0b1111),(0b10000,0b10011),(0b10100,0b10100),(0b10101,0b10101)]);
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