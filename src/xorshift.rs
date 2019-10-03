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