//! Utilitys for testing.
//! This module doesn't contains snippet.

/// https://github.com/hatoo/competitive-rust-snippets

use rand::Rng;
use std::cmp::{max, min};
use std::ops::Range;

/// return a..b
/// l <= a <= b <= r
#[allow(dead_code)]
pub fn random_range<R: Rng>(rand: &mut R, l: usize, r: usize) -> Range<usize> {
    let a = l + rand.next_u32() as usize % (r - l + 1);
    let b = l + rand.next_u32() as usize % (r - l + 1);

    min(a, b)..max(a, b)
}

#[snippet = "vec_to_string"]
pub fn vec_to_string<T: ToString>(xs: &[T]) -> Vec<String> {
    let mut res = vec![];
    for x in xs {
        res.push(x.to_string());
    }
    res
}
#[test]
fn test_vec_to_string() {
    assert_eq!(vec_to_string(&vec![1,2]), vec!["1","2"]);
}

#[snippet = "ctoi"]
fn ctoi(c: char) -> i64 {
    if 'A' <= c && c <= 'Z' {
        c as i64 - 'A' as i64
    } else if 'a' <= c && c <= 'z' {
        c as i64 - 'a' as i64 + 26
    } else {
        unreachable!("wrong character");
    }
}

 
#[snippet = "itoc"]
fn itoc(c: i64) -> char {
    if 0 <= c && c < 26 {
        (c as u8 + 'A' as u8) as char
    } else if c < 52 {
        (c as u8 - 26 + 'a' as u8) as char
    } else {
        unreachable!("wrong character");
    }
}

#[test]
fn test_ctoi_itoc() {
    let check = |c: char| {
        assert_eq!(itoc(ctoi(c)), c);
    };
    check('a');
    check('z');
    check('A');
    check('Z');
}