/// https://github.com/hatoo/competitive-rust-snippets
 
#[snippet = "BITGeneric"]
#[allow(dead_code)]
/// Generic Binary Indexed Tree
pub struct BITGeneric<T: Clone, F: Fn(&mut T, &T) -> ()> {
    buf: Vec<T>,
    zero: T,
    f: F,
}

#[snippet = "BITGeneric"]
impl<T: Clone, F: Fn(&mut T, &T) -> ()> BITGeneric<T, F> {
    #[allow(dead_code)]
    pub fn new(n: usize, zero: &T, f: F) -> BITGeneric<T, F> {
        BITGeneric {
            buf: vec![zero.clone(); n + 1],
            zero: zero.clone(),
            f: f,
        }
    }

    #[allow(dead_code)]
    #[doc = "O(log N)"]
    pub fn sum(&self, i: usize) -> T {
        let mut i = i;
        let mut s = self.zero.clone();
        while i > 0 {
            (self.f)(&mut s, &self.buf[i]);
            i &= i - 1;
        }
        s
    }

    #[allow(dead_code)]
    #[doc = "O(log N)"]
    pub fn add(&mut self, i: usize, x: &T) {
        let mut i = i as i64;
        while i < self.buf.len() as i64 {
            let t = &mut self.buf[i as usize];
            (self.f)(t, x);
            i += i & -i;
        }
    }
}

#[test]
fn test_bit_generic() {
    let mut bit = BITGeneric::new(4, &0, |a: &mut usize, b: &usize| *a += b);
    bit.add(1, &1);
    bit.add(2, &2);
    bit.add(3, &3);
    bit.add(4, &4);
    dbg!(&bit.buf);
    assert_eq!(bit.sum(1), 1);
    assert_eq!(bit.sum(2), 3);
    assert_eq!(bit.sum(3), 6);
    assert_eq!(bit.sum(4), 10);
}

#[test]
fn test_bit_generic_vs_cumsum() {
    use rand::{Rng, SeedableRng, StdRng};
    let size = 1000;
    let mut cum_sum = vec![0; size + 1];
    let mut bit = BITGeneric::new(size, &0, |a: &mut usize, b: &usize| {
        *a += b;
    });

    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    let mut sum = 0;
    for i in 1..size + 1 {
        let x = rng.next_u32() as usize / (2 * size);
        sum += x;
        cum_sum[i] = sum;
        bit.add(i, &x);
    }

    for _ in 0..1000 {
        let i = rng.next_u32() as usize % size + 1;

        assert_eq!(bit.sum(i), cum_sum[i]);
    }
}

#[cfg(test)]
use test::Bencher;

// #[bench]
// /// Add and sum 10^5 times to get averaged time.
// /// This is typical scenario to solve a problem which is O(N log(N)) and N = 10^5.
// fn bench_bit_add_sum_100k(b: &mut Bencher) {
//     use rand::{Rng, SeedableRng, StdRng};

//     let size = 100_000;
//     let mut bit = BITGeneric::new(size, &0, |a: &mut usize, b: &usize| *a += b);
//     let mut rng = StdRng::from_seed(&[1, 2, 3]);

//     let bench_size = 100000;
//     let mut args = Vec::with_capacity(bench_size);

//     for _ in 0..bench_size {
//         let i = rng.next_u32() as usize % size + 1;
//         let x = rng.next_u32() as usize / bench_size;

//         args.push((i, x));
//     }

//     b.iter(|| {
//         for &(i, x) in &args {
//             bit.add(i, &x);
//             bit.sum(i);
//         }
//     });
// }

#[snippet = "BIT"]
#[allow(dead_code)]
/// Binary Indexed Tree of usize
pub struct BIT<T> {
    buf: Vec<T>,
}

#[snippet = "BIT"]
#[allow(dead_code)]
impl <T: Clone + Default + std::ops::AddAssign> BIT<T> {
    pub fn new(n: usize) -> BIT<T> {
        BIT {
            buf: vec![T::default(); n+1],
        }
    }

    pub fn sum(&self, i: usize) -> T {
        let mut i = i;
        let mut s = T::default();
        while i > 0 {
            s += self.buf[i].clone();
            i &= i - 1;
        }
        s
    }

    pub fn add(&mut self, i: usize, x: T) {
        let mut i = i as i64;
        while i < self.buf.len() as i64 {
            self.buf[i as usize] += x.clone();
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