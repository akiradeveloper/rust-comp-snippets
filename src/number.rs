/// https://github.com/hatoo/competitive-rust-snippets

#[snippet = "mod"]
#[allow(dead_code)]
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet = "mod"]
#[allow(dead_code)]
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[snippet = "mod"]
#[allow(dead_code)]
/// O(log n)
/// solve ax+by=gcd(a,b)
/// returns (gcd, x, y)
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extgcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}

#[test]
fn test_extgcg() {
    let (gcd, x, y) = extgcd(4, 11);
    assert_eq!(x, 3);
    assert_eq!(y, -1);
    assert_eq!(gcd, 1);
    // dbg!(gcd, x, y);
}

/// ax=b(mod m)
/// ay=1 (mod m) -> y=a^{-1}
/// x=yb (mod m)
pub fn mod_inverse(a: i64, m: i64) -> i64 {
    let (_, x, _) = extgcd(a, m);
    (m + x % m) % m
}

#[snippet = "mod"]
#[allow(dead_code)]
/// x ^ n % m
pub fn modpow(x: usize, n: usize, m: usize) -> usize {
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

fn factorial(a: usize) -> usize {
    if a == 0 {
        return 1
    }
    let mut n = 1;
    let mut a = a;
    while a > 1 {
        n *= a;
        a -= 1;
    }
    n
}
fn nPk(a: usize, b: usize) -> usize {
    assert!(a>=b);
    factorial(a) / factorial(a-b)
}
fn nCk(a: usize, b: usize) -> usize {
    factorial(a) / factorial(a-b) / factorial(b)
}
fn nHk(a: usize, b: usize) -> usize {
    nCk(a+b-1, b)
}

struct ModComb {
    po: Vec<usize>,
    inv: Vec<usize>,
    n: usize,
    p: usize,
}
impl ModComb {
    // O(N)
    fn initialize(ft: &mut Self) {
        let n = ft.n as usize;

        ft.po[0] = 1;
        for i in 1..n {
            ft.po[i] = (ft.po[i-1] * i) % ft.p;
        }
        ft.inv[n-1] = modpow(ft.po[n-1], ft.p-2, ft.p);
        for i in (0..n-1).rev() {
            ft.inv[i] = (ft.inv[i+1] * (i+1)) % ft.p;
        }
    }
    fn new(max_n: usize, p: usize) -> ModComb {
        let mut ft = ModComb {
            po: vec![0; max_n+1 as usize],
            inv: vec![0; max_n+1 as usize],
            n: max_n+1,
            p: p,
        };
        Self::initialize(&mut ft);
        ft
    }
    fn nCk(&self, n: usize, k: usize) -> usize {
        if n < k { return 0; }
        (self.nPk(n,k) * self.inv[k as usize]) % self.p 
    }
    fn nPk(&self, n: usize, k: usize) -> usize {
        if n < k { return 0; }
        self.po[n as usize] * self.inv[(n-k) as usize] % self.p
    }
    fn nHk(&self, n: usize, k: usize) -> usize {
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

#[should_panic]
#[test]
fn test_modcomb_segv() {
    let p = 11;
    let modcomb = ModComb::new(8, p);
    assert_eq!(modcomb.nPk(9, 3), 9);
}

// O(n log log n)
// compute the maximum factor for each number
// e.g 5 for 60 (2x2x3x5)
#[snippet = "factor_table"]
#[allow(dead_code)]
pub fn factor_table(max_n: usize) -> Vec<usize> {
    let mut res = vec![0; max_n + 1];
    // res[1] = 1;
    for i in 2..max_n + 1 {
        if res[i] == 0 {
            let mut j = i;
            while j <= max_n {
                res[j] = i;
                j += i;
            }
        }
    }

    res
}
#[test]
fn test_factor_table() {
    let n = 1000;
    let table = factor_table(n);
    for i in 2..n + 1 {
        assert_eq!(i % table[i], 0);
    }
}

// O(n log log n)
fn eratosthenes(n_max: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut v = vec![0; n_max+1];
    for i in 2..n_max+1 {
        if v[i] == 0 {
            res.push(i);
            let mut j = i;
            while j <= n_max {
                v[j] = i;
                j += i;
            }
        }
    }
    res
}

// O(root(n))
fn is_prime(n: usize) -> bool {
    let mut d = 1;
    // O(root(n))
    while d * d <= n {
        d += 1;
    }
    // O(root(n))
    let mut res = true;
    for i in 2..d {
        if n % i == 0 {
            res = false;
            break;
        }
    }
    res
}

// O(root(n))
fn root_int(n: usize) -> usize {
    let mut d = 1;
    while d * d <= n {
        d += 1;
    }
    d - 1
}
#[test]
fn test_root_int() {
    assert_eq!(root_int(35), 5);
    assert_eq!(root_int(36), 6);
    assert_eq!(root_int(37), 6);
}

fn prime_factors(n: usize) -> std::collections::HashMap<usize,usize> {
    let mut n = n;
    let mut m = std::collections::HashMap::new();
    for i in 2..root_int(n)+1 {
        while n % i == 0 {
            if !m.contains_key(&i) {
                m.insert(i, 0 as usize);
            }
            *m.get_mut(&i).unwrap() += 1;
            n /= i;
        }
    }
    if n != 1 {
        m.insert(n, 1);
    }
    m
}
#[test]
fn test_prime_factors() {
    dbg!(prime_factors(6));
    dbg!(prime_factors(12));
    dbg!(prime_factors(15));
}