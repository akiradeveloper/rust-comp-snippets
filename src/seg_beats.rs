use std::cmp::{max, min};
struct SEGBeats {
    max_v: Vec<i64>,
    smax_v: Vec<i64>,
    max_c: Vec<usize>,
    min_v: Vec<i64>,
    smin_v: Vec<i64>,
    min_c: Vec<usize>,
    sum: Vec<i64>,
    len: Vec<usize>,
    ladd: Vec<i64>,
    lval: Vec<i64>,
    n0: usize,
}
impl SEGBeats {
    const inf: i64 = std::i64::MAX / 4;
    pub fn new(n: usize) -> SEGBeats {
        let mut n0 = 1;
        while n0 < n {
            n0 <<= 1;
        }
        let mut max_v = vec![0; 2 * n0];
        let mut smax_v = vec![0; 2 * n0];
        let mut max_c = vec![0; 2 * n0];
        let mut min_v = vec![0; 2 * n0];
        let mut smin_v = vec![0; 2 * n0];
        let mut min_c = vec![0; 2 * n0];
        let mut sum = vec![0; 2 * n0];
        let mut len = vec![0; 2 * n0];
        let mut ladd = vec![0; 2 * n0];
        let mut lval = vec![0; 2 * n0];

        for i in 0..2 * n0 {
            ladd[i] = 0;
            lval[i] = Self::inf;
        }
        len[0] = n0;
        for i in 0..n0 - 1 {
            let l = len[i] >> 1;
            len[2 * i + 1] = l;
            len[2 * i + 2] = l;
        }
        for i in 0..n {
            let j = n0 - 1 + i;
            max_v[j] = 0;
            min_v[j] = 0;
            sum[j] = 0;
            smax_v[j] = -Self::inf;
            smin_v[j] = Self::inf;
            max_c[j] = 1;
            min_c[j] = 1;
        }
        for i in n..n0 {
            let j = n0 - 1 + i;
            max_v[j] = -Self::inf;
            smax_v[j] = -Self::inf;
            min_v[j] = Self::inf;
            smin_v[j] = Self::inf;
            max_c[j] = 0;
            min_c[j] = 0;
        }
        let mut ret = SEGBeats {
            max_v: max_v,
            smax_v: smax_v,
            max_c: max_c,
            min_v: min_v,
            smin_v: smin_v,
            min_c: min_c,
            sum: sum,
            len: len,
            ladd: ladd,
            lval: lval,
            n0: n0,
        };
        ret.build();
        ret
    }
    fn build(&mut self) {
        for i in (0..self.n0 - 1).rev() {
            self.update(i);
        }
    }
    fn update_node_max(&mut self, k: usize, x: i64) {
        self.sum[k] += (x - self.max_v[k]) * self.max_c[k] as i64;

        if self.max_v[k] == self.min_v[k] {
            self.max_v[k] = x;
            self.min_v[k] = x;
        } else if self.max_v[k] == self.smin_v[k] {
            self.max_v[k] = x;
            self.smin_v[k] = x;
        } else {
            self.max_v[k] = x;
        }
        if self.lval[k] != Self::inf && x < self.lval[k] {
            self.lval[k] = x;
        }
    }
    fn update_node_min(&mut self, k: usize, x: i64) {
        self.sum[k] += (x - self.min_v[k]) * self.min_c[k] as i64;

        if self.max_v[k] == self.min_v[k] {
            self.max_v[k] = x;
            self.min_v[k] = x;
        } else if self.smax_v[k] == self.min_v[k] {
            self.min_v[k] = x;
            self.smax_v[k] = x;
        } else {
            self.min_v[k] = x;
        }
        if self.lval[k] != Self::inf && self.lval[k] < x {
            self.lval[k] = x;
        }
    }
    fn push(&mut self, k: usize) {
        if self.n0 - 1 <= k {
            return;
        }

        if self.lval[k] != Self::inf {
            self.update_all(2 * k + 1, self.lval[k]);
            self.update_all(2 * k + 2, self.lval[k]);
            self.lval[k] = Self::inf;
            return;
        }

        if self.ladd[k] != 0 {
            self.add_all(2 * k + 1, self.ladd[k]);
            self.add_all(2 * k + 2, self.ladd[k]);
            self.ladd[k] = 0;
        }

        if self.max_v[k] < self.max_v[2 * k + 1] {
            self.update_node_max(2 * k + 1, self.max_v[k]);
        }
        if self.min_v[2 * k + 1] < self.min_v[k] {
            self.update_node_min(2 * k + 1, self.min_v[k]);
        }

        if self.max_v[k] < self.max_v[2 * k + 2] {
            self.update_node_max(2 * k + 2, self.max_v[k]);
        }
        if self.min_v[2 * k + 2] < self.min_v[k] {
            self.update_node_min(2 * k + 2, self.min_v[k]);
        }
    }
    fn update(&mut self, k: usize) {
        self.sum[k] = self.sum[2 * k + 1] + self.sum[2 * k + 2];

        if self.max_v[2 * k + 1] < self.max_v[2 * k + 2] {
            self.max_v[k] = self.max_v[2 * k + 2];
            self.max_c[k] = self.max_c[2 * k + 2];
            self.smax_v[k] = std::cmp::max(self.max_v[2 * k + 1], self.smax_v[2 * k + 2]);
        } else if self.max_v[2 * k + 1] > self.max_v[2 * k + 2] {
            self.max_v[k] = self.max_v[2 * k + 1];
            self.max_c[k] = self.max_c[2 * k + 1];
            self.smax_v[k] = std::cmp::max(self.smax_v[2 * k + 1], self.max_v[2 * k + 2]);
        } else {
            self.max_v[k] = self.max_v[2 * k + 1];
            self.max_c[k] = self.max_c[2 * k + 1] + self.max_c[2 * k + 2];
            self.smax_v[k] = std::cmp::max(self.smax_v[2 * k + 1], self.smax_v[2 * k + 2]);
        }

        if self.min_v[2 * k + 1] < self.min_v[2 * k + 2] {
            self.min_v[k] = self.min_v[2 * k + 1];
            self.min_c[k] = self.min_c[2 * k + 1];
            self.smin_v[k] = min(self.smin_v[2 * k + 1], self.min_v[2 * k + 2]);
        } else if self.min_v[2 * k + 1] > self.min_v[2 * k + 2] {
            self.min_v[k] = self.min_v[2 * k + 2];
            self.min_c[k] = self.min_c[2 * k + 2];
            self.smin_v[k] = min(self.min_v[2 * k + 1], self.smin_v[2 * k + 2]);
        } else {
            self.min_v[k] = self.min_v[2 * k + 1];
            self.min_c[k] = self.min_c[2 * k + 1] + self.min_c[2 * k + 2];
            self.smin_v[k] = min(self.smin_v[2 * k + 1], self.smin_v[2 * k + 2]);
        }
    }
    fn _update_min(&mut self, x: i64, a: usize, b: usize, k: usize, l: usize, r: usize) {
        if b <= l || r <= a || self.max_v[k] <= x {
            return;
        }
        if a <= l && r <= b && self.smax_v[k] < x {
            self.update_node_max(k, x);
            return;
        }

        self.push(k);
        self._update_min(x, a, b, 2 * k + 1, l, (l + r) / 2);
        self._update_min(x, a, b, 2 * k + 2, (l + r) / 2, r);
        self.update(k);
    }
    fn _update_max(&mut self, x: i64, a: usize, b: usize, k: usize, l: usize, r: usize) {
        if b <= l || r <= a || x <= self.min_v[k] {
            return;
        }
        if a <= l && r <= b && x < self.smin_v[k] {
            self.update_node_min(k, x);
            return;
        }

        self.push(k);
        self._update_max(x, a, b, 2 * k + 1, l, (l + r) / 2);
        self._update_max(x, a, b, 2 * k + 2, (l + r) / 2, r);
        self.update(k);
    }
    fn add_all(&mut self, k: usize, x: i64) {
        self.max_v[k] += x;
        if self.smax_v[k] != -Self::inf {
            self.smax_v[k] += x;
        }
        self.min_v[k] += x;
        if self.smin_v[k] != Self::inf {
            self.smin_v[k] += x;
        }
        self.sum[k] += self.len[k] as i64 * x;
        if self.lval[k] != Self::inf {
            self.lval[k] += x;
        } else {
            self.ladd[k] += x;
        }
    }
    fn update_all(&mut self, k: usize, x: i64) {
        self.max_v[k] = x;
        self.smax_v[k] = -Self::inf;
        self.min_v[k] = x;
        self.smin_v[k] = Self::inf;
        self.max_c[k] = self.len[k];
        self.min_c[k] = self.len[k];
        self.sum[k] = x * self.len[k] as i64;
        self.lval[k] = x;
        self.ladd[k] = 0;
    }
    fn _add_val(&mut self, x: i64, a: usize, b: usize, k: usize, l: usize, r: usize) {
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.add_all(k, x);
            return;
        }

        self.push(k);
        self._add_val(x, a, b, 2 * k + 1, l, (l + r) / 2);
        self._add_val(x, a, b, 2 * k + 2, (l + r) / 2, r);
        self.update(k);
    }
    fn _update_val(&mut self, x: i64, a: usize, b: usize, k: usize, l: usize, r: usize) {
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.update_all(k, x);
            return;
        }

        self.push(k);
        self._update_val(x, a, b, 2 * k + 1, l, (l + r) / 2);
        self._update_val(x, a, b, 2 * k + 2, (l + r) / 2, r);
        self.update(k);
    }
    fn _query_max(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if b <= l || r <= a {
            return -Self::inf;
        }
        if a <= l && r <= b {
            return self.max_v[k];
        }
        self.push(k);
        let lv = self._query_max(a, b, 2 * k + 1, l, (l + r) / 2);
        let rv = self._query_max(a, b, 2 * k + 2, (l + r) / 2, r);
        return max(lv, rv);
    }
    fn _query_min(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if b <= l || r <= a {
            return Self::inf;
        }
        if a <= l && r <= b {
            return self.min_v[k];
        }
        self.push(k);
        let lv = self._query_min(a, b, 2 * k + 1, l, (l + r) / 2);
        let rv = self._query_min(a, b, 2 * k + 2, (l + r) / 2, r);
        return min(lv, rv);
    }
    fn _query_sum(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if b <= l || r <= a {
            return 0;
        }
        if a <= l && r <= b {
            return self.sum[k];
        }
        self.push(k);
        let lv = self._query_sum(a, b, 2 * k + 1, l, (l + r) / 2);
        let rv = self._query_sum(a, b, 2 * k + 2, (l + r) / 2, r);
        return lv + rv;
    }
    fn query_min(&mut self, l: usize, r: usize) -> i64 {
        self._query_min(l, r, 0, 0, self.n0)
    }
    fn query_max(&mut self, l: usize, r: usize) -> i64 {
        self._query_max(l, r, 0, 0, self.n0)
    }
    fn query_sum(&mut self, l: usize, r: usize) -> i64 {
        self._query_sum(l, r, 0, 0, self.n0)
    }
    fn update_min(&mut self, l: usize, r: usize, x: i64) {
        self._update_min(x, l, r, 0, 0, self.n0)
    }
    fn update_max(&mut self, l: usize, r: usize, x: i64) {
        self._update_max(x, l, r, 0, 0, self.n0)
    }
    fn add_val(&mut self, l: usize, r: usize, x: i64) {
        self._add_val(x, l, r, 0, 0, self.n0)
    }
    fn update_val(&mut self, l: usize, r: usize, x: i64) {
        self._update_val(x, l, r, 0, 0, self.n0)
    }
}

#[test]
fn test_segbeats_simple() {
    let mut seg = SEGBeats::new(5);
    assert_eq!(seg.query_max(0, 5), 0);
    assert_eq!(seg.query_min(0, 5), 0);
    assert_eq!(seg.query_sum(0, 5), 0);
    for i in 0..5 {
        seg.update_val(i, i+1, i as i64 +1);
    }
    for i in 0..5 {
        seg.add_val(i, i+1, 0);
    }
    assert_eq!(seg.query_sum(0, 3), 6);
    assert_eq!(seg.query_max(0, 3), 3);
    assert_eq!(seg.query_min(0, 3), 1);

    assert_eq!(seg.query_sum(0, 5), 15);
    seg.update_min(0, 3, 2); // 1,2,2,4,5
    assert_eq!(seg.query_max(0, 3), 2);
    assert_eq!(seg.query_sum(0, 5), 14);
    seg.update_max(0, 3, 3); // 3,3,3,4,5
    assert_eq!(seg.query_sum(0, 5), 18);
    assert_eq!(seg.query_min(1, 5), 3);
    seg.update_min(2, 4, 2); // 3,3,2,2,5
    assert_eq!(seg.query_sum(0, 5), 15);
    assert_eq!(seg.query_min(0, 5), 2);
    assert_eq!(seg.query_max(2, 4), 2);
    assert_eq!(seg.query_max(2, 5), 5);
}