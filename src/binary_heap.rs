struct BinaryHeap {
    v: Vec<i64>
}
impl BinaryHeap {
    fn new() -> BinaryHeap {
        Self {
            v: vec![]
        }
    }
    fn rebuild(&mut self) {
        for i in (0..self.v.len()/2).rev() {
            self.max_heapify(i);
        }
    }
    fn push(&mut self, x: i64) {
        self.v.push(x);
        self.rebuild()
    }
    fn max_heapify(&mut self, i: usize) {
        let left = 2*i + 1;
        let right = 2*i + 2;
        let mut new_p = i;
        if left < self.v.len() && self.v[i] < self.v[left] {
            new_p = left;
        }
        if right < self.v.len() && self.v[i] < self.v[right] {
            new_p = right;
        }
        if new_p != i {
            self.v.swap(i, new_p);
            self.max_heapify(new_p);
        }
    }
}

#[test]
fn test_binary_heap() {
    let mut q = BinaryHeap::new();
    for x in &[3,4,5,1,2,2,7,6] {
        q.push(*x);
    }
    dbg!(&q.v);
}