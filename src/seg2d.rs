use cargo_snippet::snippet;

use crate::seg::{SEG, Monoid};
use crate::seg_node::{SEGTree, SEGNode};
use std::collections::HashSet;

pub struct SEG2d<M: Monoid> {
    tree: SEGTree,
    segs: Vec<SEG<M>>,
    index: Vec<Vec<usize>>,
}
impl <M: Monoid> SEG2d<M> {
    pub fn new(xy: Vec<Vec<usize>>) -> Self {
        use std::iter::FromIterator;
        struct SetAdd {}
        impl Monoid for SetAdd {
            type T = Vec<usize>;
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
}