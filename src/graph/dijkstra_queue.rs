use std::collections::HashMap;
use std::collections::BinaryHeap;
#[snippet = "DijkstraHeap"]
struct DijkstraQueue<State: std::hash::Hash + std::cmp::Eq> {
    cur: usize,
    que: Vec<Vec<State>>,
    next: BinaryHeap<i64>,
}
#[snippet = "DijkstraHeap"]
impl <State: Default + Clone + std::hash::Hash + std::cmp::Eq> DijkstraQueue<State> {
    pub fn new(maxdist: usize) -> DijkstraQueue<State> {
        DijkstraQueue {
            cur: maxdist+1,
            que: vec![vec![]; maxdist+2],
            next: BinaryHeap::new(),
        }
    }
    fn pop_retry(&mut self) -> Option<(usize, State)> {
        self.forward_cur();
        let q = &mut self.que[self.cur];
        if q.is_empty() {
            None
        } else {
            let e = q.pop().unwrap();
            Some((self.cur, e))
        }
    }
    pub fn pop(&mut self) -> Option<(usize, State)> {
        let q = &mut self.que[self.cur];
        if q.is_empty() {
            self.pop_retry()
        } else {
            let e = q.pop().unwrap();
            Some((self.cur, e))
        }
    }
    pub fn push(&mut self, cost: usize, st: State) {
        if self.que[cost].is_empty() {
            self.next.push(-1 * cost as i64);
        }
        self.que[cost].push(st);
    }
    fn is_empty_retry(&mut self) -> bool {
        self.forward_cur();
        let q = &self.que[self.cur];
        q.is_empty()
    }
    pub fn is_empty(&mut self) -> bool {
        let q = &self.que[self.cur];
        if q.is_empty() {
            self.is_empty_retry()
        } else {
            false
        }
    }
    fn forward_cur(&mut self) {
        if let Some(nx) = self.next.pop() {
            self.cur = -nx as usize;
        } 
    }
}
#[test]
fn test_dijkstra_heap_struct() {
    let mut q: DijkstraQueue<char> = DijkstraQueue::new(10000);
    assert!(q.is_empty());
    assert_eq!(q.pop(), None);
    q.push(0,'x');
    q.push(10000,'a');
    assert_eq!(q.pop(), Some((0,'x')));
    assert!(!q.is_empty());
    assert_eq!(q.pop(), Some((10000,'a')));
    assert!(q.is_empty());
    assert_eq!(q.pop(), None);
    assert_eq!(q.pop(), None);
}
const SZ: usize = 1000000;
#[bench]
fn bench_dijkstra_heap_push(b: &mut test::Bencher) {
    let mut s = DijkstraQueue::new(SZ);
    let mut data = vec![];
    for i in 0..SZ {
        data.push(i);
    }
    b.iter(||
        for &x in &data {
            s.push(x,0);
        }
    );
}
#[bench]
fn bench_dijkstra_heap_pop(b: &mut test::Bencher) {
    let mut s = DijkstraQueue::new(SZ);
    for i in 0..SZ {
        s.push(i, 0);
    }
    b.iter(||
        for _ in 0..SZ {
            s.pop();
        }
    );
}
#[bench]
fn bench_dijkstra_heap_is_empty(b: &mut test::Bencher) {
    let mut s = DijkstraQueue::new(SZ);
    s.push(1,0);
    b.iter(||
        for _ in 0..SZ {
            s.is_empty();
        }
    );
}