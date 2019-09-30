#[snippet = "Matrix"]
#[derive(Clone)]
pub struct Matrix {
    v: Vec<Vec<i64>>,
}
#[snippet = "Matrix"]
impl Matrix {
    fn identity(n: usize) -> Self {
        let mut v = vec![vec![0;n];n];
        for i in 0..n {
            v[i][i] = 1;
        }
        Matrix { v: v }
    }
    fn m(&self) -> usize {
        self.v.len()
    }
    fn n(&self) -> usize {
        self.v[0].len()
    }
    fn pow(&self, k: u64, m: i64) -> Self {
        assert!(self.m() == self.n());
        let mut k = k;
        let mut x = Self::identity(self.m());
        let mut y = self.clone();
        while k > 0 {
            if k & 1 > 0 {
                x = y.clone() * x;
                x %= m;
            }
            y = y.clone() * y.clone();
            y %= m;
            k >>= 1;
        }
        x
    }
}
#[snippet = "Matrix"]
impl std::ops::Add for Matrix {
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
#[snippet = "Matrix"]
impl std::ops::Sub for Matrix {
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
#[snippet = "Matrix"]
impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut r = vec![vec![0; other.n()]; self.m()];
        for i in 0..self.m() {
            for j in 0..self.n() {
                let mut v = 0;
                for k in 0..self.n() {
                    v += self.v[i][k] * other.v[k][j];
                }
                r[i][j] = v;
            }
        }
        Matrix { v: r }
    }
}
#[snippet = "Matrix"]
impl std::ops::Rem<i64> for Matrix {
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
#[snippet = "Matrix"]
impl std::ops::RemAssign<i64> for Matrix {
    fn rem_assign(&mut self, mo: i64) {
        for i in 0..self.m() {
            for j in 0..self.n() {
                self.v[i][j] %= mo;
            }
        }
    }
}

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