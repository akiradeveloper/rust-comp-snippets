use cargo_snippet::snippet;

#[snippet("skiplist")]
mod skiplist {
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
        max_height: Option<usize>,
        left_sentinel: Rc<RefCell<SkipNode<T>>>,
        right_sentinel: Rc<RefCell<SkipNode<T>>>,
        rand_gen: RandGen,
        traverse_stat: Cell<usize>,
        connect_stat: Cell<usize>,
    }
    impl Skiplist<usize> {
        pub fn print_graph(&self) {
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
                right_sentinel.borrow_mut().prev[level] = Some(left_sentinel.clone());
            }
            Skiplist {
                max_height: None,
                left_sentinel: left_sentinel,
                right_sentinel: right_sentinel,
                rand_gen: RandGen::new(0),
                traverse_stat: Cell::new(0),
                connect_stat: Cell::new(0),
            }
        }
        fn height(&self) -> usize {
            self.max_height.unwrap_or(33)
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
            self.max_height = Some(std::cmp::max(self.max_height.unwrap_or(0), new_height));
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
                if level == 0 {
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
                    level -= 1;
                    continue;
                } else {
                    cur = next;
                    self.traverse_stat.set(self.traverse_stat.get()+1);
                }
            }
            acc
        }
        fn traverse_rev(&self, x: &T) -> Vec<Rc<RefCell<SkipNode<T>>>> {
            if self.height() == 0 {
                return vec![]
            }

            let mut cur = self.right_sentinel.clone();
            let mut acc = vec![self.right_sentinel.clone(); self.height()];
            let mut level = self.height() - 1;
            loop {
                if level == 0 {
                    loop {
                        acc[level] = cur.clone();
                        let next = cur.borrow().prev[level].clone().unwrap();
                        if next.borrow().value.is_none() || next.borrow().value.as_ref().unwrap() <= x {
                            break;
                        } else {
                            cur = next.clone();
                        }
                    }
                    break;
                }

                let next = cur.borrow().prev[level].clone().unwrap();
                if next.borrow().value.is_none() || next.borrow().value.as_ref().unwrap() <= x {
                    acc[level] = cur.clone();
                    level -= 1;
                    continue;
                } else {
                    cur = next;
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
            let f = self.traverse(x)[0].clone();
            Range {
                forward: true,
                f: f,
                b: self.right_sentinel.clone(),
            }
        }
        #[doc = "iterator in range [,x]"]
        pub fn le_iter(&self, x: &T) -> Range<T> {
            let b = self.traverse_rev(x)[0].clone();
            Range {
                forward: false,
                f: self.left_sentinel.clone(),
                b: b,
            }
        }
        #[doc = "iterator in range [..]"]
        pub fn iter(&self) -> Range<T> {
            Range {
                forward: true,
                f: self.left_sentinel.clone(),
                b: self.right_sentinel.clone(),
            }
        }
        pub fn is_empty(&self) -> bool {
            let mut it = self.iter();
            let mut l = 0;
            for _ in it {
                l += 1;
            }
            l == 0
        }
        #[doc = "O(n)"]
        pub fn pop(&mut self) -> Option<T> {
            if self.is_empty() {
                None
            } else {
                let mut it = self.iter();
                let x = it.next().unwrap();
                self.remove(&x);
                Some(x)
            }
        }
        #[doc = "O(n)"]
        pub fn pop_back(&mut self) -> Option<T> {
            if self.is_empty() {
                None
            } else {
                let mut it = self.iter().rev();
                let x = it.next().unwrap();
                self.remove(&x);
                Some(x)
            }
        }
    }
    pub struct Range<T> {
        forward: bool,
        f: Rc<RefCell<SkipNode<T>>>,
        b: Rc<RefCell<SkipNode<T>>>,
    }
    impl <T: Clone> Iterator for Range<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            let next0 = if self.forward {
                self.f.borrow().next[0].clone()
            } else {
                self.b.borrow().prev[0].clone()
            };
            if next0.is_none() {
                return None
            }
            let next = next0.unwrap();
            if self.forward {
                self.f = next;
                self.f.borrow().value.clone()
            } else {
                self.b = next;
                self.b.borrow().value.clone()
            }
        }
    }
    impl <T: Clone> DoubleEndedIterator for Range<T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            let next0 = if self.forward {
                self.b.borrow().prev[0].clone()
            } else {
                self.f.borrow().next[0].clone()
            };
            if next0.is_none() {
                return None
            }
            let next = next0.unwrap();
            if self.forward {
                self.b = next;
                self.b.borrow().value.clone()
            } else {
                self.f = next;
                self.f.borrow().value.clone()
            }
        }
    }
    impl <T> fmt::Debug for Skiplist<T> where T: fmt::Debug + Clone + std::cmp::Ord {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let v: Vec<T> = self.iter().collect();
            writeln!(f, "{:?}", v);
            Ok(())
        }
    }
    struct SkipNode<T> {
        value: Option<T>,
        prev: Vec<Option<Rc<RefCell<SkipNode<T>>>>>,
        next: Vec<Option<Rc<RefCell<SkipNode<T>>>>>,
    }
    impl <T> fmt::Debug for SkipNode<T> where T: fmt::Debug + std::cmp::Ord {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "{:?}, {:?}", self.value, self.height());
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
 

    use std::collections::HashMap;
    pub struct Multiset<T> {
        sl: Skiplist<T>,
        counting: HashMap<T, usize>,
    }
    impl <T> Multiset<T> where T: Ord + fmt::Debug + Clone + std::hash::Hash {
        pub fn new() -> Multiset<T> {
            Multiset {
                sl: Skiplist::new(),
                counting: HashMap::new(),
            }
        }
        pub fn insert(&mut self, x: T) {
            self.sl.insert(x.clone());
            *self.counting.entry(x).or_insert(0) += 1;
        }
        pub fn counting(&self, x: &T) -> usize {
            self.counting.get(x).cloned().unwrap_or(0)
        }
        pub fn remove(&mut self, x: &T) -> bool {
            let cnt = self.counting(x);
            if cnt == 0 {
                return false
            }

            if cnt >= 2 {
                *self.counting.get_mut(x).unwrap() -= 1;
            }
            else if cnt == 1 {
                self.counting.remove(x);
                self.sl.remove(x);
            }
            return true
        }
        pub fn unwrap(&self) -> &Skiplist<T> {
            &self.sl
        }
    }
}

use skiplist::*;
use std::collections::BTreeSet;

#[test]
fn test_skiplist_insert() {
    let mut s = Skiplist::new();
    assert_eq!(s.find(&10), false);
    s.insert(10);
    assert_eq!(s.find(&8), false);
    assert_eq!(s.find(&10), true);
}
#[test]
fn test_skiplist_debug0() {
    let mut s = Skiplist::new();
    let mut data = vec![920,265,659];
    for x in data {
        s.insert(x);
        assert!(s.find(&x));
    }
    s.insert(660);
    dbg!(&s);
    assert!(s.find(&660));
}
#[test]
fn test_skiplist_debug1() {
    let mut s = Skiplist::new();
    s.insert(0);
    assert!(s.find(&0));
    s.insert(5);
    assert!(s.find(&5));
}
#[test]
fn test_skiplist_debug2() {
    let mut s = Skiplist::new();
    s.insert(0);
    s.insert(5);
    s.insert(9);
    assert_eq!(s.find(&5),true);
    s.remove(&4);
    assert_eq!(s.find(&5),true);
    s.remove(&5);
    assert_eq!(s.find(&5),false);
    s.remove(&9);
    assert_eq!(s.find(&9),false);
    assert_eq!(s.find(&0),true);
}
#[test]
fn test_skiplist_pop() {
    let mut s = Skiplist::new();
    for _ in 0..1000 {
        assert!(s.is_empty());
        s.insert(2);
        s.insert(1);
        s.insert(3);
        assert!(!s.is_empty());
        assert_eq!(s.pop(), Some(1));
        assert!(!s.is_empty());
        assert_eq!(s.pop_back(), Some(3));
        assert!(!s.is_empty());
        assert_eq!(s.pop_back(), Some(2));
    }
}
#[test]
fn test_skiplist_pair() {
    let mut s = Skiplist::new();
    s.insert((10,true));
    s.insert((10,false));
    s.insert((11,false));
    assert_eq!(s.pop(), Some((10,false)));
    assert_eq!(s.pop(), Some((10,true)));
    assert_eq!(s.pop(), Some((11,false)));
}
#[test]
fn test_skiplist_compare_ref_insert_and_find() {
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
        ts.insert(x);
        sl.insert(x);
        assert_eq!(sl.find(&x), ts.contains(&x));
    }
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

#[test]
fn test_multiset() {
    let mut s = Multiset::new();
    assert_eq!(s.counting(&1),0);
    s.insert(1);
    assert_eq!(s.counting(&1),1);
    s.insert(1);
    assert_eq!(s.counting(&1),2);
    assert!(s.remove(&1));
    assert_eq!(s.unwrap().ge_iter(&1).next().unwrap(),1);
    assert_eq!(s.counting(&1),1);
    assert!(s.remove(&1));
    assert_eq!(s.counting(&1),0);
    assert_eq!(s.unwrap().ge_iter(&1).next(),None);
}