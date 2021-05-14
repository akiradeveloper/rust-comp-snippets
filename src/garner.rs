use cargo_snippet::snippet;
use crate::number::{gcd, modinv};

/// 中国余剰定理の一般解法
/// 
/// x = r1 (mod m1)
/// x = r2 (mod m2)
/// ...
/// x = rk (mod mk)
/// を満たすxは
/// 0以上m0m1...mk未満の中に一つ存在する。
/// 
/// moは素数

#[test]
fn test_garner() {
    let mut rm = vec![(2,3),(4,5)];
    let ok = pre_garner(&mut rm);
    let v = garner(rm, 1_000_000_007);
    assert_eq!(v, 14);
}

#[snippet("garner")]
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

/// 入力の整形用関数。
/// miを素にする。
/// 不可能な場合はfalseを返す。

#[snippet("garner")]
pub fn pre_garner(rm: &mut Vec<(i64,i64)>) -> bool {
    let n = rm.len();
    for i in 0..n {
        for j in 0..i {
            let g = gcd(rm[i].1, rm[j].1);
            if (rm[i].0 - rm[j].0) % g != 0 { return false }
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
    true
}