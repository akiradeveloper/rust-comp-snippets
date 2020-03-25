use cargo_snippet::snippet;

#[snippet("sliding_window")]
pub fn sliding_window<T, F: Fn(&T) -> K, K: Ord>(a: Vec<T>, k: usize, f: F) -> Vec<usize> {
    use std::collections::VecDeque;
    let n = a.len();
    let mut ans = vec![n;n];
    let mut q = VecDeque::new();
    for i in 0..n {
        while !q.is_empty() && f(&a[*q.back().unwrap()]) > f(&a[i]) {
            q.pop_back();
        }
        q.push_back(i);
        while !q.is_empty() && *q.front().unwrap() + k <= i {
            q.pop_front();
        }
        dbg!(&q);
        ans[i] = *q.front().unwrap();
    }
    ans
}
#[test]
fn test_sliding_window() {
    let mut a = vec![3,2,1,1,2,3];
    let res = sliding_window(a.clone(), 2, |&x| { x });
    assert_eq!(res, vec![0,1,2,2,3,4]);
    use crate::rev::Rev;
    let res = sliding_window(a.clone(), 2, |&x| Rev(x));
    assert_eq!(res, vec![0,0,1,2,4,5]);
}