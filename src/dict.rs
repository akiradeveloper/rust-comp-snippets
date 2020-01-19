use std::collections::HashMap;
#[snippet = "Dict"]
struct Dict<K, V> {
    inner: HashMap<K, V>,
    default: V,
}
#[snippet = "Dict"]
impl <K: std::cmp::Eq + std::hash::Hash + Clone, V: Clone> Dict<K, V> {
    pub fn new(default: V) -> Dict<K, V> {
        Dict {
            inner: HashMap::new(),
            default: default,
        }
    }
    pub fn get(&self, k: &K) -> &V {
        self.inner.get(&k).unwrap()
    }
    pub fn get_mut(&mut self, k: K) -> &mut V {
        self.inner.entry(k).or_insert(self.default.clone())
    }
    pub fn keys(&self) -> Vec<K> {
        let mut v = vec![];
        for k in self.inner.keys() {
            v.push(k.clone())
        }
        v
    }
}
#[test]
fn test_dict() {
    let mut di = Dict::new(vec![]);
    di.get_mut(3).push(4);
    di.get_mut(3).push(6);
    di.get_mut(4).push(5);

    dbg!(di.get(&3));
    dbg!(di.keys());
}