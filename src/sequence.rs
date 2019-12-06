use super::lower_bound::LowerBound;
use super::fenwick::BIT;
use std::cmp::{min, max};

fn lis<T: Ord + Clone>(xs: &[T], inf: T) -> Vec<T> {
    let n = xs.len();
    let mut dp = vec![inf; n];
    for x in xs {
        let i = dp.lower_bound(&x);
        dp[i] = min(dp[i].clone(), x.clone());
    }
    return dp;
}
#[test]
fn test_lis() {
    let xs = vec![1,3,2,4,6,5];
    let dp = lis(&xs, 1<<30);
    dbg!(&dp);
}

fn lcs<T: Eq>(xs: &[T], ys: &[T]) -> Vec<Vec<usize>> {
    let n = xs.len();
    let m = ys.len();
    let mut dp = vec![vec![0; m]; n];
    for i in 0..n {
        let x = &xs[i];
        let mut found = false;
        for j in 0..m {
            if &ys[j] == x {
                found = true;
            }
            dp[i][j] = if found {
                1
            } else { 0 };
        }
    }
    for j in 0..m {
        let y = &ys[j];
        let mut found = false;
        for i in 0..n {
            if &xs[i] == y {
                found = true;
            }
            dp[i][j] = if found {
                1
            } else { 0 };
        }
    }
    for i in 1..n {
        for j in 1..m {
            dp[i][j] = if xs[i] == ys[j] {
                dp[i-1][j-1] + 1
            } else {
                max(dp[i][j-1], dp[i-1][j])
            };
        }
    }
    return dp;
}
#[test]
fn test_lcs() {
    let xs = vec![6,9,2,8,3,7,4,6];
    let ys = vec![1,2,2,4,5,3,3,3,2,4];
    let dp = lcs(&xs, &ys);
    dbg!(dp);
}

#[doc = "O(N log N)"]
fn inversion(xs: &[usize]) -> Vec<usize> {
    let mut max_v = 0;
    for &x in xs {
        max_v = max(max_v, x);
    }
    let mut res = vec![];
    let mut bit = BIT::new(max_v+1);
    for i in 0..xs.len() {
        let x = xs[i];
        let cnt = bit.sum(x); // cnt of <= x
        res.push(i-cnt);
        bit.add(x, 1);
    }
    res
}
#[test]
fn test_inversion() {
    let xs = vec![1,3,2,4,2,1];
    assert_eq!(inversion(&xs), [0,0,1,0,2,4]);
}

#[snippet = "run_length_compression"]
#[doc = "O(N)"]
fn run_length_compression<T: Eq + Clone>(xs: &[T]) -> Vec<(T, usize)> {
    if xs.is_empty() {
        return vec![]
    }
    let mut cur = &xs[0];
    let mut rep = 1;
    let mut res = vec![];
    for i in 1..xs.len() {
        if &xs[i] == cur {
            rep += 1;
        } else {
            res.push((cur.clone(),rep));
            cur = &xs[i];
            rep = 1
        }
    }
    res.push((cur.clone(),rep));
    res
}
#[test]
fn test_run_length_compression() {
    let emp: Vec<bool> = vec![];
    assert_eq!(run_length_compression(&emp), vec![]);
    assert_eq!(run_length_compression(&vec![true]), vec![(true,1)]);
    assert_eq!(run_length_compression(&vec![2,3,3,3,2,2]), vec![(2,1),(3,3),(2,2)]);
}

#[snippet = "group_by_relevance"]
pub fn group_by_relevance<T, F: Fn(&T,&T) -> bool>(xs: Vec<T>, f: F) -> Vec<Vec<T>> {
    let mut res = vec![];

    if xs.len() == 1 {
        res.push(xs);
        return res
    }

    let mut xs = xs;
    let n = xs.len();
    let mut l = 0;
    let mut sep = vec![];
    for i in 0..n-1 {
        if !f(&xs[i], &xs[i+1]) {
            sep.push(i+1-l);
            l = i+1;
        }
    }
    sep.push(n-l);

    xs.reverse();

    for len in sep {
        let mut cur = vec![];
        for _ in 0..len {
            cur.push(xs.pop().unwrap());
        }
        res.push(cur);
    }

    res
}
#[test]
fn test_group_by_relevance() {
    assert_eq!(group_by_relevance(vec![1,2,3,1,2,3], |&a,&b| { a<b }), vec![vec![1,2,3],vec![1,2,3]]);
    assert_eq!(group_by_relevance(vec![3,2,1,3,2,1], |&a,&b| { a<b }), vec![vec![3],vec![2],vec![1,3],vec![2],vec![1]]);
    assert_eq!(group_by_relevance(vec![1,1,2,2,3,3], |&x,&y| { x == y }), vec![vec![1,1],vec![2,2],vec![3,3]]);
}

#[snippet = "group_fold"]
#[doc = "fold elems in to groups by f"]
pub fn group_fold<T, F: Fn(&T) -> G, G: Eq+Clone>(xs: Vec<T>, f: F) -> Vec<Vec<T>> {
    let mut res = vec![];
    let mut cur_g = None;
    let mut tmp = vec![];
    for x in xs {
        let g = Some(f(&x));
        if g != cur_g {
            if !tmp.is_empty() {
                res.push(tmp);
            }
            tmp = vec![x];
            cur_g = g;
        } else {
            tmp.push(x);
        }
    }
    if !tmp.is_empty() {
        res.push(tmp);
    }
    res
}
#[test]
fn test_group_fold() {
    let emp: Vec<usize> = vec![];
    assert_eq!(group_fold(vec![1,2,1], |&x| {x}), vec![vec![1],vec![2],vec![1]]);
    assert_eq!(group_fold(vec![('L',1),('L',3),('R',2),('L',1)], |&x| {x.0}), vec![vec![('L',1),('L',3)],vec![('R',2)],vec![('L',1)]]);
}

#[snippet = "kadane"]
#[doc = "return the sum of the maximum subarray. (Kadane's algorithm). O(N)"]
fn kadane(xs: &[i64]) -> i64 {
    let mut max_ending_here = xs[0];
    let mut max_so_far = xs[0];
    for i in 1..xs.len() {
        let x=xs[i];
        max_ending_here=max(x,max_ending_here+x);
        max_so_far=max(max_so_far,max_ending_here);
    }
    max_so_far
}
#[test]
fn test_kadane() {
    assert_eq!(kadane(&[1,-2,1,1,-2,1]),2);
    assert_eq!(kadane(&[1,5,-1,0,10]),15);
    assert_eq!(kadane(&[0,-1,-5,0,-4]),0);
}

#[snippet = "vec_max"]
fn vec_max<T: Ord + Clone>(xs: &[T]) -> T {
    let mut v = &xs[0];
    for x in xs {
        if x > v {
            v = x;
        }
    }
    v.clone()
}

#[snippet = "vec_min"]
fn vec_min<T: Ord + Clone>(xs: &[T]) -> T {
    let mut v = &xs[0];
    for x in xs {
        if x < v {
            v = x;
        }
    }
    v.clone()
}

#[snippet = "neighbour_table"]
pub fn neighbour_table(xs: &[usize]) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let n = xs.len();
    let mut m = 0;
    for i in 0..n {
        m = max(m, xs[i]);
    }
    m += 1;
    let mut next = vec![None; m];
    let mut cur = 0;
    for i in 0..n {
        let x = xs[i];
        next[cur] = Some(x);
        cur = x;
    }
    let mut prev = vec![None; m];
    let mut cur = m-1;
    for i in (0..n).rev() {
        let x = xs[i];
        prev[cur] = Some(x);
        cur = x;
    }

    (prev, next)
}
#[test]
fn test_neighbour_table() {
    let xs = vec![1,5,3,6,9];
    let (prev, next) = neighbour_table(&xs);
    assert_eq!(prev[1], None);
    assert_eq!(prev[5], Some(1));
    assert_eq!(prev[3], Some(5));
    assert_eq!(next[1], Some(5));
    assert_eq!(next[5], Some(3));
    assert_eq!(next[3], Some(6));
    assert_eq!(next[9], None);
}