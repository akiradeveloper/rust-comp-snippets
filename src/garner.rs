use cargo_snippet::snippet;
use crate::number::{gcd, modinv};

#[snippet("garner")]
#[doc = "compute minimum x from a list of x = r[i] (mod m[i]) all m[i] are co-primes and some r[i] should be non-zero."]
pub fn garner(rm: Vec<(i64,i64)>, mo: i64) -> i64 {
    let mut rm = rm;
    rm.push((0, mo));
    let mut coef = vec![1; rm.len()];
    let mut constants = vec![0; rm.len()];
    for i in 0..rm.len() - 1 {
        let v = (rm[i].0 + rm[i].1 - constants[i]) * modinv(coef[i], rm[i].1) % rm[i].1;
        for j in i + 1..rm.len() {
            constants[j] += coef[j] * v;
            constants[j] %= rm[j].1;
            coef[j] *= rm[i].1;
            coef[j] %= rm[j].1;
        }
    }
    constants[rm.len() - 1]
}
#[snippet("garner")]
#[doc = "none if there is no such x"]
pub fn pre_garner(rm: &mut Vec<(i64,i64)>, mo: i64) -> Option<i64> {
    let n = rm.len();
    for i in 0..n {
        for j in 0..i {
            let g = gcd(rm[i].1, rm[j].1);
            if (rm[i].0 - rm[j].0) % g != 0 { return None }
            rm[i].1 /= g;
            rm[j].1 /= g;
            let mut gi = gcd(rm[i].1, g);
            let mut gj = g / gi;
            loop {
                let g = gcd(gi,gj);
                gi *= g;
                gj /= g;
                if g == 1 { break; }
            }
            rm[i].1 *= gi;
            rm[j].1 *= gj;
            rm[i].0 %= rm[i].1;
            rm[j].0 %= rm[j].1;
        }
    }
    let mut lcm = 1;
    for i in 0..n {
        lcm = lcm * rm[i].1 % mo;
    }
    Some(lcm)
}