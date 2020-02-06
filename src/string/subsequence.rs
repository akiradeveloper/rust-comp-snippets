#[doc = "naive implementation. O(nm)"]
#[snippet = "is_subsequence"]
pub fn is_subsequence(s: &[char], t: &[char]) -> bool {
    let n = s.len();
    let m = t.len();
    let mut j = 0;
    let mut ok = false;
    for i in 0..n {
        let c = s[i];
        while j < m {
            let found = t[j] == s[i];
            j += 1;
            if found {
                if i == n-1 {
                    ok = true;
                }
                break;
            }
        }
    }
    ok
}