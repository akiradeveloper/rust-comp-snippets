use std::cmp::{max,min};

macro_rules! chmax {
    ($x:expr, $($v:expr),+) => {
        $(
            $x = std::cmp::max($x,$v);
        )+
    };
}
macro_rules! chmin {
    ($x:expr, $($v:expr),+) => {
        $(
            $x = std::cmp::min($x,$v);
        )+
    };
}
macro_rules! max {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        std::cmp::max($x, max!( $($xs),+ ))
    };
}
macro_rules! min {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        std::cmp::min($x, min!( $($xs),+ ))
    };
}

#[test]
fn test_chmax() {
    let mut dp = vec![0];
    chmax!(dp[0],1);
    assert_eq!(dp[0],1);
    chmax!(dp[0],3,2);
    assert_eq!(dp[0],3);
    chmax!(dp[0],2,1);
    assert_eq!(dp[0],3);
}

#[test]
fn test_chmin() {
    let mut dp = vec![5];
    chmin!(dp[0],4);
    assert_eq!(dp[0],4);
    chmin!(dp[0],1,2);
    assert_eq!(dp[0],1);
    chmin!(dp[0],3,2);
    assert_eq!(dp[0],1);
}

#[test]
fn test_max() {
    assert_eq!(max!(1,2,3,4,-5), 4);
}

#[test]
fn test_min() {
    assert_eq!(min!(1,2,3,4,-5), -5);
}