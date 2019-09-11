mod skiplist {
    use std::collections::{BTreeMap, BTreeSet};
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::RangeBounds;
    use std::fmt;

    struct RandGen {
        x: i64,
    }
    impl RandGen {
        const a: i64 = 1103515245;
        const b: i64 = 12345;
        const m: i64 = 1<<32;
        fn new(seed: i64) -> RandGen {
            RandGen {
                x: seed,
            }
        }
        fn next(&mut self) -> i64 {
            self.x = (Self::a*self.x+Self::b)%Self::m;
            self.x
        }
    }

    struct Skiplist<T> {
        sentinel: Rc<RefCell<SkipNode<T>>>,
        rand_gen: RandGen,
    }
    impl Skiplist<usize> {
        fn print_graph(&self) {
            for level in (0..self.height()).rev() {
                let mut line=vec![];
                let mut cur = self.sentinel.clone();
                loop {
                    let next0 = cur.borrow().next.get(&level).cloned();
                    if next0.is_none() {
                        break;
                    } else {
                        cur = next0.unwrap().clone();
                        let v = cur.borrow().value.clone().unwrap();
                        line.push(v);
                    }
                }
                let mut ss = vec![];
                for x in line {
                    while ss.len()+1 < x {
                        ss.push("--".to_string());
                    }
                    ss.push(format!("{:>02}", x));
                }
                println!("{}",ss.connect(","));
            }
        }
    }
    impl <T> Skiplist<T> where T: std::cmp::Ord + fmt::Debug + Clone {
        fn new() -> Skiplist<T> {
            Skiplist {
                sentinel: Rc::new(SkipNode::sentinel().into()),
                rand_gen: RandGen::new(0),
            }
        }
        fn height(&self) -> usize {
            self.sentinel.borrow().height()
        }
        fn pick_height(&mut self) -> usize {
            let z = self.rand_gen.next();
            let mut k = 0;
            let mut m = 1;
            while z&m!=0 {
                k+=1;
                m<<=1;
            }
            k+1
        }
        fn insert(&mut self, x: T) -> bool {
            let paths = self.traverse(&x);
            println!("insert {:?}: {:?}", x, &paths);

            if !paths.is_empty() {
                let node0 = paths[0].borrow().next.get(&0).cloned();
                if node0.is_some() {
                    let node = node0.unwrap();
                    let found = node.borrow().value.as_ref() == Some(&x);
                    if found {
                        return false;
                    }
                }
            }

            let new_height = self.pick_height();
            println!("new height: {}", new_height);
            let new_node = Rc::new(RefCell::new(SkipNode::new(x)));
            for level in (0..new_height).rev() {
                if !self.sentinel.borrow().next.contains_key(&level) {
                    SkipNode::connect(self.sentinel.clone(), new_node.clone(), level);
                } else {
                    let prev = paths[level].clone();
                    SkipNode::connect(prev, new_node.clone(), level);
                }
            }
            
            true
        }
        fn find(&self, x: &T) -> bool {
            let paths = self.traverse(x);
            println!("find {:?}: {:?}", x, &paths);

            if paths.is_empty() {
                return false;
            }

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
        fn traverse(&self, x: &T) -> Vec<Rc<RefCell<SkipNode<T>>>> {
            if self.height()==0 {
                return vec![]
            }

            let mut cur = self.sentinel.clone();
            let mut acc = vec![self.sentinel.clone(); self.height()];
            let mut level = self.height()-1;
            loop {
                if level==0 {
                    loop {
                        acc[level] = cur.clone();

                        let next0 = cur.borrow().next.get(&level).cloned();
                        if next0.is_none() {
                            break;
                        } else {
                            let next = next0.unwrap();
                            if next.borrow().value.as_ref().unwrap() >= x {
                                break;
                            } else {
                                cur = next.clone();
                            }
                        }
                    }
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
        prev: BTreeMap<usize, Rc<RefCell<SkipNode<T>>>>,
        next: BTreeMap<usize, Rc<RefCell<SkipNode<T>>>>,
    }
    impl <T> fmt::Debug for SkipNode<T> where T: fmt::Debug + std::cmp::Ord {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "value: {:?}, height: {}", self.value, self.height());
            Ok(())
        }
    }
    impl <T> SkipNode<T> where T: std::cmp::Ord + fmt::Debug {
        fn sentinel() -> SkipNode<T> {
            SkipNode {
                value: None,
                prev: BTreeMap::new(),
                next: BTreeMap::new(),
            }
        }
        fn new(value: T) -> SkipNode<T> {
            SkipNode {
                value: Some(value),
                prev: BTreeMap::new(),
                next: BTreeMap::new(),
            }
        }
        fn height(&self) -> usize {
            let next_height = self.next.keys().rev().next().cloned().map(|x| x+1).unwrap_or(0);
            let prev_height = self.prev.keys().rev().next().cloned().map(|x| x+1).unwrap_or(0);
            std::cmp::max(next_height, prev_height)
        }
        fn remove(&mut self) {
            for level in 0..self.height() {
                let prev_node = self.prev.get(&level).unwrap();
                prev_node.borrow_mut().next.remove(&level);
                if self.next.contains_key(&level) {
                    let next_node = self.next.get(&level).unwrap();
                    *next_node.borrow_mut().prev.get_mut(&level).unwrap() = prev_node.clone();
                    prev_node.borrow_mut().next.insert(level, next_node.clone());
                }
            }
        }
        // x -> z => x -> y -> z
        // z = some or none
        fn connect(x: Rc<RefCell<Self>>, y: Rc<RefCell<Self>>, level: usize) {
            let x_next = x.borrow().next.get(&level).cloned();
            x.borrow_mut().next.insert(level, y.clone());
            y.borrow_mut().prev.insert(level, x.clone());
            if x_next.is_some() {
                let x_next = x_next.unwrap();
                y.borrow_mut().next.insert(level, x_next.clone());
                x_next.borrow_mut().prev.insert(level, y.clone());
            }
        }
        fn is_connected_prev(&self, level: usize, x: Option<T>) -> bool {
            let prev0 = self.prev.get(&level);
            if prev0.is_none() {
                return x == None;
            } else {
                let prev = prev0.unwrap().clone();
                let prev_v = &prev.borrow().value;
                return &x == prev_v;
            }
        }
        fn is_connected_next(&self, level: usize, x: Option<T>) -> bool {
            let next0 = self.next.get(&level);
            if next0.is_none() {
                return x == None;
            } else {
                let next = next0.unwrap().clone();
                let next_v = &next.borrow().value;
                return &x == next_v;
            }
        }
    }

    #[test]
    fn test_skip_node_connection() {
        let x=Rc::new(RefCell::new(SkipNode::<i64>::sentinel()));
        let y=Rc::new(RefCell::new(SkipNode::<i64>::new(100)));
        SkipNode::connect(x.clone(), y.clone(), 0);
        SkipNode::connect(x.clone(), y.clone(), 1);
        assert_eq!(x.borrow().height(), 2);
        assert_eq!(y.borrow().height(), 2);
        assert!(x.borrow().is_connected_next(0, Some(100)));
        assert!(y.borrow().is_connected_prev(0, None));

        let z=Rc::new(RefCell::new(SkipNode::<i64>::new(10)));
        SkipNode::connect(x.clone(), z.clone(), 0);
        assert!(x.borrow().is_connected_next(0, Some(10)));
        assert!(y.borrow().is_connected_prev(0, Some(10)));
        assert!(z.borrow().is_connected_prev(0, None));
        assert!(z.borrow().is_connected_next(0, Some(100)));
    }
    #[test]
    fn test_rand() {
        let mut r = RandGen::new(0);
        for _ in 0..100 {
            println!("{}",r.next())
        }
    }
    #[test]
    fn test_pick_height() {
        let mut sl = Skiplist::<i64>::new();
        for _ in 0..100 {
            println!("{}",sl.pick_height())
        }
    }
    #[test]
    fn test_insert() {
        let mut s = Skiplist::new();
        assert_eq!(s.find(&10), false);
        s.insert(10);
        assert_eq!(s.find(&8), false);
        assert_eq!(s.find(&10), true);
    }
    #[test]
    fn test_debug0() {
        let mut s = Skiplist::new();
        let mut data = vec![920,265,659];
        for x in data {
            s.insert(x);
            assert!(s.find(&x));
        }
        s.insert(660);
        assert!(s.find(&660));
    }
    #[test]
    fn test_debug1() {
        let mut s = Skiplist::new();
        s.insert(0);
        assert!(s.find(&0));
        s.insert(5);
        assert!(s.find(&5));
    }
    #[test]
    fn test_debug2() {
        let mut s = Skiplist::new();
        s.insert(0);
        s.insert(5);
        s.print_graph();
        s.insert(9);
        s.print_graph();
        assert_eq!(s.find(&5),true);
    }
    #[test]
    fn test_compare_reference_insert_and_find() {
        use rand::{Rng, SeedableRng, StdRng};
        let mut rng = StdRng::from_seed(&[3, 2, 1]); 
        let mut ts = BTreeSet::new();
        let mut sl = Skiplist::new();

        let size = 50;
        let mut data1 = vec![];
        for _ in 0..size {
            let x = rng.next_u64()%size;
            data1.push(x as usize);
        }
        let mut data2 = vec![];
        for _ in 0..size {
            let x = rng.next_u64()%size;
            data2.push(x as usize);
        }
        println!("insert and find phase");
        for x in data1 {
            dbg!(x);
            ts.insert(x);
            sl.insert(x);
            assert_eq!(sl.find(&x), ts.contains(&x));
            // sl.print_graph();
        }
        sl.print_graph();
        println!("find phase");
        for x in data2 {
            assert_eq!(sl.find(&x), ts.contains(&x));
        }
    }
    #[bench]
    fn bench_skiplist_insert(b: &mut test::Bencher) {
        use rand::{Rng, SeedableRng, StdRng};
        let size = 10000;
        let mut s = Skiplist::new();
        let mut rng = StdRng::from_seed(&[3, 2, 1]);
        b.iter(||
            for _ in 0..size {
                s.insert(rng.next_u64());
            }
        );
    }
    #[bench]
    fn bench_skiplist_find(b: &mut test::Bencher) {
        use rand::{Rng, SeedableRng, StdRng};
        let size = 10000;
        let mut s = Skiplist::new();
        let mut rng = StdRng::from_seed(&[3, 2, 1]);
        for _ in 0..size {
            s.insert(rng.next_u64());
        }
        b.iter(||
            for _ in 0..size {
                s.find(&rng.next_u64());
            }
        );
    }
}