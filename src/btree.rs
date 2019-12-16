use std::collections::BTreeSet;

#[snippet = "btree_pop_min"]
pub fn btree_pop_min<T: Ord + Clone>(set: &mut BTreeSet<T>) -> Option<T> {
    let ret : Option<T> = set.iter().next().map(|k| k.clone());
    if let Some(x) = &ret {
        set.remove(x);
    }
    ret
}
#[snippet = "bree_pop_max"]
pub fn btree_pop_max<T: Ord + Clone>(set: &mut BTreeSet<T>) -> Option<T> {
    let ret : Option<T> = set.iter().rev().next().map(|k| k.clone());
    if let Some(x) = &ret {
        set.remove(x);
    }
    ret
}

#[test]
fn test_btree_pop() {
    let mut s = BTreeSet::new();
    for _ in 0..1000 {
        assert!(s.is_empty());
        s.insert(2);
        s.insert(1);
        s.insert(3);
        assert!(!s.is_empty());
        assert_eq!(btree_pop_min(&mut s), Some(1));
        assert!(!s.is_empty());
        assert_eq!(btree_pop_max(&mut s), Some(3));
        assert!(!s.is_empty());
        assert_eq!(btree_pop_max(&mut s), Some(2));
    }
}