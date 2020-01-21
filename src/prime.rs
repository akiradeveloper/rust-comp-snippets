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

#[doc = "O(root(N))"]
#[snippet = "prime_decomposition"]
fn prime_decomposition(n: i64) -> std::collections::HashMap<i64, i64> {
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
fn test_prime_decomposition() {
    dbg!(prime_decomposition(6));
    dbg!(prime_decomposition(12));
    dbg!(prime_decomposition(15));
}

#[doc = "O(n). how many prime p found in n!"]
#[snippet = "factorial_prime_count"]
fn factorial_prime_count(n: i64, p: i64, mo: i64) -> i64 {
    let mut acc = 0;
    for k in 1.. {
        let mut d = 1;
        for _  in 0..k { d *= p; }
        if d > n { break; }
        acc += n / d;
        acc %= mo;
    }
    acc
}
#[test]
fn test_prime_count() {
    assert_eq!(factorial_prime_count(4, 2, std::i64::MAX), 3);
    assert_eq!(factorial_prime_count(6, 2, std::i64::MAX), 4);
    assert_eq!(factorial_prime_count(6, 3, std::i64::MAX), 2);
    assert_eq!(factorial_prime_count(6, 5, std::i64::MAX), 1);
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
    for i in 2..n+1 {
        let max_factor = table[i];
        assert_eq!(i % max_factor, 0);
    }
}

#[doc = "O(N log log N)"]
#[snippet = "eratosthenes"]
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