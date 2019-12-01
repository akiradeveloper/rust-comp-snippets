/// https://github.com/hatoo/competitive-rust-snippets
 
#[snippet = "ceil"]
fn ceil(x: i64, a: i64) -> i64 {
    let y = x/a;
    if x%a == 0 {
        a*y
    } else {
        a*(y+1)
    }
}
#[snippet = "floor"]
fn floor(x: i64, a: i64) -> i64 {
    let y = x/a;
    a*y
}

#[snippet = "into_digits"]
fn into_digits(b: i64, n: i64) -> Vec<i64> {
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


