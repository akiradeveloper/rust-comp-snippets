use cargo_snippet::snippet;
use crate::ntt;

#[snippet("Polynomial")]
#[derive(Debug)]
struct Polynomial {
    coeff: Vec<i64>,
    mo: i64,
}
#[snippet("Polynomial")]
impl Polynomial {
    pub fn new(coeff: Vec<i64>, mo: i64) -> Polynomial {
        Polynomial {
            coeff: coeff,
            mo: mo,
        }
    }
    pub fn multiply(&self, other: &Self) -> Self {
        assert!(self.mo == other.mo);
        Polynomial {
            coeff: ntt::ntt_multiply(&self.coeff, &other.coeff, self.mo),
            mo: self.mo,
        }
    }
    pub fn pow(&self, k: i64) -> Self {
        let mut res = Polynomial {
            coeff: vec![1],
            mo: self.mo,
        };
        let mut x = Polynomial {
            coeff: self.coeff.clone(),
            mo: self.mo,
        };

        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                res = res.multiply(&x);
            }
            x = x.multiply(&x);
            k >>= 1;
        }
        res
    }
}