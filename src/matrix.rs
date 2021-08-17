use cargo_snippet::snippet;

#[snippet("Matrix")]
pub mod matrix {
    #[derive(Clone)]
    pub struct Matrix {
        pub v: Vec<Vec<i64>>,
    }
    impl Matrix {
        pub fn new(v: Vec<Vec<i64>>) -> Self {
            Self { v }
        }
        pub fn into_inner(self) -> Vec<Vec<i64>> {
            self.v
        }
        /// (a|b)
        pub fn combine(a: Self, b: Self) -> Self {
            assert_eq!(a.m(), b.m());
            let m = a.m();
            let mut ret = Self::zeros(m, a.n()+b.n());
            for i in 0..m {
                for j in 0..a.n() {
                    ret[i][j] = a[i][j];
                }
                for j in 0..b.n() {
                    ret[i][a.n()+j] = b[i][j];
                }
            }
            ret
        }
        pub fn transpose(self) -> Self {
            let mut ret = Self::zeros(self.n(), self.m());
            for i in 0..self.n() {
                for j in 0..self.m() {
                    ret[j][i] = self[i][j];
                }
            }
            ret
        }
        pub fn zeros(m: usize, n: usize) -> Self {
            let v = vec![vec![0;n];m];
            Self { v }
        }
        pub fn identity(n: usize) -> Self {
            let mut v = vec![vec![0;n];n];
            for i in 0..n {
                v[i][i] = 1;
            }
            Matrix { v: v }
        }
        pub fn m(&self) -> usize {
            self.v.len()
        }
        pub fn n(&self) -> usize {
            self.v[0].len()
        }
        pub fn mul_rem(&self, other: &Self, mo: i64) -> Self {
            assert!(self.n() == other.m());
            let K = self.n();
            let M = self.m();
            let N = other.n();
            let mut r = vec![vec![0; N]; M];
            for i in 0..M {
                for j in 0..N {
                    let mut v = 0;
                    for k in 0..K {
                        v += self.v[i][k] * other.v[k][j] % mo;
                        v %= mo;
                    }
                    r[i][j] = v;
                }
            }
            Matrix { v: r }
        }
        pub fn pow(&self, k: u64, mo: i64) -> Self {
            assert!(self.m() == self.n());
            let mut k = k;
            let mut x = Self::identity(self.m());
            let mut y = self.clone();
            while k > 0 {
                if k & 1 > 0 {
                    x = y.clone() * x;
                    x %= mo;
                }
                y = y.mul_rem(&y, mo);
                y %= mo;
                k >>= 1;
            }
            x
        }
    }

    use std::ops::*;

    impl Add for Matrix {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            let mut r = self.v.clone();
            for i in 0..self.m() {
                for j in 0..self.n() {
                    r[i][j] += other.v[i][j];
                }
            }
            Matrix { v: r }
        }
    }
    impl Sub for Matrix {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            let mut r = self.v.clone();
            for i in 0..self.m() {
                for j in 0..self.n() {
                    r[i][j] -= other.v[i][j];
                }
            }
            Matrix { v: r }
        }
    }
    impl Mul for Matrix {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            assert!(self.n() == other.m());
            let K = self.n();
            let M = self.m();
            let N = other.n();
            let mut r = vec![vec![0; N]; M];
            for i in 0..M {
                for j in 0..N {
                    let mut v = 0;
                    for k in 0..K {
                        v += self.v[i][k] * other.v[k][j];
                    }
                    r[i][j] = v;
                }
            }
            Matrix { v: r }
        }
    }
    impl Rem<i64> for Matrix {
        type Output = Self;
        fn rem(self, mo: i64) -> Self {
            let mut r = self.v.clone();
            for i in 0..self.m() {
                for j in 0..self.n() {
                    r[i][j] %= mo;
                }
            }
            Matrix { v: r }
        }
    }
    impl RemAssign<i64> for Matrix {
        fn rem_assign(&mut self, mo: i64) {
            for i in 0..self.m() {
                for j in 0..self.n() {
                    self.v[i][j] %= mo;
                }
            }
        }
    }
    impl std::ops::Index<usize> for Matrix {
        type Output = [i64];
        fn index(&self, i: usize) -> &Self::Output {
            &self.v[i]
        }
    }
    impl std::ops::IndexMut<usize> for Matrix {
        fn index_mut(&mut self, i: usize) -> &mut Self::Output {
            &mut self.v[i]
        }
    }
}
#[snippet("Matrix")]
pub type Matrix = matrix::Matrix;

#[test]
fn test_matrix_add() {
    let a = Matrix {
        v: vec![vec![1,2],vec![3,4]]
    };
    let b = Matrix {
        v: vec![vec![5,6],vec![7,8]]
    };
    let c = a.clone()+b.clone();
    dbg!(c.v);
    let d = a-b;
    dbg!(d.v);
}
#[test]
fn test_matrix_sub() {
    let a = Matrix {
        v: vec![vec![1,2],vec![3,4]]
    };
    let b = Matrix {
        v: vec![vec![5,6],vec![7,8]]
    };
    let c = a-b;
    dbg!(c.v);

}
#[test]
fn test_matrix_rem() {
    let mut a = Matrix {
        v: vec![vec![5,6],vec![7,8]]
    };
    a%=3;
    dbg!(a.v);
}
#[test]
fn test_matrix_mul() {
    let a = Matrix {
        v: vec![vec![1,2],vec![3,4]]
    };
    let b = Matrix {
        v: vec![vec![5,6],vec![7,8]]
    };
    let c = a*b;
    dbg!(c.v);
}
#[test]
fn test_matrix_mul_vec() {
    let a = Matrix {
        v: vec![
            vec![1,2],
            vec![3,4],
        ]
    };
    let b = Matrix {
        v: vec![
            vec![5],
            vec![6],
        ]
    };
    let c = a * b;
    dbg!(c.v);
}
#[test]
fn test_matrix_pow() {
    let x = Matrix {
        v: vec![
            vec![1,2],
            vec![3,4]
        ]
    };
    let x3 = x.clone() * x.clone() * x.clone();
    assert_eq!(x.pow(3, std::i64::MAX).v, x3.v);
}