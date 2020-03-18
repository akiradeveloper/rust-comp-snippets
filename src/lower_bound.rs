/// https://github.com/hatoo/competitive-rust-snippets

use cargo_snippet::snippet;
use std::cmp::Ordering;

#[snippet("lower_bound")]
/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait LowerBound<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

#[snippet("lower_bound")]
impl<T: Ord> LowerBound<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

#[test]
fn test() {
    let vec = vec![1, 2, 4, 6, 7, 12, 54, 60];
    assert_eq!(vec.lower_bound(&4), 2);
    assert_eq!(vec.upper_bound(&4), 3);
} 

use std::collections::HashMap;

#[snippet("RangeCompression")]
struct RangeCompression {
    inv: HashMap<i64, usize>,
    ranges: Vec<i64>,
}
#[snippet("RangeCompression")]
impl RangeCompression {
    pub fn new(ranges: Vec<i64>) -> RangeCompression {
        let n = ranges.len();
        let mut ranges = ranges;
        ranges.sort();
        let mut inv = HashMap::new();
        for i in 0..n {
            inv.insert(ranges[i], i);
        }
        RangeCompression {
            ranges: ranges,
            inv: inv,
        }
    }
    pub fn index(&self, i: i64) -> usize {
        let ri = self.ranges.lower_bound(&i);
        let r = self.ranges[ri];
        *self.inv.get(&r).unwrap()
    }
}

#[test]
fn test_range_compression() {
    let rc = RangeCompression::new(vec![3,7,0,100]);
    let ts = vec![
        (0,0),
        (1,1),(2,1),(3,1),
        (4,2),(5,2),(6,2),(7,2),
        (8,3),(90,3),(100,3),
    ];
    for (x,i) in ts {
        assert_eq!(rc.index(x), i);
    }
}