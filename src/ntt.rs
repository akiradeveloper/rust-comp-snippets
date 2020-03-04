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

use crate::ntt_ext::{ntt_heia, ntt_yuya178};

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
    assert_eq!(ntt_heia::multiply(&x, &y, ten(9)+7), t1);
    // assert_eq!(ntt_yuya178::multiply(&x, &y, ten(9)+7), t1);
    // assert_eq!(fft::multiply(&x, &y, ten(9)+7), t1);
}

const N: usize = 10000;

#[bench]
fn bench_ntt(b: &mut test::Bencher) {
    let mut x = vec![0;N];
    for i in 0..N {
        x[i] = i as i64;
    }
    b.iter(||
        ntt_multiply(&x, &x, 1_000_000_007)
    )
}

#[bench]
fn bench_ntt_naive(b: &mut test::Bencher) {
    let mut x = vec![0;N];
    for i in 0..N {
        x[i] = i as i64;
    }
    b.iter(||
        ntt_multiply_naive(&x, &x, 1_000_000_007)
    )
}

#[bench]
fn bench_ntt_heia(b: &mut test::Bencher) {
    let mut x = vec![0;N];
    for i in 0..N {
        x[i] = i as i64;
    }
    b.iter(||
        ntt_heia::multiply(&x, &x, 1_000_000_007)
    )
}

#[bench]
fn bench_ntt_yuya178(b: &mut test::Bencher) {
    let mut x = vec![0;N];
    for i in 0..N {
        x[i] = i as i64;
    }
    b.iter(||
        ntt_yuya178::multiply(&x, &x, 1_000_000_007)
    )
}

use crate::fft;

#[bench]
fn bench_ntt_fft(b: &mut test::Bencher) {
    let mut x = vec![0;N];
    for i in 0..N {
        x[i] = i as i64;
    }
    b.iter(||
        fft::multiply(&x, &x, 1_000_000_007)
    )
}