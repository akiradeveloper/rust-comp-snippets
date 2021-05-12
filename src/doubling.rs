use cargo_snippet::snippet;

/// ダブリング
/// 
/// 理論的には
/// [f^1,f^2,f^4,...]
/// という配列を構築すればよいが、関数を保持するというのは難しい。
/// 例えば、f^4を計算するのにf^2が必要といったように関数を構築するのに遅延計算を行うと、計算量に影響する。
/// 
/// そこで、このライブラリでは、関数を表す値を保持することにする。
/// 
/// x0: 単位
/// f: ap(f^1, x0)
/// ap: 任意のfをxに適用する
/// inv: f^n(x0)が与えられた時、f^nに相当する値を計算する

#[snippet("Doubling")]
trait Doublable {
    type T: std::fmt::Debug;
    fn x0(&self) -> Self::T;
    fn f(&self) -> Self::T;
    fn ap(&self, f: &Self::T, x: &Self::T) -> Self::T;
    fn inv(&self, x: &Self::T) -> Self::T;
}
#[snippet("Doubling")]
struct Doubling<D: Doublable> {
    d: D,
    f_table: Vec<D::T>,
}
#[snippet("Doubling")]
impl <D: Doublable> Doubling<D> {
    pub fn new(d: D, maxbit: usize) -> Self {
        let mut f = vec![d.f()];
        for i in 1..=maxbit {
            let x = d.x0();
            let fx = d.ap(&f[i-1], &x);
            let ffx = d.ap(&f[i-1], &fx);
            f.push(d.inv(&ffx));
        }
        Doubling {
            d: d,
            f_table: f,
        }
    }
    pub fn pow(&self, k: i64) -> D::T {
        let mut k = k;
        let mut res = self.d.x0();
        let mut i = 0;
        while k > 0 {
            if k & 1 == 1 {
                res = self.d.ap(&self.f_table[i], &res);
            }
            k >>= 1;
            i += 1;
        }
        res
    }
}
#[test]
fn test_doubling() {
    struct F;
    impl Doublable for F {
        type T = i64;
        fn x0(&self) -> i64 { 1 }
        fn f(&self) -> i64 { 2 }
        fn ap(&self, f: &i64, x: &i64) -> i64 { f*x }
        fn inv(&self, x: &i64) -> i64 { *x }
    }
    let mut f = Doubling::new(F, 5);
    assert_eq!(f.pow(1), 2);
    assert_eq!(f.pow(2), 4);
    assert_eq!(f.pow(3), 8);
    assert_eq!(f.pow(50), 1125899906842624);
}