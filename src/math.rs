/// https://github.com/hatoo/competitive-rust-snippets

#[allow(dead_code)]
/// v[n][k] = nCk / 2^n
fn nck_prob(n: usize) -> Vec<Vec<f64>> {
    let mut res = vec![vec![1.0]];

    for _ in 1..n {
        let mut v = Vec::new();
        {
            let last = res.last().unwrap();
            v.push(last.first().unwrap() / 2.0);
            for i in 0..last.len() - 1 {
                v.push((last[i] + last[i + 1]) / 2.0);
            }
            v.push(last.last().unwrap() / 2.0);
        }
        res.push(v);
    }
    res
}

#[snippet = "convex_hull_check"]
#[allow(dead_code)]
/// A check function for convex hull trick
pub fn convex_hull_check((a1, b1): (i64, i64), (a2, b2): (i64, i64), (a3, b3): (i64, i64)) -> bool {
    // Convert to f64 due to overflow
    (a2 as f64 - a1 as f64) * (b3 as f64 - b2 as f64)
        >= (b2 as f64 - b1 as f64) * (a3 as f64 - a2 as f64)
}

#[snippet = "XorShift"]
#[derive(Debug)]
#[allow(dead_code)]
pub struct Xorshift {
    seed: u64,
}

#[snippet = "XorShift"]
impl Xorshift {
    #[allow(dead_code)]
    pub fn new() -> Xorshift {
        Xorshift {
            seed: 0xf0fb588ca2196dac,
        }
    }

    #[allow(dead_code)]
    pub fn with_seed(seed: u64) -> Xorshift {
        Xorshift { seed: seed }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn next(&mut self) -> u64 {
        self.seed = self.seed ^ (self.seed << 13);
        self.seed = self.seed ^ (self.seed >> 7);
        self.seed = self.seed ^ (self.seed << 17);
        self.seed
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn rand(&mut self, m: u64) -> u64 {
        self.next() % m
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn randf(&mut self) -> f64 {
        use std::mem;
        const UPPER_MASK: u64 = 0x3FF0000000000000;
        const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
        let tmp = UPPER_MASK | (self.next() & LOWER_MASK);
        let result: f64 = unsafe { mem::transmute(tmp) };
        result - 1.0
    }
}

#[test]
fn test_xorshift_randf() {
    let mut rng = Xorshift::new();
    for _ in 0..1_000_000 {
        let f = rng.randf();
        assert!(f >= 0.0);
        assert!(f <= 1.0);
    }
}

#[cfg(test)]
use test::Bencher;

// #[bench]
// fn bench_xorshift_next(b: &mut Bencher) {
//     let mut rng = Xorshift::new();
//     b.iter(|| {
//         for _ in 0..1_000_000 {
//             rng.next();
//         }
//     });
// }

// #[bench]
// fn bench_xorshift_rand(b: &mut Bencher) {
//     let mut rng = Xorshift::new();
//     b.iter(|| {
//         for _ in 0..1_000_000 {
//             rng.rand(10000);
//         }
//     });
// }

// #[bench]
// fn bench_xorshift_randf(b: &mut Bencher) {
//     let mut rng = Xorshift::new();
//     b.iter(|| {
//         for _ in 0..1_000_000 {
//             rng.randf();
//         }
//     });
// }