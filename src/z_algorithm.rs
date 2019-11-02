#[doc = "z[i] is the common prefix length between s and s[i..]. O(N)"]
#[snippet = "z_algorithm"]
pub fn z_algorithm(s: &[u64]) -> Vec<usize> {
    let mut z = vec![0; s.len()];
    z[0] = s.len();
    let mut i = 1;
    let mut j = 0;
    while i < s.len() {
        while i+j < s.len() && s[j] == s[i+j] {
            j += 1;
        }
        z[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i+k < s.len() && k+z[k] < j {
            z[i+k] = z[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    z
}
#[test]
fn test_z_algorithm() {
    let s = vec![1,1,1,2,1,1,1,1,2];
    let z = z_algorithm(&s);
    assert_eq!(z, [9,2,1,0,3,4,2,1,0]);
}