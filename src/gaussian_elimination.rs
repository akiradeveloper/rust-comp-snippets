use cargo_snippet::snippet;
use crate::number::modinv;
use crate::matrix::Matrix;

/// ガウスの掃き出し法

/// 
/// Rankというのはざっくりいうと、
/// 実質的に何次元の一次変換かということ。

#[snippet("GaussianElimination")]
struct GaussianElimination {
    A: Vec<Vec<i64>>,
    rank: usize,
}
#[snippet("GaussianElimination")]
impl GaussianElimination {
    #[doc = "Ax = y"]
    pub fn new(A: Vec<Vec<i64>>, y: Vec<i64>, mo: i64) -> GaussianElimination {
        let h = A.len();
        let w = A[0].len();
        assert!(y.len() == h);
        let mut tmp = vec![vec![0;w+1]; h];
        for i in 0..h {
            for j in 0..w {
                tmp[i][j] = (A[i][j] + mo) % mo;
            }
        }
        for i in 0..h {
            tmp[i][w] = y[i];
        }
        let rank = Self::sweep(&mut tmp, mo);
        GaussianElimination {
            A: tmp,
            rank: rank,
        }
    }
    fn sweep(A: &mut [Vec<i64>], mo: i64) -> usize {
        let h = A.len();
        let w = A[0].len();
        let mut rank = 0;
        for j in 0..w-1 {
            let mut pivot = h;
            for i in rank..h {
                if A[i][j] != 0 {
                    pivot = i;
                    break;
                }
            }
            if pivot == h { continue; }
            for j2 in 0..w {
                let x = A[pivot][j2];
                let y = A[rank][j2];
                A[rank][j2] = x;
                A[pivot][j2] = y;
            }
            let inv = modinv(A[rank][j], mo);
            for j2 in 0..w {
                A[rank][j2] = (A[rank][j2] * inv) % mo;
            }
            for i in 0..h {
                if i != rank && A[i][j] != 0 {
                    let fac = A[i][j];
                    for j2 in 0..w {
                        A[i][j2] -= (A[rank][j2] * fac) % mo;
                        if A[i][j2] < 0 {
                            A[i][j2] += mo;
                        }
                    }
                }
            }
            rank += 1;
        }
        rank
    }
    fn h(&self) -> usize {
        self.A.len()
    }
    fn w(&self) -> usize {
        self.A[0].len() - 1
    }
    fn rank(&self) -> usize {
        self.rank
    }
    fn x(&self) -> Option<Vec<i64>> {
        let h = self.h();
        let w = self.w();
        for i in self.rank()..h {
            if self.A[i][w] != 0 {
                return None
            }
        }
        let mut res = vec![0; w];
        for i in 0..self.rank() {
            res[i] = self.A[i][w];
        }
        Some(res)
    }
}
enum LinSolveResult {
    Infinite,
    None,
    One(i64),
}
struct LinSolve {
    pub M1: GaussianElimination,
    pub M2: GaussianElimination,
}
impl LinSolve {
    /// Ax = y
    /// の解xをmod Mの下で計算する。
    pub fn solve(A: Vec<Vec<i64>>, y: Vec<i64>, mo: i64) -> LinSolveResult {
        unimplemented!()
    }
}
struct InvMatrix {
    pub M: GaussianElimination,
}
impl InvMatrix {
    /// ガウスの掃き出し法を使って逆行列を求める
    pub fn solve(A: Vec<Vec<i64>>, mo: i64) -> Option<Vec<Vec<i64>>> {
        let n = A.len();
        let e = Matrix::identity(n);
        let m = vec![vec![0;2*n];n];
        unimplemented!()
    }
}
#[test]
fn test_gaussian_elimination_1() {
    let mut A = vec![
        vec![1,2,-2],
        vec![1,-1,3],
        vec![2,3,-5],
    ];
    let y = vec![3,4,1];
    let ge = GaussianElimination::new(
        A, y, 1_000_000_009,
    );
    let x = ge.x();
    assert!(x.is_some());
    let x = x.unwrap();
    assert_eq!(x, vec![1,3,2]);
}
#[test]
fn test_gaussian_elimination_2() {
    let mut A = vec![
        vec![8,6,4],
        vec![6,4,2],
        vec![4,2,1],
    ];
    let y = vec![36,22,12];
    let ge = GaussianElimination::new(
        A, y, 1_000_000_009,
    );
    let x = ge.x();
    assert!(x.is_some());
    let x = x.unwrap();
    assert_eq!(x, vec![1,2,4]);
}