use cargo_snippet::snippet;

use crate::seg::{SEG, Monoid};
use crate::seg_node::{SEGTree, SEGNode};
use crate::lower_bound::LowerBound;

#[snippet("SEG2d")]
pub struct SEG2d<M: Monoid> {
    tree: SEGTree,
    segs: Vec<SEG<M>>,
    index: Vec<Vec<usize>>,
}
#[snippet("SEG2d")]
impl<M: Monoid> SEG2d<M> {
    pub fn new(i2j: Vec<Vec<usize>>) -> Self {
        let tree = SEGTree::new(i2j.len());
        let n = i2j.len().next_power_of_two();
        let mut index = vec![vec![];2*n];
        for i in 0..i2j.len() {
            let mut v = i2j[i].clone();
            v.sort(); v.dedup();
            index[i+n] = v;
        }
        let mut k = n-1;
        while k>=1 {
            let l = 2*k;
            let r = 2*k+1;
            let mut v = vec![];
            v.extend_from_slice(&index[l]);
            v.extend_from_slice(&index[r]);
            v.sort(); v.dedup();
            index[k] = v;
            k-=1;
        }
        let mut segs = vec![];
        for ii in &index {
            let s: SEG<M> = SEG::new(ii.len());
            segs.push(s);
        }
        Self { tree, index, segs }
    }
    /// 計算量
    /// O(logH logW)
    pub fn update(&mut self, i: usize, j: usize, v: M::T) {
        let nodes = self.tree.update_nodes(i);
        for node in nodes {
            match node {
                SEGNode::Leaf { k } => {
                    let i = self.index[k].binary_search(&j).unwrap();
                    self.segs[k].update(i, v.clone());
                }
                SEGNode::Branch { k, l, r } => {
                    let mut v = M::id();
                    if let Ok(il) = self.index[l].binary_search(&j) {
                        let vl = self.segs[l].get(il);
                        v = M::op(&v, &vl);
                    }
                    if let Ok(ir) = self.index[r].binary_search(&j) {
                        let vr = self.segs[r].get(ir);
                        v = M::op(&v, &vr);
                    }
                    let i = self.index[k].binary_search(&j).unwrap();
                    self.segs[k].update(i, v);
                }
            }
        }
    }
    /// [x0,x1) x [y0,y1)
    /// 計算量
    /// O(logH logW)
    pub fn query(&self, i0: usize, i1: usize, j0: usize, j1: usize) -> M::T {
        let nodes = self.tree.query_nodes(i0, i1);
        let mut ans = M::id();
        for k in nodes {
            let l = self.index[k].lower_bound(&j0);
            let r = self.index[k].lower_bound(&j1);
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