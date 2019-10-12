#[snippet = "manacher"]
#[doc = "manacher[i] is the palindrome radius at i"]
pub fn manacher(s: &[usize]) -> Vec<usize> {
    let mut r = vec![0; s.len()];
    let mut i = 0;
    let mut j = 0;
    while i < s.len() {
        while i >= j && i+j < s.len() && s[i-j] == s[i+j] {
            j += 1;
        }
        r[i] = j;
        let mut k = 1;
        while i >= k && i+k < s.len() && k+r[i-k] < j {
            r[i+k] = r[i-k]; 
            k += 1;
        }
        i += k;
        j -= k;
    }
    r
}
#[test]
fn test_manacher_even() {
    let s = vec![1,1];
    // 奇数長回文しか検出しない（長さ3でやっと半径2になる）
    assert_eq!(manacher(&s), [1,1]);
}
#[test]
fn test_manacher() {
    let s = vec![0,1,0,0,0,1,0,1,0];
    assert_eq!(manacher(&s), [1,2,1,4,1,2,3,2,1]);
}