use cargo_snippet::snippet;

#[snippet("Doubling")]
trait Doublable {
    type T: std::fmt::Debug;
    fn id(&self) -> Self::T;
    fn f(&self) -> Self::T;
    fn double(&self, x: &Self::T, y: &Self::T) -> Self::T;
}
#[snippet("Doubling")]
struct Doubling<F: Doublable> {
    f: F,
    pow_table: Vec<F::T>,
}
#[snippet("Doubling")]
impl <F: Doublable> Doubling<F> {
    pub fn new(f: F, maxbit: usize) -> Self {
        let mut pow = vec![f.id()];
        for i in 0..=maxbit {
            let x = &pow[i];
            let y = f.double(&f.f(), x);
            pow.push(y);
        }
        Doubling {
            f: f,
            pow_table: pow,
        }
    }
    pub fn pow(&self, k: i64) -> F::T {
        let mut k = k;
        let mut res = self.f.id();
        let mut i = 1;
        while k > 0 {
            if k & 1 == 1 {
                res = self.f.double(&res, &self.pow_table[i]);
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
        fn id(&self) -> i64 { 1 }
        fn f(&self) -> i64 { 2 }
        fn double(&self, x: &i64, y: &i64) -> i64 { x*y }
    }
    let mut dbl = F;
    let mut f: Doubling<F> = Doubling::new(dbl, 60);
    assert_eq!(f.pow(1), 2);
    assert_eq!(f.pow(2), 4);
    assert_eq!(f.pow(3), 8);
    assert_eq!(f.pow(50), 1125899906842624);
}