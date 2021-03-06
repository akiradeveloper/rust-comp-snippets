use cargo_snippet::snippet;

#[snippet("cumsum1")]
struct CumSum1 {
    base: Vec<i64>,
    dp: Vec<i64>,
}
#[snippet("cumsum1")]
impl CumSum1 {
    pub fn new(n: usize) -> Self {
        CumSum1 {
            base: vec![0; n],
            dp: vec![],
        }
    }
    pub fn from_vec(a: Vec<i64>) -> Self {
        let n = a.len();
        let mut x = Self::new(n);
        for i in 0..n {
            x.set(i,a[i]);
        }
        x.build();
        x
    }
    pub fn add(&mut self, i: usize, x: i64) {
        self.base[i] += x;
    }
    pub fn set(&mut self, i: usize, x: i64) {
        self.base[i] = x;
    }
    pub fn build(&mut self) {
        let n = self.base.len();
        let mut dp = vec![0; n+1];
        let mut acc = 0;
        for i in 0..n {
            acc += self.base[i];
            dp[i+1] = acc;
        }
        self.dp = dp;
    }
    #[doc = "[i,j)"]
    pub fn query(&self, i: usize, j: usize) -> i64 {
        self.dp[j] - self.dp[i]
    }
}

#[test]
fn test_cumsum1() {
    let x = vec![0,1,2,1];
    let mut cs = CumSum1::new(4);
    for i in 0..4 {
        cs.set(i,x[i]);
    }
    cs.build();
    assert_eq!(cs.query(0,0), 0);
    assert_eq!(cs.query(0,1), 0);
    assert_eq!(cs.query(0,2), 1);
    assert_eq!(cs.query(0,3), 3);
    assert_eq!(cs.query(0,4), 4);
}

/// 二次元平面上の累積和
/// dp[i][j]が[0,i)x[0,j)として計算出来ると、
/// 任意の範囲の累積和が計算出来るようになる。
/// 
/// 計算量:
/// 構築 O(N^2)
/// クエリ O(1)

#[snippet("cumsum2")]
pub struct CumSum2 {
    base: Vec<Vec<i64>>,
    dp: Vec<Vec<i64>>,
}
#[snippet("cumsum2")]
impl CumSum2 {
    pub fn new(n: usize, m: usize) -> CumSum2 {
        CumSum2 {
            base: vec![vec![0;m];n],
            dp: vec![]
        }
    }
    pub fn add(&mut self, i: usize, j: usize, x: i64) {
        self.base[i][j] += x;
    }
    pub fn set(&mut self, i: usize, j: usize, x: i64) {
        self.base[i][j] = x;
    }
    pub fn build(&mut self) {
        let n = self.base.len();
        let m = self.base[0].len();
        let mut dp = vec![vec![0; m+1]; n+1];
        for i in 0..n {
            for j in 0..m {
                dp[i+1][j+1] = self.base[i][j];
            }
        }
        for i in 1..n+1 {
            for j in 1..m+1 {
                dp[i][j] += dp[i-1][j] + dp[i][j-1] - dp[i-1][j-1];
            }
        }
        self.dp = dp;
    }
    pub fn query(&self, i0: usize, i1: usize, j0: usize, j1: usize) -> i64 {
        self.dp[i1][j1] - (self.dp[i0][j1] + self.dp[i1][j0] - self.dp[i0][j0])
    }
}
#[test]
fn test_cum2() {
    let mut cum2 = CumSum2::new(2,2);
    cum2.set(0,0,1); cum2.set(0,1,2);
    cum2.set(1,0,3); cum2.set(1,1,4);
    cum2.build();
    assert_eq!(cum2.query(0, 2, 0, 2), 10);
    assert_eq!(cum2.query(0, 1, 1, 2), 2);
    assert_eq!(cum2.query(1, 2, 1, 2), 4);
    assert_eq!(cum2.query(0, 1, 0, 2), 3);
}

/// いもす法（２次元）
/// 
/// [i0,i1)x[j0,j1)の領域に範囲加算をしていったあと、
/// i,jの値を調べる。
/// 横方向、縦方向のスイープを行うだけで構築可能。
/// 
/// 計算量:
/// 構築 O(N+M)
/// クエリ O(1)

#[snippet("Imosu2d")]
pub struct Imosu2d {
    n: usize,
    m: usize,
    dp: Vec<Vec<i64>>,
}
#[snippet("Imosu2d")]
impl Imosu2d {
    pub fn new(n: usize, m: usize) -> Imosu2d {
        Imosu2d {
            n: n,
            m: m,
            dp: vec![vec![0;m+1];n+1],
        }
    }
    pub fn add(&mut self, i0: usize, i1: usize, j0: usize, j1: usize, x: i64) {
        self.dp[i0][j0] += x;
        self.dp[i0][j1] += -x;
        self.dp[i1][j0] += -x;
        self.dp[i1][j1] += x;
    }
    pub fn build(&mut self) {
        // right sweep
        for i in 0..self.n+1 {
            for j in 0..self.m {
                self.dp[i][j+1] += self.dp[i][j];
            }
        }
        // down sweep
        for j in 0..self.m+1 {
            for i in 0..self.n {
                self.dp[i+1][j] += self.dp[i][j];
            }
        }
    }
    pub fn get(&self, i: usize, j: usize) -> i64 {
        self.dp[i][j]
    }
}
#[test]
fn test_imosu_2d() {
    let mut tbl = vec![vec![0;5];5];
    let mut imosu = Imosu2d::new(5,5);

    let tests = vec![
        (0,3,0,3,5),
        (2,4,1,3,8),
        (1,3,2,4,3),
    ];

    for t in tests {
        let (i0,i1,j0,j1,x) = t;
        for i in i0..i1 {
            for j in j0..j1 {
                tbl[i][j] += x;
            }
        }
        imosu.add(i0,i1,j0,j1,x);
    }

    imosu.build();
    for i in 0..5 {
        for j in 0..5 {
            assert_eq!(imosu.get(i,j), tbl[i][j]);
        }
    }
}

use crate::arith_seq::ArithSeq;

/// いもす法（１次元）
/// 
/// いもす法の１次元ならば、２次元の時のように均一の値ではなく、
/// 一次式ax+yを足し合わせることが出来る。
/// なぜならば、
/// aの配列をスイープすることによって
/// a,2a,3a,...の
/// 足し合わせをO(N)で計算出来るから。
/// 
/// 計算量:
/// 構築 O(N)
/// クエリ O(1)

#[snippet("Imosu1d")]
pub struct Imosu1d {
    n: usize,
    dp1: Vec<i64>,
    dp2: Vec<i64>,
    dp3: Vec<i64>,
}
#[snippet("Imosu1d")]
impl Imosu1d {
    pub fn new(n: usize) -> Imosu1d {
        Imosu1d {
            n: n,
            // オフセット（左端の値）についてのスイープ用
            dp1: vec![0;n+1],
            // aに関するスイープ用
            dp2: vec![0;n+1],
            dp3: vec![0;n+1],
        }
    }
    #[doc = "y=ax+b"]
    pub fn add_line(&mut self, l: usize, r: usize, line: ArithSeq) {
        if l >= r { return; }

        let a = line.a;
        let b = line.b;

        let y = line.y(l as i64);
        self.dp1[l] += y;
        self.dp1[r] += -y;

        self.dp2[l+1] += a;
        self.dp2[r] += -a;

        let L = r - (l+1);
        self.dp3[r] += -a * L as i64;
    }
    fn sweep(dp: &mut [i64]) {
        let mut cur = 0;
        for i in 0..dp.len() {
            cur += dp[i];
            dp[i] = cur;
        }
    }
    pub fn build(&mut self) {
        let n = self.dp1.len();
        Self::sweep(&mut self.dp1);
        Self::sweep(&mut self.dp2);
        for i in 0..n {
            self.dp2[i] += self.dp3[i];
        }
        Self::sweep(&mut self.dp2);
        for i in 0..n {
            self.dp1[i] += self.dp2[i];
        }
    }
    pub fn get(&self, k: usize) -> i64 {
        self.dp1[k]
    }
}

#[test]
fn test_imosu_1d() {
    let mut imosu = Imosu1d::new(5);
    imosu.add_line(0,3,ArithSeq::new(3,1)); // [1,4,7,0,0]
    imosu.add_line(1,5,ArithSeq::new(-2,-1)); // [0,-3,-5,-7,-9]
    imosu.build();
    assert_eq!(imosu.dp1, vec![1,1,2,-7,-9,0]);
}