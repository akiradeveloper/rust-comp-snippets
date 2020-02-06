#[snippet = "CumRL"]
trait Foldable {
    type T: Clone;
    fn fold(acc: Self::T, x: Self::T) -> Self::T;
}
#[snippet = "CumRL"]
#[derive(Clone)]
struct CumRL<F: Foldable> {
    lcum: Vec<F::T>,
    rcum: Vec<F::T>,
}
#[snippet = "CumRL"]
impl <F: Foldable> CumRL<F> {
    fn new(elems: Vec<F::T>) -> CumRL<F> {
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
    fn len(&self) -> usize {
        self.lcum.len()
    }
    fn lcum(&self, len: usize) -> F::T {
        self.lcum[len-1].clone()
    }
    fn rcum(&self, len: usize) -> F::T {
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

trait ZenHoable: Foldable + Clone + Sized {
    type NVal: Clone;
    type EVal: Clone;
    type DPVal: Clone;
    fn f(nvalue: Self::NVal, evalue: Self::EVal, dp: &[Self::DPVal]) -> Self::DPVal;
    fn g(nvalue: Self::NVal, evalue: Self::EVal, dp: &CumRL<Self>) -> Self::DPVal;
}
struct ZenHo<Z: ZenHoable> {
    g: Vec<Vec<usize>>,
    nvalues: Vec<Z::NVal>,
    evalues: HashMap<(usize,usize), Z::EVal>,
    dp: HashMap<(usize,usize), Z::DPVal>,
    rootcum: Vec<Option<CumRL<Z>>>,
}
impl <Z: ZenHoable> ZenHo<Z> {
    fn new(nvalues: Vec<Z::NVal>) -> ZenHo<Z> {
        let n = nvalues.len();
        ZenHo {
            g: vec![vec![]; n],
            nvalues: nvalues, 
            evalues: HashMap::new(),
            dp: HashMap::new(),
            rootcum: vec![None; n],
        }
    }
    fn n(&self) -> usize {
        self.g.len()
    }
    fn add_edge(&mut self, u: usize, v: usize, eval: Z::EVal) {
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
            let e = (u,p);
            let mut dp = vec![];
            for i in 0..self.g[u].len() {
                let v = self.g[u][i];
                if Some(v) == par { continue; }
                let dpval: Z::DPVal = self.dp.get(&(v,u)).cloned().unwrap();
                dp.push(dpval);
            }
            let newv: Z::DPVal = Z::f(self.nvalues[u].clone(), self.evalues.get(&e).cloned().unwrap(), &dp);
            self.dp.insert(e, newv);
        }
    }
    fn reroot_dfs(&mut self, par: Option<usize>, u: usize) {
    }
    fn build(&mut self, root: usize) {

    }
}