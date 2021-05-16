/// https://github.com/hatoo/competitive-rust-snippets
 
use cargo_snippet::snippet;

/// フェニック木
/// 
/// a1,a2,...,aN (1-indexed)
/// の数列に対して、
/// add i x: aiにxを足す
/// sum k: a1+a2+...+akを求める。
/// 
/// 計算量:
/// ともにO(logN)
/// 
/// sumがa1からの和を調べる特殊性から、
/// 2Nの容量を必要としない。

#[snippet("BIT")]
#[allow(dead_code)]
pub struct BIT<T> {
    buf: Vec<T>,
}

#[snippet("BIT")]
#[allow(dead_code)]
impl <T: Clone + Default + std::ops::AddAssign> BIT<T> {
    pub fn new(n: usize) -> BIT<T> {
        BIT {
            buf: vec![T::default(); n+1],
        }
    }

    pub fn sum(&self, i: usize) -> T {
        let mut i = i as i64;
        let mut s = T::default();
        while i > 0 {
            s += self.buf[i as usize].clone();
            // i&-iで一番下のビットを計算する。
            // 5 -> 4 -> 0
            i -= i & -i;
            // i &= i - 1;
        }
        s
    }

    pub fn add(&mut self, i: usize, x: T) {
        let mut i = i as i64;
        while i < self.buf.len() as i64 {
            self.buf[i as usize] += x.clone();
            // 5 -> 6 -> 8
            i += i & -i;
        }
    }
}

#[test]
fn test_bit_vs_cumsum() {
    use rand::{Rng, SeedableRng, StdRng};
    let size = 1000;
    let mut cum_sum = vec![0; size + 1];
    let mut bit = BIT::new(size);

    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    let mut sum = 0;
    for i in 1..size + 1 {
        let x = rng.next_u32() as usize / (2 * size);
        sum += x;
        cum_sum[i] = sum;
        bit.add(i, x);
    }

    for _ in 0..1000 {
        let i = rng.next_u32() as usize % size + 1;
        assert_eq!(bit.sum(i), cum_sum[i]);
    }
}

#[test]
fn test_bit() {
    let mut bit = BIT::new(4);
    bit.add(1, 1);
    bit.add(2, 2);
    bit.add(3, 3);
    bit.add(4, 4);
    dbg!(&bit.buf);
    assert_eq!(bit.sum(1), 1);
    assert_eq!(bit.sum(2), 3);
    assert_eq!(bit.sum(3), 6);
    assert_eq!(bit.sum(4), 10);
}