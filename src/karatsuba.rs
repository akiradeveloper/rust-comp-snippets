#[snippet = "karatsuba"]
fn karatsuba<T>(a: &[T], b: &[T], c: &mut [T]) 
where T: std::marker::Copy +
         std::ops::Add<Output = T> +
         std::ops::Sub<Output = T> +
         std::ops::Mul<Output = T> +
         std::default::Default 
{
    let n = a.len();
    if n <= 32 {
        for (i, a) in a.iter().enumerate() {
            for (c, b) in c[i..].iter_mut().zip(b.iter()) {
                *c = *c + *a * *b;
            }
        }
        return;
    }
    if n & 1 == 1 {
        karatsuba(&a[1..], &b[1..], &mut c[2..]);
        let x = a[0];
        let y = b[0];
        c[0] = c[0] + x * y;
        for (c, (a, b)) in c[1..].iter_mut().zip(a[1..].iter().zip(b[1..].iter())) {
            *c = *c + x * *b + *a * y;
        }
        return;
    }
    let m = n / 2;
    karatsuba(&a[..m], &b[..m], &mut c[..n]);
    karatsuba(&a[m..], &b[m..], &mut c[n..]);
    let mut buf = vec![T::default(); 2 * n];
    let (x, y) = buf.split_at_mut(m);
    let (y, z) = y.split_at_mut(m);
    for (x, (p, q)) in x.iter_mut().zip(a.iter().zip(a[m..].iter())) {
        *x = *p + *q;
    }
    for (y, (p, q)) in y.iter_mut().zip(b.iter().zip(b[m..].iter())) {
        *y = *p + *q;
    }
    karatsuba(x, y, z);
    for (z, (p, q)) in z.iter_mut().zip(c[..n].iter().zip(c[n..].iter())) {
        *z = *z - (*p + *q);
    }
    for (c, z) in c[m..].iter_mut().zip(z.iter()) {
        *c = *c + *z;
    }
}
 
#[snippet = "karatsuba"]
pub fn multiply<T>(a: &[T], b: &[T]) -> Vec<T>
where T: std::marker::Copy +
         std::ops::Add<Output = T> +
         std::ops::Sub<Output = T> +
         std::ops::Mul<Output = T> +
         std::default::Default 
{
    let mut i = 0;
    let mut j = 0;
    let mut ans = vec![T::default(); a.len() + b.len()];
    let mut c = Vec::with_capacity(a.len() + b.len());
    while i < a.len() && j < b.len() {
        let x = a.len() - i;
        let y = b.len() - j;
        let z = std::cmp::min(x, y);
        c.clear();
        c.resize(2 * z, T::default());
        karatsuba(&a[i..(i + z)], &b[j..(j + z)], &mut c);
        for (ans, c) in ans[(i + j)..].iter_mut().zip(c.iter()) {
            *ans = *ans + *c;
        }
        if x <= y {
            j += x;
        } else {
            i += y;
        }
    }
    ans.truncate(a.len() + b.len() - 1);
    ans
}