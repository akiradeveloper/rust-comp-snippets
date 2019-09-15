#[snippet = "ternary_search"]
#[doc = "f(x) should be upper convex function in [l,r]. find x where f(x) is the max in the range."]
fn ternary_search<F: Fn(f64) -> f64>(f: F, l: f64, r: f64) -> f64 {
    let mut l = l;
    let mut r = r;
    let eps = 0.00000000001;
    while r-l > eps {
        let m1 = (2.0*l+r)/3.0;
        let m2 = (l+2.0*r)/3.0;
        let f1 = f(m1);
        let f2 = f(m2);
        if f1<f2 {
            l = m1;
        } else {
            r = m2;
        }
    }
    l
}