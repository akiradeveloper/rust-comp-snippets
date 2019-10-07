/// https://github.com/hatoo/competitive-rust-snippets

#[snippet = "gcd"]
#[allow(dead_code)]
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet = "lcm"]
#[allow(dead_code)]
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

#[doc = "ax+by=gcd(a,b) returns (gcd, x, y)"]
#[snippet = "extgcd"]
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

/// ax=b(mod m)
/// ay=1 (mod m) -> y=a^{-1}
/// x=yb (mod m)
#[snippet = "mod_inverse"]
pub fn mod_inverse(a: i64, m: i64) -> i64 {
    let (_, x, _) = extgcd(a, m);
    (m + x % m) % m
}

#[snippet = "ModComb"]
#[snippet = "modpow"]
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

#[snippet = "factorial"]
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

// Knuth's algorithm
#[snippet = "nCk"]
fn nCk(a: i64, b: i64) -> i64 {
    if a < b { return 0; }
    let mut a = a;
    let mut r = 1;
    for d in 1..b+1 {
        r *= a;
        a -= 1;
        r /= d;
    }
    r
}
#[test]
fn test_knuth_nCk() {
    assert_eq!(nCk(1,2), 0);
    assert_eq!(nCk(5,0), 1);
    assert_eq!(nCk(5,1), 5);
    assert_eq!(nCk(5,2), 10);
    assert_eq!(nCk(5,5), 1);
}

#[snippet = "comb_table"]
#[doc = "Pascal's triangle. O(N^2)"]
fn comb_table(n_max: usize) -> Vec<Vec<i64>> {
    let mut dp = vec![vec![0; n_max+1]; n_max+1];
    for i in 0..n_max {
        for j in 0..i+1 {
            if j==0 || j==i {
                dp[i][j] = 1;
            } else {
                dp[i][j] = dp[i-1][j-1] + dp[i-1][j];
            }
        }
    }
    dp
}
#[test]
fn test_comb_table() {
    let nCk = comb_table(50);
    assert_eq!(nCk[5][0], 1);
    assert_eq!(nCk[5][1], 5);
    assert_eq!(nCk[5][2], 10);
    assert_eq!(nCk[5][5], 1);
}

#[snippet = "ModComb"]
struct ModComb {
    fact: Vec<i64>,
    fact_inv: Vec<i64>,
    n: usize,
    p: i64,
}
#[snippet = "ModComb"]
impl ModComb {
    fn initialize(ft: &mut Self) {
        let n = ft.n;

        ft.fact[0] = 1;
        for i in 1..n {
            ft.fact[i] = (ft.fact[i-1] * i as i64) % ft.p;
        }
        ft.fact_inv[n-1] = modpow(ft.fact[n-1], ft.p-2, ft.p);
        for i in (0..n-1).rev() {
            ft.fact_inv[i] = (ft.fact_inv[i+1] * (i+1) as i64) % ft.p;
        }
    }
    #[doc = "O(N)"]
    fn new(max_n: usize, p: i64) -> ModComb {
        let mut ft = ModComb {
            fact: vec![0; max_n+1],
            fact_inv: vec![0; max_n+1],
            n: max_n+1,
            p: p,
        };
        Self::initialize(&mut ft);
        ft
    }
    fn fact(&self, n: usize) -> i64 {
        self.fact[n]
    }
    fn nCk(&self, n: i64, k: i64) -> i64 {
        if n < k { return 0; }
        (self.nPk(n, k) * self.fact_inv[k as usize]) % self.p 
    }
    fn nPk(&self, n: i64, k: i64) -> i64 {
        if n < k { return 0; }
        self.fact[n as usize] * self.fact_inv[(n-k) as usize] % self.p
    }
    fn nHk(&self, n: i64, k: i64) -> i64 {
        if n==0 && k==0 { return 1 }
        self.nCk(n+k-1, k)
    }
    // 区別できるnを区別出来ないkに分割
    fn nSk(&self, n: i64, k: i64) -> i64 {
        if n < k { return 0; }
        let mut res = 0;
        for i in 0..k+1 {
            let v = self.nCk(k, i) * modpow(i, n, self.p) % self.p;
            if (k - i) % 2 == 1 { // odd
                res = (res + self.p - v) % self.p;
            } else { // 
                res = (res + v) % self.p;
            }
        }
        return res * self.fact_inv[k as usize] % self.p;
    }
    fn nBk(&self, n: i64, k: i64) -> i64 {
        0
    }
}

#[test]
fn test_modcomb_fact() {
    let p = 1_000_000_007;
    let com = ModComb::new(200000, p);
    assert_eq!(com.fact(3), 6);
    assert_eq!(com.fact(5), 120);
}

#[test]
fn test_modcomb_nHk() {
    let p = 1_000_000_007;
    let com = ModComb::new(200000, p);

    assert_eq!(com.nHk(10, 2), 55);
    assert_eq!(com.nHk(10, 3), 220);
    assert_eq!(com.nHk(10, 4), 715);
    assert_eq!(com.nHk(400, 296), 546898535);
    assert_eq!(com.nHk(100000, 100000), 939733670);
}

#[test]
fn test_modcomb_nSk() {
    let p = 1_000_000_007;
    let com = ModComb::new(200000, p);
    assert_eq!(com.nSk(4, 3) * com.fact(3) % p, 36);
    assert_eq!(com.nSk(10, 3) * com.fact(3) % p, 55980);
    assert_eq!(com.nSk(100, 100) * com.fact(100) % p, 437918130);
}

#[should_panic]
#[test]
fn test_modcomb_mem_bound() {
    let p = 11;
    let modcomb = ModComb::new(8, p);
    assert_eq!(modcomb.nPk(9, 3), 9);
}

#[snippet = "factor_table"]
#[doc = "compute the maximum factor for each number. O(N log log N)"]
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

#[snippet = "eratosthenes"]
#[doc = "O(N log log N)"]
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

#[snippet = "divisors"]
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


#[snippet = "is_prime"]
#[doc = "O(root N)"]
fn is_prime(n: i64) -> bool {
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

// O(root(N))
#[snippet = "prime_factors"]
fn prime_factors(n: i64) -> std::collections::HashMap<i64, i64> {
    fn root_int(n: i64) -> i64 {
        let mut d = 1;
        while d * d <= n {
            d += 1;
        }
        d - 1
    }
    let mut n = n;
    let mut m = std::collections::HashMap::new();
    for i in 2..root_int(n)+1 {
        while n % i == 0 {
            if !m.contains_key(&i) {
                m.insert(i, 0);
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

#[doc = "how many prime p found in n!"]
fn prime_count(n: i64, p: i64, mo: i64) -> i64 {
    let mut acc = 0;
    for k in 1.. {
        let d: i64 = modpow(p, k, std::i64::MAX);
        if d > n { break; }
        acc += n / d;
        acc %= mo;
    }
    acc
}
#[test]
fn test_prime_count() {
    assert_eq!(prime_count(4, 2, std::i64::MAX), 3);
    assert_eq!(prime_count(6, 2, std::i64::MAX), 4);
    assert_eq!(prime_count(6, 3, std::i64::MAX), 2);
    assert_eq!(prime_count(6, 5, std::i64::MAX), 1);
}

#[snippet = "bin_digits"]
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
    assert_eq!(bin_digits(0), []);
    assert_eq!(bin_digits(3), [true,true]);
    assert_eq!(bin_digits(7), [true,true,true]);
    assert_eq!(bin_digits(6), [false,true,true]);
    assert_eq!(bin_digits(10), [false,true,false,true]);
    assert_eq!(bin_digits(16), [false,false,false,false,true]);
}

#[snippet = "partition_dp"]
#[allow(dead_code)]
/// dp[i][j] = j th partition number of i
pub fn partition_dp(n: usize, m: usize, p: i64) -> Vec<Vec<i64>> {
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..m + 1 {
        dp[0][i] = 1;
    }
    for i in 1..n + 1 {
        for j in 1..m + 1 {
            if i >= j {
                dp[i][j] = (dp[i - j][j] + dp[i][j - 1]) % p;
            } else {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }
    dp
}
#[test]
fn test_partition_dp() {
    const M: i64 = 1000000007;
    let dp = partition_dp(100, 50, M);
    assert_eq!(dp[4][3], 4);
    assert_eq!(dp[5][4], 6);
}