#[snippet = "CumRL"]
trait Foldable {
    type T: Clone;
    fn fold(acc: Self::T, x: Self::T) -> Self::T;
}
#[snippet = "CumRL"]
#[derive(Clone, Debug)]
struct CumRL<F: Foldable> {
    lcum: Vec<F::T>,
    rcum: Vec<F::T>,
}
#[snippet = "CumRL"]
impl <F: Foldable> CumRL<F> {
    pub fn new(elems: Vec<F::T>) -> CumRL<F> {
        let n = elems.len();
        let fi = elems[0].clone();
        let mut lcum = vec![elems[0].clone()];
        for i in 1..n {
            lcum.push(F::fold(lcum[i-1].clone(), elems[i].clone()));
        }
        let mut elems = elems;
        elems.reverse();
        let mut rcum = vec![elems[0].clone()];
        for i in 1..n {
            rcum.push(F::fold(rcum[i-1].clone(), elems[i].clone()));
        }
        CumRL {
            lcum: lcum,
            rcum: rcum,
        }
    }
    pub fn len(&self) -> usize {
        self.lcum.len()
    }
    pub fn lcum(&self, len: usize) -> F::T {
        self.lcum[len-1].clone()
    }
    pub fn rcum(&self, len: usize) -> F::T {
        self.rcum[len-1].clone()
    }
}
#[test]
fn test_cumrl() {
    struct Sum;
    impl Foldable for Sum {
        type T = i64;
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

#[snippet = "ZenHo"]
trait ZenHoable: Foldable + Clone + Sized {
    type NVal: Clone;
    type EVal: Clone;
    fn f(nvalue: Self::NVal, evalue: Self::EVal, dp: &[Self::T]) -> Self::T;
    fn g(nvalue: Self::NVal, evalue: Self::EVal, dp: &CumRL<Self>, L: usize, R: usize) -> Self::T;
}
#[derive(Debug)]
#[snippet = "ZenHo"]
struct ZenHo<Z: ZenHoable> {
    g: Vec<Vec<usize>>,
    nvalues: Vec<Z::NVal>,
    evalues: HashMap<(usize,usize), Z::EVal>,
    dp: HashMap<(usize,usize), Z::T>,
    rootcum: Vec<Option<CumRL<Z>>>,
}
#[snippet = "ZenHo"]
impl <Z: ZenHoable> ZenHo<Z> {
    pub fn new(nvalues: Vec<Z::NVal>) -> ZenHo<Z> {
        let n = nvalues.len();
        ZenHo {
            g: vec![vec![]; n],
            nvalues: nvalues, 
            evalues: HashMap::new(),
            dp: HashMap::new(),
            rootcum: vec![None; n],
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
            let newv = Z::f(self.nvalues[u].clone(), self.evalues.get(&(u,p)).cloned().unwrap(), &dp);
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
            let R = n-1-i;
            let newv = Z::g(self.nvalues[u].clone(), self.evalues.get(&(u,v)).cloned().unwrap(), &cum, L, R);
            self.dp.insert((u,v), newv);
        }
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            self.reroot_bfs(Some(u), v);
        }
    }
    #[doc = "O(n)"]
    pub fn build(&mut self, root: usize) {
        self.init_dfs(None, root);
        // self.reroot_bfs(None, root);
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
        fn fold(acc: usize, x: usize) -> usize {
            acc + x
        }
    }
    impl ZenHoable for M {
        type NVal = usize;
        type EVal = usize;
        fn f(n: usize, e: usize, dp: &[usize]) -> usize {
            let mut tot = 0;
            for &x in dp {
                tot += x
            }
            tot += 1;
            tot
        }
        fn g(n: usize, e: usize, cum: &CumRL<Self>, l: usize, r: usize) -> usize {
            let mut tot = 0;
            if l>0 {
                tot += cum.lcum(l);
            }
            if r>0 {
                tot += cum.rcum(r);
            }
            tot += 1;
            tot
        }
    }
    let mut zenho: ZenHo<M> = ZenHo::new(vec![0;5]);
    let E = vec![(0,1),(0,2),(0,3),(2,4)];
    for (u,v) in E {
        zenho.add_edge(u, v, 0);
        zenho.add_edge(v, u, 0);
    }
    zenho.build(0);
    dbg!(&zenho);
}