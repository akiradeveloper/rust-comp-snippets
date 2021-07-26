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
                    let i = self.index[k].binary_search(&y).unwrap();
                    self.segs[k].update(i, v.clone());
                },
                SEGNode::Branch { k, l, r } => {
                    let mut v = M::id();
                    if let Ok(il) = self.index[l].binary_search(&y) {
                        let vl = self.segs[l].get(il);
                        v = M::op(&v, &vl);
                    }
                    if let Ok(ir) = self.index[r].binary_search(&y) {
                        let vr = self.segs[r].get(ir);
                        v = M::op(&v, &vr);
                    }
                    let i = self.index[k].binary_search(&y).unwrap();
                    self.segs[k].update(i, v);
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

#[test]
fn test_seg2d() {
    struct MAX;
    impl Monoid for MAX {
        type T = i64;
        fn id() -> Self::T {
            std::i64::MIN
        }
        fn op(a: &Self::T, b: &Self::T) -> Self::T {
            std::cmp::max(*a, *b)
        }
    }
    let mut y = vec![];
    for i in 0..5 {
        y.push(vec![0]);
    }
    let mut s: SEG2d<MAX> = SEG2d::new(y);
    s.update(0, 0, 1);
    s.update(1, 0, 2);
    s.update(2, 0, 3);
    s.update(3, 0, 2);
    s.update(4, 0, 1);
    assert_eq!(s.query(0, 1, 0, 1), 1);
    assert_eq!(s.query(0, 2, 0, 1), 2);
    assert_eq!(s.query(0, 3, 0, 1), 3);
    assert_eq!(s.query(0, 4, 0, 1), 3);
    assert_eq!(s.query(0, 5, 0, 1), 3);
    assert_eq!(s.query(2, 5, 0, 1), 3);
    assert_eq!(s.query(3, 5, 0, 1), 2);
    assert_eq!(s.query(4, 5, 0, 1), 1);
}