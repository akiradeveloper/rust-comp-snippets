use super::lower_bound::BinarySearchCpp;
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

fn inversion(xs: &[usize]) -> Vec<usize> {
    let mut max_v = 0;
    for &x in xs {
        max_v = max(max_v, x);
    }
    let mut res = vec![];
    let mut bit = BIT::new(max_v+1, &0, |a: &mut usize, b: &usize| *a += b);
    for i in 0..xs.len() {
        let x = xs[i];
        let cnt = bit.sum_excl(x+1); // cnt of <= x
        res.push(i-cnt);
        bit.add_0_orig(x, &1);
    }
    res
}
#[test]
fn test_inversion() {
    let xs = vec![1,3,2,4,2,1];
    assert_eq!(inversion(&xs), [0,0,1,0,2,4]);
}