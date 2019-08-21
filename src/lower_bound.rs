/// https://github.com/hatoo/competitive-rust-snippets

use std::cmp::Ordering;

#[snippet = "lower_bound"]
/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearchCpp<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

#[snippet = "lower_bound"]
impl<T: Ord> BinarySearchCpp<T> for [T] {
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