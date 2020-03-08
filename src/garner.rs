use crate::number::modinv;

#[snippet = "garner"]
#[doc = "compute minimum x from a list of x = r[i] (mod m[i]) all m[i] are co-primes"]
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

pub fn pre_garner(rm: &mut Vec<(i64,i64)>) -> i64 {
    0
}