use cargo_snippet::snippet;

use crate::seg::{SEG, Monoid};
use crate::seg_node::{SEGTree, SEGNode};
use crate::lower_bound::LowerBound;

pub struct SEG2d<M: Monoid> {
    tree: SEGTree,
    segs: Vec<SEG<M>>,
    index: Vec<Vec<usize>>,
}
impl <M: Monoid> SEG2d<M> {
    pub fn new(xy: Vec<Vec<usize>>) -> Self {
        struct SetAdd {}
        impl Monoid for SetAdd {
            type T = Vec<usize>; // Set
            fn id() -> Self::T {
                vec![]
            }
            fn op(a: &Self::T, b: &Self::T) -> Self::T {
                let mut c = vec![];
                for &x in a {
                    c.push(x);
                }
                for &x in b {
                    c.push(x);
                }
                c.sort(); c.dedup();
                c
            }
        }
        let n = xy.len();
        let tree = SEGTree::new(n);
        let mut s: SEG<SetAdd> = SEG::new(n);
        for i in 0..n {
            let y = xy[i].clone();
            s.update(i, y);
        }
        let index: Vec<Vec<usize>> = s.buf;
        let mut segs = vec![];
        for ii in &index {
            let s: SEG<M> = SEG::new(ii.len());
            segs.push(s);
        }
        Self {
            tree,
            index,
            segs,
        }
    }
    /// 計算量
    /// O(logH logW)
    pub fn update(&mut self, x: usize, y: usize, v: M::T) {
        let nodes = self.tree.update_nodes(x);
        for node in nodes {
            match node {
                SEGNode::Leaf { k } => {
                    let i = self.index[k].binary_search(&y).expect("y not found");
                    self.segs[k].update(i, v.clone());
                },
                SEGNode::Branch { k, .. } => {
                    let i = self.index[k].binary_search(&y).expect("y not found");
                    self.segs[k].update(i, v.clone());
                },
            }
        }
    }
    /// [x0,x1) x [y0,y1)
    /// 計算量
    /// O(logH logW)
    pub fn query(&self, x0: usize, x1: usize, y0: usize, y1: usize) -> M::T {
        let nodes = self.tree.query_nodes(x0, x1);
        let mut ans = M::id();
        for k in nodes {
            let l = self.index[k].lower_bound(&y0);
            let r = self.index[k].lower_bound(&y1);
            let v = self.segs[k].query(l, r);
            ans = M::op(&ans, &v);
        }
        ans
    }
}