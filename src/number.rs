/// https://github.com/hatoo/competitive-rust-snippets
 
use cargo_snippet::snippet;

#[snippet("into_digits")]
pub fn into_digits(b: i64, n: i64) -> Vec<i64> {
    let mut r = vec![];
    let mut n = n;
    while n>0 {
        let v = n % b;
        r.push(v);
        n -= v;
        n /= b;
    }
    r
}
#[test]
fn test_into_digits() {
    assert_eq!(into_digits(2, 6), [0,1,1]);
    assert_eq!(into_digits(10, 21), [1,2]);
}

#[snippet("PowTable")]
struct PowTable {
    dp: Vec<i64>,
}
#[snippet("PowTable")]
impl PowTable {
    pub fn new(x: i64, maxbit: usize, mo: i64) -> Self {
        let mut dp = vec![1];
        for i in 0..maxbit {
            let v = (dp[i] * x) % mo;
            dp.push(v);
        }
        PowTable {
            dp: dp
        }
    }
    pub fn pow(&self, k: usize) -> i64 {
        self.dp[k]
    }
}

#[snippet("prefix_decomposition")]
pub fn prefix_decomposition(b: i64, n: i64) -> Vec<(Vec<i64>, Vec<i64>)> {
    let digits = into_digits(b, n);
    let m = digits.len();
    let mut r = into_digits(b, n+1);
    let pow = PowTable::new(b, r.len(), std::i64::MAX);
    let from_digits = |x: &[i64]| {
        let m = x.len();
        let mut sum = 0;
        for i in 0..m {
            sum += pow.pow(i) * x[i];
        }
        sum
    };

    let mut res = vec![];
    for i in 0..digits.len() {
        if m-1-i > 0 && digits[m-1-i] == 0 { continue; }
        let rnum = from_digits(&r);
        let mut y = into_digits(b, rnum-1);
        let mut x = y.clone();
        for j in 0..std::cmp::min(i+1,x.len()) {
            x[j] = 0;
        }
        for _ in x.len()..m {
            x.push(0);
        }
        for _ in y.len()..m {
            y.push(0);
        }
        r = x.clone();
        res.push((x,y));
    }
    res.reverse();
    res
}
#[test]
fn test_prefix_decomposition() {
    let res = prefix_decomposition(10, 0);
    dbg!(&res);
    // let res = prefix_decomposition(10, 12345);
    // dbg!(&res);
    // let res = prefix_decomposition(10, 10101);
    // dbg!(&res);
    // let res = prefix_decomposition(2, 20);
    // dbg!(&res);
    // let res = prefix_decomposition(2, 100000000000000);
    // dbg!(&res);
}

#[snippet("gcd")]
#[allow(dead_code)]
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet("lcm")]
#[allow(dead_code)]
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

#[snippet("extgcd")]
#[doc = "ax+by=gcd(a,b) returns (gcd, x, y)"]
#[allow(dead_code)]
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

#[snippet("modinv")]
#[doc = "ay=1 (mod m) -> y=a^{-1}"]
pub fn modinv(a: i64, m: i64) -> i64 {
    let (_, x, _) = extgcd(a, m);
    (m + x % m) % m
}

#[snippet("ModComb")]
#[snippet("modpow")]
#[allow(dead_code)]
/// x ^ n % m
pub fn modpow(x: i64, n: i64, m: i64) -> i64 {
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

#[snippet("factorial")]
fn factorial(a: i64, p: i64) -> i64 {
    if a == 0 {
        return 1
    }
    let mut n = 1;
    let mut a = a;
    while a > 1 {
        n *= a;
        n %= p;
        a -= 1;
    }
    n
}

#[snippet("divisors")]
#[doc = "O(root N)"]
fn divisors(n: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut d = 1;
    while d*d<=n {
        if n%d == 0 {
            res.push(d);
            if d*d != n {
                res.push(n/d);
            }
        }
        d += 1;
    }
    res
}
#[test]
fn test_divisors() {
    let mut xs = divisors(36);
    xs.sort();
    assert_eq!(xs, [1,2,3,4,6,9,12,18,36]);
}