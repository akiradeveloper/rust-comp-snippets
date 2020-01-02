use crate::complex::Complex;

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