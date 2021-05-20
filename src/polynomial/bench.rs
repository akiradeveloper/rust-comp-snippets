use super::{ntt, fft, karatsuba};

const N: usize = 10000;

#[bench]
fn bench_karatsuba(b: &mut test::Bencher) {
    let mut x = vec![0;N];
    for i in 0..N {
        x[i] = i as i64;
    }
    b.iter(||
        karatsuba::multiply(&x, &x)
    )
}

#[bench]
fn bench_ntt(b: &mut test::Bencher) {
    let mut x = vec![0;N];
    for i in 0..N {
        x[i] = i as i64;
    }
    b.iter(||
        ntt::ntt_multiply(&x, &x, 1_000_000_007)
    )
}

#[bench]
fn bench_ntt_naive(b: &mut test::Bencher) {
    let mut x = vec![0;N];
    for i in 0..N {
        x[i] = i as i64;
    }
    b.iter(||
        ntt::ntt_multiply_naive(&x, &x, 1_000_000_007)
    )
}

#[bench]
fn bench_ntt_fft(b: &mut test::Bencher) {
    let mut x = vec![0;N];
    for i in 0..N {
        x[i] = i as i64;
    }
    b.iter(||
        fft::multiply(&x, &x)
    )
}