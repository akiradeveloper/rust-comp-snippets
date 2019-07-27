/// https://github.com/hatoo/competitive-rust-snippets

#[snippet = "mod"]
#[allow(dead_code)]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet = "mod"]
#[allow(dead_code)]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[snippet = "mod"]
#[allow(dead_code)]
/// (gcd, x, y)
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extgcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}

#[snippet = "mod"]
#[allow(dead_code)]
/// x ^ n % m
pub fn modpow(x: u64, n: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}

#[test]
fn test_modpow() {
    let m = 1_000_000_007;
    let x = 1234;
    let mut t = 1;
    for i in 0..1000 {
        assert_eq!(modpow(x, i, m), t);
        t = t * x % m;
    }
}

struct ModComb {
    po: Vec<u64>,
    inv: Vec<u64>,
    n: u64,
    p: u64,
}
impl ModComb {
    // O(N)
    fn initialize(ft: &mut Self) {
        let n = ft.n as usize;

        ft.po[0] = 1;
        for i in 1..n {
            ft.po[i] = (ft.po[i-1] * i as u64) % ft.p;
        }
        ft.inv[n-1] = modpow(ft.po[n-1], ft.p-2, ft.p);
        for i in (0..n-1).rev() {
            ft.inv[i] = (ft.inv[i+1] * (i as u64 +1)) % ft.p;
        }
    }
    fn new(max_n: u64, p: u64) -> Self {
        // nHk needs twice the length of the max_n.
        // since it doesn't affect the computational order, we always double the number for convenience.
        let mut ft = Self {
            po: vec![0; 2 * max_n as usize],
            inv: vec![0; 2 * max_n as usize],
            n: 2 * max_n,
            p: p,
        };
        Self::initialize(&mut ft);
        ft
    }
    fn nCk(&self, n: u64, k: u64) -> u64 {
        if n < k { return 0; }
        (self.nPk(n,k) * self.inv[k as usize]) % self.p 
    }
    fn nPk(&self, n: u64, k: u64) -> u64 {
        if n < k { return 0; }
        self.po[n as usize] * self.inv[(n-k) as usize] % self.p
    }
    fn nHk(&self, n: u64, k: u64) -> u64 {
        if n==0 && k==0 { return 1 }
        self.nCk(n+k-1,k)
    }
}

#[test]
fn test_modcomb() {
    let p = 1_000_000_007;
    let com = ModComb::new(200000, p);

    assert_eq!(com.nHk(10, 2), 55);
    assert_eq!(com.nHk(10, 3), 220);
    assert_eq!(com.nHk(10, 4), 715);
    assert_eq!(com.nHk(400, 296), 546898535);
    assert_eq!(com.nHk(100000, 100000), 939733670);
}