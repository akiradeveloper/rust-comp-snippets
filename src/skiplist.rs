mod skiplist {
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::RangeBounds;
    struct Skiplist<T> {
        sentinel: Rc<RefCell<SkipNode<T>>>,
    }
    impl <T> Skiplist<T> where T: std::cmp::Ord {
        fn new() -> Skiplist<T> {
            Skiplist {
                sentinel: Rc::new(SkipNode::sentinel().into())
            }
        }
        fn insert(&mut self, x: T) -> bool {
            if self.find(&x) { return false }

            let paths = self.traverse(&x);
            let node0 = paths[0].borrow().next.get(&0).cloned();
             if node0.is_none() {
                return false
            }
            let node = node0.unwrap();
            let found = node.borrow().value == Some(x);
            if found {
                return false;
            }


            // compute the height
            // connect
            true
        }
        fn find(&self, x: &T) -> bool {
            let paths = self.traverse(x);
            let node0 = paths[0].borrow().next.get(&0).cloned();
            if node0.is_none() {
                return false
            }
            let node = node0.unwrap();
            let found = node.borrow().value.as_ref() == Some(x);
            found
        }
        fn range<R: RangeBounds<T>>(&self, range: R) -> Range<T> {
            unimplemented!()
        }
        fn height(&self) -> usize {
            self.sentinel.borrow().height
        }
        fn traverse(&self, x: &T) -> Vec<Rc<RefCell<SkipNode<T>>>> {
            let mut cur = self.sentinel.clone();
            let mut acc = vec![self.sentinel.clone(); self.height()];
            let mut level = self.height()-1;
            loop {
                if level==0 {
                    acc[level] = cur.clone();
                    break;
                }

                let next0 = cur.borrow().next.get(&level).cloned();
                if next0.is_none() {
                    acc[level] = cur.clone();
                    level-=1;
                    continue; 
                } else {
                    let next = next0.unwrap();
                    if next.borrow().value.as_ref().unwrap() >= x {
                        acc[level] = cur.clone();
                        level-=1;
                        continue;
                    } else {
                        cur = next;
                    }
                }
            }
            acc
        }
    }
    struct Range<T> {
        cur: Rc<RefCell<SkipNode<T>>>,
    }
    struct SkipNode<T> {
        value: Option<T>,
        height: usize,
        prev: HashMap<usize, Rc<RefCell<SkipNode<T>>>>,
        next: HashMap<usize, Rc<RefCell<SkipNode<T>>>>,
    }
    impl <T> SkipNode<T> where T: std::cmp::Ord {
        fn sentinel() -> SkipNode<T> {
            SkipNode {
                value: None,
                height: 0,
                prev: HashMap::new(),
                next: HashMap::new(),
            }
        }
        fn new(value: T, height: usize) -> SkipNode<T> {
            SkipNode {
                value: Some(value),
                height: height,
                prev: HashMap::new(),
                next: HashMap::new(),
            }
        }
        fn remove(&mut self) {
            for level in 0..self.height {
                let prev_node = self.prev.get(&level).unwrap();
                prev_node.borrow_mut().next.remove(&level);
                if self.next.contains_key(&level) {
                    let next_node = self.next.get(&level).unwrap();
                    *next_node.borrow_mut().prev.get_mut(&level).unwrap() = prev_node.clone();
                    prev_node.borrow_mut().next.insert(level, next_node.clone());
                }
            }
        }
        fn connect(x: Rc<RefCell<Self>>, y: Rc<RefCell<Self>>, level: usize) {
            x.borrow_mut().next.insert(level, y.clone());
            y.borrow_mut().prev.insert(level, x.clone());
        }
    }

    #[test]
    fn test_skip_node_connection() {
        let x=Rc::new(RefCell::new(SkipNode::<i64>::sentinel()));
        let y=Rc::new(RefCell::new(SkipNode::<i64>::new(0, 2)));
        SkipNode::connect(x.clone(), y.clone(), 0);
        SkipNode::connect(x.clone(), y.clone(), 1);
    }
    #[test]
    fn test_insert() {
        let mut s = Skiplist::new();
        assert_eq!(s.find(&10), false);
        s.insert(10);
        assert_eq!(s.find(&8), false);
        assert_eq!(s.find(&10), true);
    }
    #[bench]
    fn bench_skiplist(b: &mut test::Bencher) {
        use rand::{Rng, SeedableRng, StdRng};
        let size = 1_000_000;
        let mut s = Skiplist::new();
        let mut rng = StdRng::from_seed(&[3, 2, 1]);

        b.iter(||
            for _ in 0..size {
                s.insert(rng.next_u64());
            }
        );
        b.iter(||
            for _ in 0..size {
                s.find(&rng.next_u64());
            }
        );
    }
}