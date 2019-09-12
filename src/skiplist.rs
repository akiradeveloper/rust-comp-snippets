#[snippet = "skiplist"]
mod skiplist {
    use crate::xorshift;
    
    use std;
    use std::collections::{BTreeMap, BTreeSet};
    use std::rc::Rc;
    use std::cell::{Cell, RefCell};
    // use std::ops::RangeBounds;
    use std::fmt;

    struct RandGen {
        x: u64,
    }
    impl RandGen {
        fn new(seed: u64) -> RandGen {
            RandGen {
                x: seed,
            }
        }
        fn next(&mut self) -> u64 {
            const a: u64 = 1103515245;
            const b: u64 = 12345;
            const m: u64 = 1<<32;
            self.x = (a*self.x+b)%m;
            self.x
        }
    }

    pub struct Skiplist<T> {
        max_height: usize,
        left_sentinel: Rc<RefCell<SkipNode<T>>>,
        right_sentinel: Rc<RefCell<SkipNode<T>>>,
        rand_gen: RandGen,
        traverse_stat: Cell<usize>,
        connect_stat: Cell<usize>,
    }
    impl Skiplist<usize> {
        fn print_graph(&self) {
            for level in (0..self.height()).rev() {
                let mut line=vec![];
                let mut cur = self.left_sentinel.clone();
                loop {
                    let next0 = cur.borrow().next[level].clone();
                    let next = next0.unwrap().clone();
                    if next.borrow().value.is_none() {
                        break;
                    } else {
                        cur = next.clone();
                        let v = cur.borrow().value.clone().unwrap();
                        line.push(v);
                    }
                }
                let mut ss = vec![];
                for x in line {
                    while ss.len() < x {
                        ss.push("--".to_string());
                    }
                    ss.push(format!("{:>02}", x));
                }
                println!("{}",ss.connect(","));
            }
            println!("");
        }
    }
    impl <T> Skiplist<T> where T: std::cmp::Ord + fmt::Debug + Clone {
        pub fn new() -> Skiplist<T> {
            let left_sentinel = Rc::new(RefCell::new(SkipNode::sentinel()));
            let right_sentinel = Rc::new(RefCell::new(SkipNode::sentinel()));
            let sentinel_height = left_sentinel.borrow().height();
            for level in 0..sentinel_height {
                left_sentinel.borrow_mut().next[level] = Some(right_sentinel.clone());
                right_sentinel.borrow_mut().next[level] = Some(left_sentinel.clone());
            }
            Skiplist {
                max_height: 0,
                left_sentinel: left_sentinel,
                right_sentinel: right_sentinel,
                rand_gen: RandGen::new(0),
                traverse_stat: Cell::new(0),
                connect_stat: Cell::new(0),
            }
        }
        fn height(&self) -> usize {
            self.max_height
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
        pub fn insert(&mut self, x: T) -> bool {
            let mut paths = self.traverse(&x);
            // println!("insert {:?}: {:?}", x, &paths);

            if !paths.is_empty() {
                let next0 = paths[0].borrow().next[0].clone();
                let next = next0.unwrap();
                let found = next.borrow().value.as_ref() == Some(&x);
                if found {
                    return false;
                }
            }

            let new_height = self.pick_height();
            self.max_height = std::cmp::max(self.max_height, new_height);
            while paths.len() < new_height {
                paths.push(self.left_sentinel.clone());
            }
            let new_node = Rc::new(RefCell::new(SkipNode::new(x, new_height)));
            for level in 0..new_height {
                let prev = &paths[level];
                self.connect_stat.set(self.connect_stat.get()+1);
                SkipNode::connect(prev, &new_node, level);
            }
            
            true
        }
        fn find_node(&self, x: &T) -> Option<Rc<RefCell<SkipNode<T>>>> {
            let paths = self.traverse(x);
            // println!("find {:?}: {:?}", x, &paths);

            if paths.is_empty() {
                return None
            }

            let next0 = paths[0].borrow().next[0].clone();
            let next = next0.unwrap();
            if next.borrow().value.as_ref() == Some(x) {
                Some(next)
            } else {
                None
            }
        }
        pub fn find(&self, x: &T) -> bool {
            self.find_node(x).is_some()
        }
        pub fn reset_stat(&self) {
            self.traverse_stat.set(0);
            self.connect_stat.set(0);
        }
        pub fn show_stat(&self) {
            println!("traverse: {}", self.traverse_stat.get());
            println!("connect: {}", self.connect_stat.get());
        }
        fn traverse(&self, x: &T) -> Vec<Rc<RefCell<SkipNode<T>>>> {
            if self.height() == 0 {
                return vec![]
            }

            let mut cur = self.left_sentinel.clone();
            let mut acc = vec![self.left_sentinel.clone(); self.height()];
            let mut level = self.height() - 1;
            loop {
                if level==0 {
                    loop {
                        acc[level] = cur.clone();

                        let next0 = cur.borrow().next[level].clone();
                        let next = next0.unwrap();
                        if next.borrow().value.is_none() || next.borrow().value.as_ref().unwrap() >= x {
                            break;
                        } else {
                            cur = next.clone();
                            self.traverse_stat.set(self.traverse_stat.get()+1);
                        }
                    }
                    break;
                }

                let next0 = cur.borrow().next[level].clone();
                let next = next0.unwrap();
                if next.borrow().value.is_none() || next.borrow().value.as_ref().unwrap() >= x {
                    acc[level] = cur.clone();
                    level-=1;
                    continue;
                } else {
                    cur = next;
                    self.traverse_stat.set(self.traverse_stat.get()+1);
                }
            }
            acc
        }
        pub fn remove(&mut self, x: &T) -> bool {
            let node = self.find_node(x);
            if node.is_none() {
                return false
            }
            let node = node.unwrap();
            node.borrow_mut().remove();
            true
        }
        #[doc = "iterator in range [x,]"]
        pub fn ge_iter(&self, x: &T) -> Range<T> {
            unimplemented!();
        }
        #[doc = "iterator in range [,x]"]
        pub fn le_iter(&self, x: &T) -> Range<T> {
            unimplemented!();
        }
        #[doc = "iterator in range [..]"]
        pub fn iter(&self) -> Range<T> {
            Range {
                forward: true,
                cur: self.left_sentinel.clone(),
                last: self.right_sentinel.clone(),
            }
        }
    }
    pub struct Range<T> {
        forward: bool,
        cur: Rc<RefCell<SkipNode<T>>>,
        last: Rc<RefCell<SkipNode<T>>>,
    }
    impl <T: Clone> Iterator for Range<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            let next0 = if self.forward {
                self.cur.borrow().next[0].clone()
            } else {
                self.cur.borrow().prev[0].clone()
            };
            if next0.is_none() {
                return None
            }
            let next = next0.unwrap();
            self.cur = next;
            self.cur.borrow().value.clone()
        }
    }
    impl <T: Clone> DoubleEndedIterator for Range<T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            let next0 = if self.forward {
                self.cur.borrow().prev[0].clone()
            } else {
                self.cur.borrow().next[0].clone()
            };
            if next0.is_none() {
                return None
            }
            let next = next0.unwrap();
            self.cur = next;
            self.cur.borrow().value.clone()
        }
    }
    struct SkipNode<T> {
        value: Option<T>,
        prev: Vec<Option<Rc<RefCell<SkipNode<T>>>>>,
        next: Vec<Option<Rc<RefCell<SkipNode<T>>>>>,
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
                prev: vec![None; 33],
                next: vec![None; 33],
            }
        }
        fn new(value: T, height: usize) -> SkipNode<T> {
            SkipNode {
                value: Some(value),
                prev: vec![None; height],
                next: vec![None; height],
            }
        }
        fn height(&self) -> usize {
            self.next.len()
        }
        fn remove(&mut self) {
            for level in 0..self.height() {
                let prev_node = self.prev[level].clone().unwrap();
                let next_node = self.next[level].clone().unwrap();
                next_node.borrow_mut().prev[level] = Some(prev_node.clone());
                prev_node.borrow_mut().next[level] = Some(next_node.clone());
            }
        }
        // x -> z => x -> y -> z
        // z = some or none
        fn connect(x: &Rc<RefCell<Self>>, y: &Rc<RefCell<Self>>, level: usize) {
            let x_next = x.borrow().next[level].clone().unwrap();
            x.borrow_mut().next[level] = Some(y.clone());
            y.borrow_mut().prev[level] = Some(x.clone());

            y.borrow_mut().next[level] = Some(x_next.clone());
            x_next.borrow_mut().prev[level] = Some(y.clone());
        }
    }

    #[test]
    fn test_pick_height() {
        let mut sl = Skiplist::<i64>::new();
        let mut cnt = vec![0;60];
        for _ in 0..1_000 {
            cnt[sl.pick_height()] += 1;
        }
        println!("{:?}",cnt);
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
        s.remove(&4);
        assert_eq!(s.find(&5),true);
        s.remove(&5);
        s.print_graph();
        assert_eq!(s.find(&5),false);
        s.remove(&9);
        s.print_graph();
        assert_eq!(s.find(&9),false);
        assert_eq!(s.find(&0),true);
    }
    #[test]
    fn test_compare_reference_insert_and_find() {
        use rand::{Rng, SeedableRng, StdRng};
        let mut rng = StdRng::from_seed(&[3, 2, 1]); 
        let mut ts = BTreeSet::new();
        let mut sl = Skiplist::new();

        let size = 10000;
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
        let mut data3 = vec![];
        for _ in 0..size {
            let x = rng.next_u64()%size;
            data3.push(x as usize);
        }
        println!("insert and find phase");
        for x in data1 {
            // dbg!(x);
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
        println!("remove phase");
        for x in data3 {
            assert_eq!(sl.remove(&x), ts.remove(&x));
            assert_eq!(sl.find(&x), ts.contains(&x));
        }
    }
    #[test]
    fn test_skiplist_stat() {
        use rand::{Rng, SeedableRng, StdRng};
        let size = 1000;
        let mut rng = StdRng::from_seed(&[3, 2, 1]);
        let mut s = Skiplist::new();
        println!("insert");
        for _ in 0..size {
            s.insert(rng.next_u64());
        }
        s.show_stat();
        s.reset_stat();
        println!("connect");
        for _ in 0..size {
            s.find(&rng.next_u64());
        }
        s.show_stat();
    }
    #[bench]
    fn bench_skiplist_insert_random(b: &mut test::Bencher) {
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
    fn bench_skiplist_find_random(b: &mut test::Bencher) {
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
    #[bench]
    fn bench_skiplist_insert_forward(b: &mut test::Bencher) {
        let mut s = Skiplist::new();
        let size = 10000;
        let mut data = vec![];
        for i in 0..size {
            data.push(i);
        }
        b.iter(||
            for &x in &data {
                s.insert(x);
            }
        );
    }
    #[bench]
    fn bench_skiplist_insert_reverse(b: &mut test::Bencher) {
        let mut s = Skiplist::new();
        let size = 10000;
        let mut data = vec![];
        for i in 0..size {
            data.push(i);
        }
        data.reverse();
        b.iter(||
            for &x in &data {
                s.insert(x);
            }
        );
    }
    #[bench]
    fn bench_skiplist_connect_simple(b: &mut test::Bencher) {
        let left=Rc::new(RefCell::new(SkipNode::new(0,2)));
        let right=Rc::new(RefCell::new(SkipNode::new(10000000,2)));
        for l in 0..2 {
            left.borrow_mut().next[l]=Some(right.clone());
            right.borrow_mut().prev[l]=Some(left.clone());
        }
        let mut data = vec![];
        for i in 0..10000 {
            let n=Rc::new(RefCell::new(SkipNode::new(i+1,2)));
            data.push(n)
        }
        b.iter(||
            for n in &data {
                for l in 0..2 {
                    SkipNode::connect(&left, n, l);
                }
            }
        )
    }
    #[bench]
    fn bench_skiplist_connect_random(b: &mut test::Bencher) {
        use rand::{Rng, SeedableRng, StdRng};
        let mut rng = StdRng::from_seed(&[3, 2, 1]);

        let left=Rc::new(RefCell::new(SkipNode::new(0,2)));
        let right=Rc::new(RefCell::new(SkipNode::new(10000000,2)));
        for l in 0..2 {
            left.borrow_mut().next[l]=Some(right.clone());
            right.borrow_mut().prev[l]=Some(left.clone());
        }
        let mut prev_cands = vec![left.clone()];
        let mut data = vec![];
        for i in 0..10000 {
            let n=Rc::new(RefCell::new(SkipNode::new(i+1,2)));
            let i=rng.next_u64() as usize % prev_cands.len();
            let prev = prev_cands[i].clone();
            data.push((prev,n.clone()));
            prev_cands.push(n.clone());
        }
        b.iter(||
            for (prev,n) in &data {
                for l in 0..2 {
                    SkipNode::connect(&prev, &n, l);
                }
            }
        )
    }
    #[bench]
    fn bench_skiplist_alloc_new_node(b: &mut test::Bencher) {
        b.iter(||
            for _ in 0..10000 {
                Rc::new(RefCell::new(SkipNode::new(0,2)));
            }
        )
    }
    #[bench]
    fn bench_skiplist_pick_height(b: &mut test::Bencher) {
        let mut sl=Skiplist::<i64>::new();
        b.iter(||
            for _ in 0..10000 {
                sl.pick_height();
            }
        )
    }
    #[bench]
    fn bench_btree_insert_random(b: &mut test::Bencher) {
        use rand::{Rng, SeedableRng, StdRng};
        let size = 10000;
        let mut s = BTreeSet::new();
        let mut rng = StdRng::from_seed(&[3, 2, 1]);
        b.iter(||
            for _ in 0..size {
                s.insert(rng.next_u64());
            }
        );
    }
    #[bench]
    fn bench_btree_find_random(b: &mut test::Bencher) {
        use rand::{Rng, SeedableRng, StdRng};
        let size = 10000;
        let mut s = BTreeSet::new();
        let mut rng = StdRng::from_seed(&[3, 2, 1]);
        for _ in 0..size {
            s.insert(rng.next_u64());
        }
        b.iter(||
            for _ in 0..size {
                s.contains(&rng.next_u64());
            }
        );
    }
}