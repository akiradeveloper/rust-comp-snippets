use cargo_snippet::snippet;

#[snippet("Doubling")]
trait Doublable {
    type T: std::fmt::Debug;
    fn id() -> Self::T;
    fn f() -> Self::T;
    fn double(x: &Self::T, y: &Self::T) -> Self::T;
}
#[snippet("Doubling")]
struct Doubling<F: Doublable> {
    pow_table: Vec<F::T>,
}
#[snippet("Doubling")]
impl <F: Doublable> Doubling<F> {
    pub fn new(maxbit: usize) -> Self {
        let mut pow = vec![F::id()];
        for i in 0..=maxbit {
            let x = &pow[i];
            let y = F::double(&F::f(), x);
            pow.push(y);
        }
        Doubling {
            pow_table: pow,
        }
    }
    pub fn pow(&self, k: i64) -> F::T {
        let mut k = k;
        let mut res = F::id();
        let mut i = 1;
        while k > 0 {
            if k & 1 == 1 {
                res = F::double(&res, &self.pow_table[i]);
            }
            k >>= 1;
            i *= 2;
        }
        res
    }
}
#[test]
fn test_doubling() {
    struct F;
    impl Doublable for F {
        type T = i64;
        fn id() -> i64 { 1 }
        fn f() -> i64 { 2 }
        fn double(x: &i64, y: &i64) -> i64 { x*y }
    }
    let mut f: Doubling<F> = Doubling::new(60);
    assert_eq!(f.pow(1), 2);
    assert_eq!(f.pow(2), 4);
    assert_eq!(f.pow(3), 8);
    assert_eq!(f.pow(50), 1125899906842624);
}