use cargo_snippet::snippet;
use crate::number::modinv;
use crate::matrix::Matrix;

/// ガウスの掃き出し法
/// Rankというのはざっくりいうと、
/// 実質的に何次元の一次変換かということ。
#[snippet("GaussianElimination")]
pub struct GaussianElimination {
    pub mat: Matrix,
    pub rank: usize,
}
#[snippet("GaussianElimination")]
impl GaussianElimination {
    pub fn sweep(mat: Matrix, mo: i64) -> GaussianElimination {
        let mut mat = mat;
        let rank = Self::do_sweep(&mut mat, mo);
        GaussianElimination {
            mat,
            rank: rank,
        }
    }
    fn do_sweep(mat: &mut Matrix, mo: i64) -> usize {
        let h = mat.m();
        let w = mat.n();
        let mut rank = 0;
        for j in 0..w {
            let mut pivot = h;
            for i in rank..h {
                if mat[i][j] != 0 {
                    pivot = i;
                    break;
                }
            }
            if pivot == h { continue; }
            for j2 in 0..w {
                let x = mat[pivot][j2];
                let y = mat[rank][j2];
                mat[rank][j2] = x;
                mat[pivot][j2] = y;
            }
            let inv = modinv(mat[rank][j], mo);
            for j2 in 0..w {
                mat[rank][j2] = (mat[rank][j2] * inv) % mo;
            }
            for i in 0..h {
                if i != rank && mat[i][j] != 0 {
                    let fac = mat[i][j];
                    for j2 in 0..w {
                        mat[i][j2] -= (mat[rank][j2] * fac) % mo;
                        if mat[i][j2] < 0 {
                            mat[i][j2] += mo;
                        }
                    }
                }
            }
            rank += 1;
        }
        rank
    }
}
#[snippet("LinSolve")]
#[derive(PartialEq, Debug)]
pub enum LinSolveResult {
    Infinite,
    None,
    One(Matrix),
}
#[snippet("LinSolve")]
pub struct LinSolve;
#[snippet("LinSolve")]
impl LinSolve {
    /// Ax = y
    /// の解xをmod Mの下で計算する。
    pub fn solve(a: Matrix, y: Matrix, mo: i64) -> LinSolveResult {
        assert_eq!(a.m(), a.n());
        let n = a.m();
        let elim1 = GaussianElimination::sweep(a.clone(), mo);
        dbg!(&elim1.mat);
        let elim2 = GaussianElimination::sweep(Matrix::combine(a, y), mo);
        dbg!(&elim2.mat);
        if elim1.rank == n {
            let mut ret = Matrix::zeros(n, 1);
            for i in 0..n {
                ret[i][0] = elim2.mat[i][n];
            }
            LinSolveResult::One(ret)
        } else {
            if elim1.rank == elim2.rank {
                LinSolveResult::Infinite
            } else {
                LinSolveResult::None
            }
        }
    }
}
#[snippet("InvMatrix")]
struct InvMatrix;
#[snippet("InvMatrix")]
impl InvMatrix {
    /// ガウスの掃き出し法を使って逆行列を求める
    pub fn solve(a: Matrix, mo: i64) -> Option<Matrix> {
        assert_eq!(a.m(), a.n());
        let n = a.m();
        let e = Matrix::identity(n);
        let elim1 = GaussianElimination::sweep(a.clone(), mo);
        dbg!(&elim1.mat);
        let elim2 = GaussianElimination::sweep(Matrix::combine(a, e), mo);
        dbg!(&elim2.mat);

        if elim1.rank == n {
            let mut ret = Matrix::zeros(n, n);
            for i in 0..n {
                for j in 0..n {
                    ret[i][j] = elim2.mat[i][n+j];
                }
            }
            Some(ret)
        } else {
            None
        }
    }
}
#[test]
fn test_sweep() {
    let a = Matrix::new(vec![
        vec![0,2,3],
        vec![3,1,1],
        vec![1,2,3],
    ]);
    let b = GaussianElimination::sweep(a.clone(), 1_000_000_009);
    assert_eq!(b.mat, Matrix::identity(3));
}
#[test]
fn test_linsolve_1() {
    let a = Matrix::new(vec![
        vec![1,2,-2],
        vec![1,-1,3],
        vec![2,3,-5],
    ]);
    let y = Matrix::new(vec![vec![3,4,1]]).transpose();
    let x = LinSolve::solve(
        a, y, 1_000_000_009,
    );
    assert_eq!(x, LinSolveResult::One(Matrix::new(vec![vec![1,3,2]]).transpose()));
}
#[test]
fn test_linsolve_2() {
    let a = Matrix::new(vec![
        vec![8,6,4],
        vec![6,4,2],
        vec![4,2,1],
    ]);
    let y = Matrix::new(vec![vec![36,22,12]]).transpose();
    let x = LinSolve::solve(
        a, y, 1_000_000_009,
    );
    assert_eq!(x, LinSolveResult::One(Matrix::new(vec![vec![1,2,4]]).transpose()));
}
#[test]
fn test_linsolve_3() {
    let a = Matrix::new(vec![
        vec![2,3,-2,-3],
        vec![-1,-2,1,2],
        vec![1,3,-2,-2],
        vec![2,1,0,-3],
    ]);
    let y = Matrix::new(vec![vec![1,0,1,1]]).transpose();
    let x = LinSolve::solve(a, y, 1_000_000_009);
    assert_eq!(x, LinSolveResult::None);
}
#[test]
fn test_linsolve_4() {
    let a = Matrix::new(vec![
        vec![1,3,-1,-2],
        vec![-1,-2,1,1],
        vec![1,1,-1,0],
        vec![2,1,-2,1],
    ]);
    let y = Matrix::new(vec![vec![-1,0,1,3]]).transpose();
    let x = LinSolve::solve(a, y, 1_000_000_009);
    assert_eq!(x, LinSolveResult::Infinite);
}
#[test]
fn test_invmat_1() {
    let a = Matrix::new(vec![
        vec![1,2,-2],
        vec![1,-1,3],
        vec![2,3,-5],
    ]);
    let a_inv = InvMatrix::solve(a, 1_000_000_009).unwrap();
    dbg!(&a_inv);
    let y = Matrix::new(vec![vec![3,4,1]]).transpose();
    let x = a_inv * y % 1_000_000_009;
    assert_eq!(x, Matrix::new(vec![vec![1,3,2]]).transpose());
}