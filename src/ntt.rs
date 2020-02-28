use crate::number::{mod_inverse, modpow};

struct NTT {
    pub mo: i64,
}
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
            h = mod_inverse(h, self.mo);
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
        loop {
            if m < n {
                let m2 = 2*m;
                let base = modpow(h, (n/m2) as i64, self.mo);
                let mut w = 1;
                for x in 0..m {
                    let mut s = x;
                    loop {
                        if s < n {
                            let u = a[s];
                            let d = (a[s+m] * w) % self.mo;
                            a[s] = u + d;
                            if a[s] >= self.mo { a[s] -= self.mo }
                            a[s+m] = u - d;
                            if a[s+m] < 0 { a[s+m] += self.mo }
                            s += m2;
                        } else {
                            break;
                        }
                    }
                }
                m *= 2;
            } else {
                break;
            }
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
        let n_inv = mod_inverse(a.len() as i64, self.mo);
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
            c[i] = a[i] * b[i];
        }

        self.intt(&mut c, n);
        c
    }
}
pub fn garner(mr: Vec<(i64,i64)>, mo: i64) -> i64 {
    let mut mr = mr;
    mr.push((mo, 0));
    let mut coef = vec![1; mr.len()];
    let mut constants = vec![0; mr.len()];
    for i in 0..mr.len() - 1 {
        let v = (mr[i].1 + mr[i].0 - constants[i]) * mod_inverse(coef[i], mr[i].0) % mr[i].0;
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

    let x = ntt1.convolve(&a,&b);
    let y = ntt2.convolve(&a,&b);
    let z = ntt3.convolve(&a,&b);
    
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
}