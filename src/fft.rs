use crate::complex::Complex;

#[snippet = "fft"]
pub fn convolve(a: &[f64], b: &[f64]) -> Vec<f64> {
    let n = a.len() + b.len() - 1;
    let mut m = 1;
    while m < n {
        m *= 2;
    }
    let mut x = vec![Complex::new(0.,0.); m];
    for i in 0..a.len() {
        x[i] = Complex::new(a[i], 0.);
    }
    let mut y = vec![Complex::new(0.,0.); m];
    for i in 0..b.len() {
        y[i] = Complex::new(b[i], 0.);
    }
    let X = fast_fourier_transform(&x, false);
    let Y = fast_fourier_transform(&y, false);
    let mut Z = vec![Complex::new(0.,0.); m];
    for i in 0..m {
        Z[i] = X[i] * Y[i];
    }
    let z = fast_fourier_transform(&Z, true);
    let mut ret = vec![0.; m];
    for i in 0..m {
        ret[i] = z[i].x;
    }
    ret
}

#[snippet = "fft"]
pub fn fast_fourier_transform(arr: &[Complex], inv: bool) -> Vec<Complex> {
    let n = arr.len();
    assert!(n.count_ones() == 1, "the length of array is not square");
    let mut a: Vec<_> = arr.to_vec();
    let mut tmp: Vec<_> = (0..n).map(|_| Complex::new(0., 0.)).collect();
    let mut ai: Vec<_> = (0..n).map(|i| i).collect();
    let mut ti: Vec<_> = (0..n).map(|_| 0).collect();
    let bit = n.trailing_zeros();
    let f = if inv { -1.0 } else { 1.0 };
    for si in (0..bit).rev() {
        let s = 1 << si;
        std::mem::swap(&mut a, &mut tmp);
        std::mem::swap(&mut ai, &mut ti);
        let zeta = Complex::polar(1.0, std::f64::consts::PI * 2.0 * f / (s << 1) as f64);
        let mut z_i = Complex::new(1.0, 0.0);
        let mut ev = 0;
        let mut od = 1;
        for i in 0..n {
            if (i & s) != 0 {
                a[i] = (tmp[i - s] - tmp[i]) * z_i;
                ai[i] = ti[od];
                od += 2;
                z_i *= zeta;
            }
            else {
                a[i] = tmp[i] + tmp[i + s];
                ai[i] = ti[ev];
                ev += 2;
                z_i = Complex::new(1.0, 0.0);
            }
        }
    }
 
    std::mem::swap(&mut a, &mut tmp);
    let inv_n = if inv { n as f64 } else { 1.0 };
    for i in 0..n { a[ai[i]] = Complex::new(tmp[i].x / inv_n, tmp[i].y / inv_n); }
    a
}