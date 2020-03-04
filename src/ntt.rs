use crate::number::{modinv, modpow};

#[snippet = "NTT"]
struct NTT {
    pub mo: i64,
}
#[snippet = "NTT"]
impl NTT {
    pub fn new(mo: i64) -> NTT {
        NTT {
            mo: mo,
        }
    }
    fn _ntt(&self, a: &mut [i64], n: usize, inverse: bool) {
        let g = 3;
        let mut h = modpow(g, (self.mo-1)/n as i64, self.mo);
        if inverse {
            h = modinv(h, self.mo);
        }

        let mut i = 0;
        for j in 1..n-1 {
            let mut k = n>>1;
            loop {
                i ^= k;
                if k > i {
                    k >>= 1;
                } else {
                    break;
                }
            }
            if j < i {
                let tmp = a[i];
                a[i] = a[j];
                a[j] = tmp;
            }
        }

        let mut m = 1;
        while m < n {
            let m2 = m * 2;
            let base = modpow(h, (n/m2) as i64, self.mo);
            let mut w = 1;
            for x in 0..m {
                let mut s = x;
                while s < n {
                    let u = a[s];
                    let d = (a[s+m] * w) % self.mo;
                    a[s] = u + d;
                    if a[s] >= self.mo { a[s] -= self.mo; }
                    a[s+m] = u - d;
                    if a[s+m] < 0 { a[s+m] += self.mo; }
                    s += m2;
                }
                w = (w * base) % self.mo;
            }
            m *= 2;
        }
        for i in 0..n {
            if a[i] < 0 {
                a[i] += self.mo;
            }
        }
    }
    fn ntt(&self, a: &mut [i64], n: usize) {
        self._ntt(a, n, false);
    }
    fn intt(&self, a: &mut [i64], n: usize) {
        self._ntt(a, n, true);
        let n_inv = modinv(a.len() as i64, self.mo);
        for i in 0..n {
            a[i] = (a[i] * n_inv) % self.mo;
        }
    }
    pub fn convolve(&self, a: &[i64], b: &[i64]) -> Vec<i64> {
        let mut a = a.to_vec();
        let mut b = b.to_vec();
        
        let mut n = 1;
        while n < a.len() + b.len() {
            n <<= 1;
        }
        a.resize(n, 0);
        b.resize(n, 0);

        self.ntt(&mut a, n);
        self.ntt(&mut b, n);

        let mut c = vec![0;n];
        for i in 0..n {
            c[i] = (a[i] * b[i]) % self.mo;
        }

        self.intt(&mut c, n);
        c
    }
}
#[snippet = "garner"]
#[doc = "compute minimum x from a list of x % m[i] = r[i]"]
pub fn garner(mr: Vec<(i64,i64)>, mo: i64) -> i64 {
    let mut mr = mr;
    mr.push((mo, 0));
    let mut coef = vec![1; mr.len()];
    let mut constants = vec![0; mr.len()];
    for i in 0..mr.len() - 1 {
        let v = (mr[i].1 + mr[i].0 - constants[i]) * modinv(coef[i], mr[i].0) % mr[i].0;
        for j in i + 1..mr.len() {
            constants[j] += coef[j] * v;
            constants[j] %= mr[j].0;
            coef[j] *= mr[i].0;
            coef[j] %= mr[j].0;
        }
    }
    constants[mr.len() - 1]
}
pub fn ntt_multiply_naive(a: &[i64], b: &[i64], mo: i64) -> Vec<i64> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    let n = a.len();
    let m = b.len();
    for i in 0..n {
        a[i] %= mo;
    }
    for i in 0..m {
        b[i] %= mo;
    }
    let ntt1 = NTT::new(167772161);
    let ntt2 = NTT::new(469762049);
    let ntt3 = NTT::new(1224736769);

    let x = ntt1.convolve(&a, &b);
    let y = ntt2.convolve(&a, &b);
    let z = ntt3.convolve(&a, &b);
    
    let L = x.len();
    let mut res = vec![0;L];
    for i in 0..L {
        let mr = vec![
            (ntt1.mo, x[i]),
            (ntt2.mo, y[i]),
            (ntt3.mo, z[i]),
        ];
        res[i] = garner(mr, mo);
    }
    res.truncate(n+m-1);
    res
}
#[snippet = "NTT"]
pub fn ntt_multiply(a: &[i64], b: &[i64], mo: i64) -> Vec<i64> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    let n = a.len();
    let m = b.len();
    for i in 0..n {
        a[i] %= mo;
    }
    for i in 0..m {
        b[i] %= mo;
    }
    let ntt1 = NTT::new(167772161);
    let ntt2 = NTT::new(469762049);
    let ntt3 = NTT::new(1224736769);

    let x = ntt1.convolve(&a, &b);
    let y = ntt2.convolve(&a, &b);
    let z = ntt3.convolve(&a, &b);

    let m1 = ntt1.mo;
    let m2 = ntt2.mo;
    let m3 = ntt3.mo;
    let m1_inv_m2 = modinv(m1, m2);
    let m12_inv_m3 = modinv(m1 * m2, m3);
    let m12_mod = (m1 * m2) % mo;

    let L = x.len();
    let mut res = vec![0; L];
    for i in 0..L {
        let mut v1 = (y[i] - x[i]) * m1_inv_m2;
        v1 %= m2;
        if v1 < 0 { v1 += m2; }

        let mut v2 = (z[i] - (x[i] + m1*v1) % m3) * m12_inv_m3;
        v2 %= m3;
        if v2 < 0 { v2 += m3; }

        let mut const3 = (x[i] + m1*v1 + m12_mod * v2) % mo;
        if const3 < 0 { const3 += mo; }
        res[i] = const3;
    }
    res.truncate(n+m-1);
    res
}

#[test]
fn test_ntt_multiply() {
    fn ten(n: usize) -> i64 {
        let mut res = 1;
        for _ in 0..n {
            res *= 10;
        }
        res
    }
    let mut x = vec![];
    for i in 0..10 {
        x.push(ten(8) + i as i64);
    }
    let y = x.clone();

    let t1 = vec![
        930000007, 60000000, 390000001, 920000004,
		650000003, 580000006, 710000014, 40000021,
		570000042, 300000064, 370000109, 240000144,
		910000175, 380000187, 650000193, 720000185,
        590000162, 260000123, 730000074];

    assert_eq!(ntt_multiply_naive(&x, &y, ten(9)+7), t1);
    assert_eq!(ntt_multiply(&x, &y, ten(9)+7), t1);
}

#[snippet = "ntt_ext"]
pub mod ntt_ext {
    pub trait ModI:
        Sized
        + PartialEq
        + Copy
        + std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + std::ops::Mul<Output = Self>
        + std::ops::Div<Output = Self>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::default::Default
        + std::fmt::Display
        + std::fmt::Debug
    {
        fn m() -> u64;
        fn new(x: u64) -> Self;
        fn pow(self, n: u64) -> Self;
        fn inv(&self) -> Self;
    }
    macro_rules! define_modint {
        ($n:ident,$m:expr) => {
            #[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
            struct $n(u64);

            #[allow(dead_code)]
            impl ModI for $n {
                fn m() -> u64 {
                    $m
                }
                fn new(x: u64) -> $n {
                    $n(x % $m)
                }

                fn pow(self, mut n: u64) -> $n {
                    let mut ret = $n::new(1);
                    let mut base = self;
                    while n > 0 {
                        if n & 1 == 1 {
                            ret *= base;
                        }
                        base *= base;
                        n >>= 1;
                    }
                    ret
                }

                fn inv(&self) -> $n {
                    self.pow($m - 2)
                }
            }

            impl std::default::Default for $n {
                fn default() -> $n {
                    $n::new(0u64)
                }
            }

            impl std::convert::From<u64> for $n {
                fn from(x: u64) -> $n {
                    $n::new(x)
                }
            }

            // Binary operator
            impl std::ops::Add for $n {
                type Output = $n;
                fn add(self, rhs: $n) -> Self::Output {
                    $n::new(self.0 + rhs.0)
                }
            }

            impl std::ops::Sub for $n {
                type Output = $n;
                fn sub(self, rhs: $n) -> Self::Output {
                    if self.0 >= rhs.0 {
                        $n::new(self.0 - rhs.0)
                    } else {
                        $n::new($m - rhs.0 + self.0)
                    }
                }
            }

            impl std::ops::Mul for $n {
                type Output = $n;
                fn mul(self, rhs: $n) -> Self::Output {
                    $n::new(self.0 * rhs.0)
                }
            }

            impl std::ops::Div for $n {
                type Output = $n;
                fn div(self, rhs: $n) -> Self::Output {
                    $n::new(self.0 / rhs.0)
                }
            }

            // Assign
            impl std::ops::AddAssign for $n {
                fn add_assign(&mut self, rhs: $n) {
                    *self = *self + rhs;
                }
            }

            impl std::ops::SubAssign for $n {
                fn sub_assign(&mut self, rhs: $n) {
                    *self = *self - rhs;
                }
            }

            impl std::ops::MulAssign for $n {
                fn mul_assign(&mut self, rhs: $n) {
                    *self = *self * rhs;
                }
            }

            impl std::ops::DivAssign for $n {
                fn div_assign(&mut self, rhs: $n) {
                    *self = *self / rhs;
                }
            }

            impl std::fmt::Display for $n {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
            impl std::fmt::Debug for $n {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        };
    }
    // 10^8 < p < 10^9
    // 3 is primitive p-1 root of these
    // 167772161 = 5*2^25 + 1, 469762049 = 7*2^26 + 1, 998244353 = 119*2^23 + 1
    // 1224736769 = 73 * 2^24 + 1
    // define_modint!(ModInt167772161, 167772161);
    define_modint!(ModInt998244353, 998244353);
    define_modint!(ModInt1224736769, 1224736769);
    fn ntt<T: ModI>(a: &mut [T], n: usize, inv: bool) {
        // h = log2(n)
        let h = {
            let mut i = 0;
            while 1 << i != n {
                i += 1;
            }
            i
        };
        let mut j: usize;
        for i in 0..n {
            j = 0;
            for k in 0..h {
                // (i >> k & 1)はiのk桁目のbit
                // (h - 1 - k)は全体をh-bitとしてk桁目の反対の位置
                j |= (i >> k & 1) << (h - 1 - k);
            }
            // はじめの一回だけひっくりかえす
            if i < j {
                a.swap(i, j)
            };
        }
        // バタフライ演算
        let mut b = 1;
        while b < n {
            let zeta: T = T::new(3).pow((T::m() - 1) / (2 * b as u64));
            for j in 0..b {
                // 3 is primitive root of proth prime
                // 3 ^ ((m - 1) / (n * j)) is primitive n root's j power
                let e: T = if inv {
                    zeta.pow(j as u64).inv()
                } else {
                    zeta.pow(j as u64)
                };
                let mut k = 0;
                while k < n {
                    let s: T = a[j + k];
                    let t: T = a[j + k + b] * e;
                    a[j + k] = s + t;
                    a[j + k + b] = s - t;
                    k += b * 2;
                }
            }
            b *= 2;
        }

        if inv {
            let ni = T::new(n as u64).inv();
            for i in 0..n {
                a[i] *= ni;
            }
        }
    }

    fn mod_conv<T: ModI>(mut a: &mut [T], mut b: &mut [T]) -> Vec<T> {
        let n = a.len();
        // calc each mod
        ntt(&mut a, n, false);
        ntt(&mut b, n, false);
        let mut c = Vec::with_capacity(n);
        for i in 0..n {
            c.push(a[i] * b[i]);
        }
        ntt(&mut c, n, true);
        c
    }

    fn single_convolution<T: ModI>(a: &mut [T], b: &mut [T]) -> Vec<T> {
        let d: usize = a.len() + b.len() - 1;
        let n = d.checked_next_power_of_two().unwrap();
        let mut a = a.to_vec();
        a.resize(n, T::new(0));
        let mut b = b.to_vec();
        b.resize(n, T::new(0));
        let mut res = mod_conv(&mut a, &mut b);
        res.truncate(d);
        res
    }
    fn mod_pow(mut a: u64, mut n: u64, m: u64) -> u64 {
        let mut ret = 1;
        while n > 0 {
            if n & 1 == 1 {
                ret *= a;
                ret %= m;
            }
            a *= a;
            a %= m;
            n >>= 1;
        }
        ret
    }
    // mod mの体におけるaの逆元
    fn mod_inv(a: u64, m: u64) -> u64 {
        mod_pow(a, m - 2, m)
    }
    fn garner(mr: &mut Vec<(u64, u64)>, m: u64) -> u64 {
        mr.push((m, 0));
        // coef... mixed radixの係数, constants... 前まで求めた係数
        let mut coef: Vec<u64> = vec![1; mr.len()];
        let mut constants: Vec<u64> = vec![0; mr.len()];
        for i in 0..mr.len() - 1 {
            let v: u64 = (mr[i].1 + mr[i].0 - constants[i]) * mod_inv(coef[i], mr[i].0) % mr[i].0;
            for j in i + 1..mr.len() {
                constants[j] += coef[j] * v;
                constants[j] %= mr[j].0;
                coef[j] *= mr[i].0;
                coef[j] %= mr[j].0;
            }
        }
        constants[mr.len() - 1]
    }

    pub fn multiply(a: &[i64], b: &[i64], mo: i64) -> Vec<i64> {
        type F0 = ModInt1224736769;
        type F1 = ModInt998244353;

        let n = a.len();
        let m = b.len();
     
        let mut a0 = vec![];
        for i in 0..n {
            a0.push(F0::new(a[i] as u64));
        }
        let mut a1 = vec![];
        for i in 0..n {
            a1.push(F1::new(a[i] as u64));
        }
        let mut b0 = vec![];
        for i in 0..m {
            b0.push(F0::new(b[i] as u64));
        }
        let mut b1 = vec![];
        for i in 0..m {
            b1.push(F1::new(b[i] as u64));
        }

        let res0 = single_convolution(&mut a0, &mut b0);
        let res1 = single_convolution(&mut a1, &mut b1);

        let mut c = vec![];
        for i in 0..res0.len() {
            let mut mr = vec![(1224736769u64, res0[i].0), (998244353u64, res1[i].0)];
            let v = garner(&mut mr, mo as u64);
            c.push(v as i64)
        }
        c
    }
}

#[bench]
fn bench_ntt(b: &mut test::Bencher) {
    let mut x = vec![0;10000];
    for i in 0..10000 {
        x[i] = i as i64;
    }
    b.iter(||
        ntt_multiply(&x, &x, 1_000_000_007)
    )
}

#[bench]
fn bench_ntt_naive(b: &mut test::Bencher) {
    let mut x = vec![0;10000];
    for i in 0..10000 {
        x[i] = i as i64;
    }
    b.iter(||
        ntt_multiply_naive(&x, &x, 1_000_000_007)
    )
}