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

#[snippet = "split_sequence"]
#[derive(Debug)]
pub enum SplitComp<T> {
    Seq(Vec<T>),
    Splitter(T),
}
#[snippet = "split_sequence"]
pub fn split_sequence<T, F: Fn(&T) -> bool>(xs: Vec<T>, splitter: F) -> Vec<SplitComp<T>> {
    let mut res = vec![];
    let mut xs = xs;
    xs.reverse();
    let mut cur = vec![];
    while !xs.is_empty() {
        let x = xs.pop().unwrap();
        let b = splitter(&x);
        if b {
            if cur.len() > 0 {
                res.push(SplitComp::Seq(cur));
            }
            res.push(SplitComp::Splitter(x));
            cur = vec![];
        } else {
            cur.push(x);
        }
    }
    if cur.len() > 0 {
        res.push(SplitComp::Seq(cur));
    }
    res
}
#[test]
fn test_split_sequence() {
    let mut t1 = vec![1,2,3,4];
    dbg!(split_sequence(t1, |&x| { x == 2 }));
    let mut t2 = vec![1,2,3,4];
    dbg!(split_sequence(t2, |&x| { x == 1 }));
    let mut t3 = vec![1,3,2,4,1];
    dbg!(split_sequence(t3, |&x| { x >= 3 }));
}