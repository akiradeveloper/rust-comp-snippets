use cargo_snippet::snippet;

use crate::seg::{SEG, Monoid};

#[snippet("SEGNode")]
#[derive(PartialEq, Debug)]
pub enum Node {
    Leaf { k: usize },
    Branch { k: usize, l: usize, r: usize },
}
#[snippet("SEGNode")]
pub struct SEGTree {
    /// 葉の数（２の累乗）
    pub n: usize,
}
#[snippet("SEGNode")]
impl SEGTree {
    pub fn new(n: usize) -> SEGTree {
        let n = n.next_power_of_two();
        Self {
            n: n,
        }
    }
    pub fn update_nodes(&self, i: usize) -> Vec<Node> {
        use Node::*;
        let mut i = i+self.n;
        let mut res = vec![Leaf { k: i }];
        while i>1 {
            i >>= 1;
            res.push(Branch { k: i, l: i*2, r: i*2+1 });
        }
        res
    }
    /// [l,r)
    pub fn query_nodes(&self, l: usize, r: usize) -> Vec<usize> {
        let mut ret = vec![];
        let mut l = l + self.n;
        let mut r = r + self.n;
        while l < r {
            if l&1 > 0 {
                ret.push(l);
                l += 1;
            }
            if r&1 > 0 {
                r -= 1;
                ret.push(r);
            }
            l >>= 1;
            r >>= 1;
        }
        ret.sort();
        ret
    }
}
#[test]
fn test_update_nodes() {
    use Node::*;
    let t = SEGTree::new(3);
    assert_eq!(t.update_nodes(0), vec![Leaf{k:4},Branch{k:2,l:4,r:5},Branch{k:1,l:2,r:3}]);
    assert_eq!(t.update_nodes(1), vec![Leaf{k:5},Branch{k:2,l:4,r:5},Branch{k:1,l:2,r:3}]);
    assert_eq!(t.update_nodes(2), vec![Leaf{k:6},Branch{k:3,l:6,r:7},Branch{k:1,l:2,r:3}]);
}
#[test]
fn test_query_nodes() {
    let t = SEGTree::new(3);
    assert_eq!(t.query_nodes(0,1), vec![4]);
    assert_eq!(t.query_nodes(1,2), vec![5]);
    assert_eq!(t.query_nodes(0,2), vec![2]);
    assert_eq!(t.query_nodes(1,3), vec![5,6]);
    assert_eq!(t.query_nodes(1,4), vec![3,5]);
    assert_eq!(t.query_nodes(0,4), vec![1]);
}

pub struct SEG2d {

}