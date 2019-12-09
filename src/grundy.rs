#[snippet = "grundy_def"]
pub fn grundy_def(children: Vec[i64]) -> i64 {
    children.sort();
    let mut n = children.len();
    for i in 0..n {
        if i != children[i] {
            return i as i64
        }
    }
    n as i64
}
#[test]
fn test_grundy() {
    assert_eq!(grundy_def(vec![0,1,3]), 2);
    assert_eq!(grundy_def(vec![1,3,5]), 0);
    assert_eq!(grundy_def(vec![0,1,2]), 4);
}