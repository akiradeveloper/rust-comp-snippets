// Verified: ECR64-D

use cargo_snippet::snippet;

/// 全方位木DP
///  
/// 木において、ある頂点uの入辺v->uの値をDP[u,v]で表すことにする。
/// これを効率的に求めるアルゴリズムである。
/// 
/// 解説: https://www.akiradeveloper.com/post/algorithm-rerooting/
///
/// 木であるが、辺は有向として、各方向に値を格納出来るようにしている。
/// （アルゴリズムの実行のためにはbidirectionalにする必要がある）
/// 頂点や辺の値は、DPの計算に利用することが出来る。

#[snippet("CumRL")]
trait Foldable {
    type T: Clone + std::fmt::Debug;
    fn id() -> Self::T;
    fn fold(acc: Self::T, x: Self::T) -> Self::T;
}
#[snippet("CumRL")]
#[derive(Clone, Debug)]
struct CumRL<F: Foldable> {
    lcum: Vec<F::T>,
    rcum: Vec<F::T>,
}
#[snippet("CumRL")]
impl <F: Foldable> CumRL<F> {
    pub fn new(elems: Vec<F::T>) -> CumRL<F> {
        let n = elems.len();
        let mut lcum = vec![F::id()];
        for i in 0..n {
            let x = F::fold(lcum[i].clone(), elems[i].clone());
            lcum.push(x);
        }
        let mut elems = elems;
        elems.reverse();
        let mut rcum = vec![F::id()];
        for i in 0..n {
            let x = F::fold(rcum[i].clone(), elems[i].clone());
            rcum.push(x);
        }
        CumRL {
            lcum: lcum,
            rcum: rcum,
        }
    }
    pub fn len(&self) -> usize {
        self.lcum.len() - 1
    }
    pub fn lcum(&self, len: usize) -> F::T {
        self.lcum[len].clone()
    }
    pub fn rcum(&self, len: usize) -> F::T {
        self.rcum[len].clone()
    }
}
#[test]
fn test_cumrl() {
    struct Sum;
    impl Foldable for Sum {
        type T = i64;
        fn id() -> i64 { 0 }
        fn fold(acc: i64, x: i64) -> i64 { acc + x }
    }
    let v = vec![1,2,3,4,5];
    let mut cum: CumRL<Sum> = CumRL::new(v);
    assert_eq!(cum.lcum(1), 1);
    assert_eq!(cum.lcum(3), 6);
    assert_eq!(cum.lcum(5), 15);
    assert_eq!(cum.rcum(1), 5);
    assert_eq!(cum.rcum(2), 9);
    assert_eq!(cum.rcum(5), 15);
}

use std::collections::HashMap;

#[snippet("ZenHo")]
trait ZenHoable: Foldable + Clone + Sized {
    type NVal: Clone;
    type EVal: Clone;
    /// rerootingした時に新しいDP値を計算するために呼ばれる。
    /// uの下にあったvを持ち上げてu->vの値を計算する場合、
    /// nvalue: uの値
    /// evalue: u->vの値
    fn f(&self, nvalue: Self::NVal, evalue: Self::EVal, dp: &CumRL<Self>, l: usize, r: usize) -> Self::T;
}
#[snippet("ZenHo")]
#[derive(Debug)]
struct ZenHo<Z: ZenHoable> {
    z: Z,
    g: Vec<Vec<usize>>,
    nvalues: Vec<Z::NVal>,
    evalues: HashMap<(usize,usize), Z::EVal>,
    dp: HashMap<(usize,usize), Z::T>,
}
#[snippet("ZenHo")]
impl <Z: ZenHoable> ZenHo<Z> {
    pub fn new(z: Z, nvalues: Vec<Z::NVal>) -> ZenHo<Z> {
        let n = nvalues.len();
        ZenHo {
            z: z,
            g: vec![vec![]; n],
            nvalues: nvalues, 
            evalues: HashMap::new(),
            dp: HashMap::new(),
        }
    }
    pub fn n(&self) -> usize {
        self.g.len()
    }
    pub fn add_edge(&mut self, u: usize, v: usize, eval: Z::EVal) {
        self.g[u].push(v);
        self.evalues.insert((u,v), eval);
    }
    fn init_dfs(&mut self, par: Option<usize>, u: usize) {
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            if Some(v) == par { continue; }
            self.init_dfs(Some(u), v);
        }
        if let Some(p) = par {
            let mut dp = vec![];
            for i in 0..self.g[u].len() {
                let v = self.g[u][i];
                if Some(v) == par { continue; }
                let dpval = self.dp.get(&(v,u)).cloned().unwrap();
                dp.push(dpval);
            }
            let cumRL: CumRL<Z> = CumRL::new(dp);
            let newv = self.z.f(self.nvalues[u].clone(), self.evalues.get(&(u,p)).cloned().unwrap(), &cumRL, cumRL.len(), 0);
            self.dp.insert((u,p), newv);
        }
    }
    fn reroot_bfs(&mut self, par: Option<usize>, u: usize) {
        let mut dp = vec![];
        // we have dp v->u
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            let x = self.dp.get(&(v,u)).cloned().unwrap();
            dp.push(x);
        }
        let cum: CumRL<Z> = CumRL::new(dp);
        let n = cum.len();
        // let's make value dp u->v
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            let L = i;
            let R = n-1-L;
            let newv = self.z.f(self.nvalues[u].clone(), self.evalues.get(&(u,v)).cloned().unwrap(), &cum, L, R);
            self.dp.insert((u,v), newv);
        }
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            if Some(v) == par { continue; }
            self.reroot_bfs(Some(u), v);
        }
    }
    #[doc = "O(n)"]
    pub fn build(&mut self, root: usize) {
        self.init_dfs(None, root);
        self.reroot_bfs(None, root);
    }
    pub fn calc(&self, u: usize, v: usize) -> Z::T {
        self.dp.get(&(u,v)).cloned().unwrap()
    }
}

#[test]
fn test_zenho() {
    use super::*;
    #[derive(Clone, Debug)]
    struct M;
    impl Foldable for M {
        type T = usize;
        fn id() -> usize { 0 }
        fn fold(acc: usize, x: usize) -> usize {
            acc + x
        }
    }
    impl ZenHoable for M {
        type NVal = usize;
        type EVal = usize;
        fn f(&self, _: usize, _: usize, cum: &CumRL<Self>, l: usize, r: usize) -> usize {
            let mut tot = 0;
            tot += cum.lcum(l);
            tot += cum.rcum(r);
            tot += 1;
            tot
        }
    }
    let mut zenho: ZenHo<M> = ZenHo::new(M, vec![0;5]);
    let E = vec![(0,1),(0,2),(0,3),(2,4)];
    for (u,v) in E {
        zenho.add_edge(u, v, 0);
        zenho.add_edge(v, u, 0);
    }
    zenho.build(0);
    dbg!(&zenho.dp);
    
    let ans = vec![
        (1,0,1),
        (2,0,2),
        (3,0,1),
        (4,2,1),
        (0,1,4),
        (0,2,3),
        (0,3,4),
        (2,4,4),
    ];
    for (u,v,should_be) in ans {
        assert_eq!(zenho.calc(u,v), should_be);
    }
}