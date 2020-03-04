#[snippet = "ntt_heia"]
pub mod ntt_heia {
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

#[snippet = "ntt_yuya178"]
pub mod ntt_yuya178 {
    pub fn multiply(a: &[i64], b: &[i64], mo: i64) -> Vec<i64> {
        let a = a.to_vec();
        let b = b.to_vec();
        let mut c = conv(a, b);
        for i in 0..c.len() {
            c[i] %= mo;
        }
        c
    }
    fn ntt(c: Vec<i64>, inv: bool, mo: i64, g: i64) -> Vec<i64>{
        let n = c.len();
        if n==1 {
            return c;
        }
        let c1: Vec<i64> = (0..n/2).map(|x| c[2*x]).collect();
        let c2: Vec<i64> = (0..n/2).map(|x| c[2*x+1]).collect();
        let r1 = ntt(c1, inv, mo, g);
        let r2 = ntt(c2, inv, mo, g);
        let mut ret = vec![0;n];
        let mut h = mod_pow(g,(mo-1)/(n as i64),mo);  // (h ^ n) == 1 (mod mo)
        if inv {h = mod_inv(h,mo);}  // h = h ^ -1
        let mut base = 1;
        for i in 0..n {
            ret[i] = r1[i%(n/2)] + base * r2[i%(n/2)];
            ret[i] %= mo;
            base = base * h % mo;
        }
        ret
    }
     
    pub fn conv(mut a1: Vec<i64>, mut a2: Vec<i64>) -> Vec<i64> {
        let sz = a1.len()+a2.len();
        let mut n = 1;
        while sz>n {n*=2;}
        while a1.len()<n { a1.push(0);}
        while a2.len()<n { a2.push(0);}
     
        // https://www.cnblogs.com/Guess2/p/8422205.html
        let mo = 5767169; // 5767169 = 11 * 2^19 + 1
        let g = 3;        // 5767169 の原始根
        let f1 = ntt(a1.clone(),false,mo,g);
        let f2 = ntt(a2.clone(),false,mo,g);
        let c1: Vec<i64> = (0..n).map(|x| f1[x]*f2[x]%mo).collect();
        let ans = ntt(c1.clone(),true,mo,g);
        let A = ans.iter().map(|x| x*mod_inv(n as i64,mo)%mo).collect::<Vec<i64>>();
     
        let mo2 = 7340033; // 7340033 = 7 * 2^20 + 1
        let g2 = 3;        // 7340033 の原始根
        let f12 = ntt(a1.clone(),false,mo2,g2);
        let f22 = ntt(a2.clone(),false,mo2,g2);
        let c12: Vec<i64> = (0..n).map(|x| f12[x]*f22[x]%mo2).collect();
        let ans2 = ntt(c12.clone(),true,mo2,g2);
        let A2 = ans2.iter().map(|x| x*mod_inv(n as i64,mo2)%mo2).collect::<Vec<i64>>();
     
        // x % mo  == A[i]
        // x % mo2 == A2[i]
        // を解く
        let mut ret = vec![0;ans.len()];
        for i in 0..ans.len() {
            let p = mo * mo2;
            ret[i] = (A[i] + (A2[i]-A[i]+mo2) % mo2 * mod_inv(mo,mo2) % mo2 * mo % p) % p;
        }
        ret
    }
     
    fn mod_pow(mut a: i64, n: i64, mo: i64) -> i64{
        a %= mo;
        if a == 0 {
            return 0;
        }
        let mut lo = n;
        let mut ret = 1;
        let mut x = a;
        while lo > 0 {
            if lo % 2 == 1 {
                ret = ret * x % mo;
            }
            x = x * x % mo;
            lo /= 2;
        }
        ret
    }
     
    fn mod_inv(a: i64, mo: i64) -> i64{
        mod_pow(a,mo-2,mo)
    }
}