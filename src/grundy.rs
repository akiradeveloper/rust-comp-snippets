#[snippet = "grundy_def"]
pub fn grundy_def(xs: Vec<i64>) -> i64 {
    let mut xs = xs;
    xs.sort();
    let mut n = xs.len();
    for i in 0..n {
        if i as i64 != xs[i] {
            return i as i64
        }
    }
    n as i64
}
#[test]
fn test_grundy_def() {
    assert_eq!(grundy_def(vec![0,1,3]), 2);
    assert_eq!(grundy_def(vec![1,3,5]), 0);
    assert_eq!(grundy_def(vec![0,1,2]), 3);
}