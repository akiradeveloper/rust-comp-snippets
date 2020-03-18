use cargo_snippet::snippet;

#[snippet("compare_str")]
#[doc = "-1: s<t, 0: s=t, 1: s>t"]
pub fn compare_str(s: &[char], t: &[char]) -> i8 {
    let n = std::cmp::min(s.len(), t.len());
    for i in 0..n {
        if s[i] < t[i] {
            return -1
        } else if s[i] > t[i] {
            return 1
        } else {} // eq
    }
    if s.len() < t.len() {
        return -1
    } else if s.len() > t.len() {
        return 1
    } else {
        return 0
    }
}
#[test]
fn test_compare_str() {
    assert_eq!(compare_str(&['a','b'], &['a','c']), -1);
    assert_eq!(compare_str(&['a','b'], &['a','b']), 0);
    assert_eq!(compare_str(&['b','b'], &['a','b']), 1);
}