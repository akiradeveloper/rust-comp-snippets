#[snippet = "partition_dp"]
#[doc = "O(nm)"]
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

// Knuth's algorithm
#[snippet = "nCk"]
#[doc = "referential impl. only for small numbers. O(b)"]
fn nCk(a: i64, b: i64) -> i64 {
    if a < b { return 0; }
    let mut a = a;
    let mut r = 1;
    for d in 1..b+1 { // O(b)
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

#[doc = "referential impl. O(n)"]
fn catalan(n: i64) -> i64 {
    nCk(2*n,n) / (n+1)
}
#[test]
fn test_catalan() {
    assert_eq!(catalan(1), 1);
    assert_eq!(catalan(2), 2);
    assert_eq!(catalan(3), 5);
}

use crate::modint::Mod;
#[doc = "O(N)"]
fn catalan_table(n_max: usize) -> Vec<Mod> {
    let mut tbl: Vec<Mod> = vec![0.into(); n_max+1];
    let mut n1: Mod = 1.into();
    let mut n2: Mod = 1.into();
    let mut nf: Mod = 1.into();
    for i in 0..n_max+1 {
        let cur = n2 / (nf * n1);
        tbl[i] = cur;
        let i = i as i64;
        n2 *= i * 2 + 1;
        n2 *= i * 2 + 2;
        nf *= i + 1;
        n1 *= i + 2;
    }
    tbl
}
#[test]
fn test_catalan_table() {
    let tbl = catalan_table(20);
    for i in 0..20 {
        assert_eq!(tbl[i], catalan(i as i64).into());
    }
}

use crate::number::modpow;
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
    #[doc = "choose k numbers from 1..n"]
    fn nCk(&self, n: i64, k: i64) -> i64 {
        if n < k { return 0; }
        (self.nPk(n, k) * self.fact_inv[k as usize]) % self.p 
    }
    fn nPk(&self, n: i64, k: i64) -> i64 {
        if n < k { return 0; }
        self.fact[n as usize] * self.fact_inv[(n-k) as usize] % self.p
    }
    #[doc = "split k into n number as x1+x2+...xn=k"]
    fn nHk(&self, n: i64, k: i64) -> i64 {
        if n==0 && k==0 { return 1 }
        self.nCk(n+k-1, k)
    }
    #[doc = "put n balls into k different boxes. In case of n=3,k+2 [[1,2],[3]]==[[3],[1,2]]"]
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