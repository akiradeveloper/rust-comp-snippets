use std::cmp::{max,min};
struct SEGBeats {
    max_v: Vec<i64>, smax_v: Vec<i64>, max_c: Vec<i64>,
    min_v: Vec<i64>, smin_v: Vec<i64>, min_c: Vec<i64>,
    sum: Vec<i64>,
    len: Vec<usize>, ladd: Vec<i64>, lval: Vec<i64>,
    n0: usize,
}
impl SEGBeats {
    const inf: i64 = std::i64::MAX / 4;
    pub fn new(n: usize) -> SEGBeats {
        let mut n0 = 1;
        while n0 < n { n0 <= 1; }
        let mut max_v = vec![0;2*n0];
        let mut smax_v = vec![0;2*n0];
        let mut max_c = vec![0;2*n0];
        let mut min_v = vec![0;2*n0];
        let mut smin_v = vec![0;2*n0];
        let mut min_c = vec![0;2*n0];
        let mut sum = vec![0;2*n0];
        let mut len = vec![0;2*n0];
        let mut ladd = vec![0;2*n0];
        let mut lval = vec![0;2*n0];

        for i in 0..2*n0 {
            ladd[i] = 0;
            lval[i] = Self::inf;
        }
        len[0] = n0;
        for i in 0..n0-1 {
            let l = len[i] >> 1;
            len[2*i+1] = l;
            len[2*i+2] = l;
        }
        for i in 0..n {
            let j = n0-1+i;
            max_v[j] = 0;
            min_v[j] = 0;
            sum[j] = 0;
            smax_v[j] = -Self::inf;
            smin_v[j] = Self::inf;
            max_c[j] = 1;
            min_c[j] = 1;
        }
        for i in n..n0 {
            let j = n0-1+i;
            max_v[j] = -Self::inf;
            smax_v[j] = -Self::inf;
            min_v[j] = Self::inf;
            smin_v[j] = Self::inf;
            max_c[j] = 0;
            min_c[j] = 0;
        }
        let mut ret = SEGBeats {
            max_v: max_v, smax_v: smax_v, max_c: max_c,
            min_v: min_v, smin_v: smin_v, min_c: min_c,
            sum: sum,
            len: len, ladd: ladd, lval: lval,
            n0: n0,
        };
        ret.build();
        ret
    }
    fn build(&mut self) {
        for i in (0..self.n0-1).rev() {
            self.update(i);
        }
    }
    fn update_node_max(&mut self, k: usize, x: i64) {
        self.sum[k] += (x - self.max_v[k]) * self.max_c[k];

        if (self.max_v[k] == self.min_v[k]) {
          self.max_v[k] = x;
          self.min_v[k] = x;
        } else if (self.max_v[k] == self.smin_v[k]) {
          self.max_v[k] = x;
          self.smin_v[k] = x;
        } else {
          self.max_v[k] = x;
        }
    
        if (self.lval[k] != Self::inf && x < self.lval[k]) {
          self.lval[k] = x;
        }
    }
    fn update_node_min(&mut self, k: usize, x: i64) {
        self.sum[k] += (x - self.min_v[k]) * self.min_c[k];

        if (self.max_v[k] == self.min_v[k]) {
          self.max_v[k] = x;
          self.min_v[k] = x;
        } else if (self.smax_v[k] == self.min_v[k]) {
          self.min_v[k] = x;
          self.smax_v[k] = x;
        } else {
          self.min_v[k] = x;
        }
    
        if (self.lval[k] != Self::inf && self.lval[k] < x) {
          self.lval[k] = x;
        }
    }
    fn push(&mut self, k: usize) {
        if (self.n0-1 <= k) { return; }

        if (self.lval[k] != Self::inf) {
          self.updateall(2*k+1, self.lval[k]);
          self.updateall(2*k+2, self.lval[k]);
          self.lval[k] = Self::inf;
          return;
        }
    
        if(self.ladd[k] != 0) {
          self.addall(2*k+1, self.ladd[k]);
          self.addall(2*k+2, self.ladd[k]);
          self.ladd[k] = 0;
        }
    
        if(self.max_v[k] < self.max_v[2*k+1]) {
          self.update_node_max(2*k+1, self.max_v[k]);
        }
        if(self.min_v[2*k+1] < self.min_v[k]) {
          self.update_node_min(2*k+1, self.min_v[k]);
        }
    
        if(self.max_v[k] < self.max_v[2*k+2]) {
          self.update_node_max(2*k+2, self.max_v[k]);
        }
        if(self.min_v[2*k+2] < self.min_v[k]) {
          self.update_node_min(2*k+2, self.min_v[k]);
        }
    }
    fn update(&mut self, k: usize) {
        self.sum[k] = self.sum[2*k+1] + self.sum[2*k+2];

        if(self.max_v[2*k+1] < self.max_v[2*k+2]) {
          self.max_v[k] = self.max_v[2*k+2];
          self.max_c[k] = self.max_c[2*k+2];
          self.smax_v[k] = std::cmp::max(self.max_v[2*k+1], self.smax_v[2*k+2]);
        } else if(self.max_v[2*k+1] > self.max_v[2*k+2]) {
          self.max_v[k] = self.max_v[2*k+1];
          self.max_c[k] = self.max_c[2*k+1];
          self.smax_v[k] = std::cmp::max(self.smax_v[2*k+1], self.max_v[2*k+2]);
        } else {
          self.max_v[k] = self.max_v[2*k+1];
          self.max_c[k] = self.max_c[2*k+1] + self.max_c[2*k+2];
          self.smax_v[k] = std::cmp::max(self.smax_v[2*k+1], self.smax_v[2*k+2]);
        }
    
        if(self.min_v[2*k+1] < self.min_v[2*k+2]) {
          self.min_v[k] = self.min_v[2*k+1];
          self.min_c[k] = self.min_c[2*k+1];
          self.smin_v[k] = min(self.smin_v[2*k+1], self.min_v[2*k+2]);
        } else if(self.min_v[2*k+1] > self.min_v[2*k+2]) {
          self.min_v[k] = self.min_v[2*k+2];
          self.min_c[k] = self.min_c[2*k+2];
          self.smin_v[k] = min(self.min_v[2*k+1], self.smin_v[2*k+2]);
        } else {
          self.min_v[k] = self.min_v[2*k+1];
          self.min_c[k] = self.min_c[2*k+1] + self.min_c[2*k+2];
          self.smin_v[k] = min(self.smin_v[2*k+1], self.smin_v[2*k+2]);
        }
    }
    fn _update_min(&mut self) {
        if(b <= l || r <= a || self.max_v[k] <= x) {
            return;
          }
          if(a <= l && r <= b && self.smax_v[k] < x) {
            self.update_node_max(k, x);
            return;
          }
      
          self.push(k);
          self._update_min(x, a, b, 2*k+1, l, (l+r)/2);
          self._update_min(x, a, b, 2*k+2, (l+r)/2, r);
          self.update(k);
    }
    fn _update_max(&mut self) {
        if(b <= l || r <= a || x <= self.min_v[k]) {
            return;
          }
          if(a <= l && r <= b && x < self.smin_v[k]) {
            self.update_node_min(k, x);
            return;
          }
      
          self.push(k);
          self._update_max(x, a, b, 2*k+1, l, (l+r)/2);
          self._update_max(x, a, b, 2*k+2, (l+r)/2, r);
          self.update(k);
    }
    fn add_all(&mut self) {
        self.max_v[k] += x;
        if(self.smax_v[k] != -Self::inf) self.smax_v[k] += x;
        self.min_v[k] += x;
        if(self.smin_v[k] != Self::inf) self.smin_v[k] += x;
    
        self.sum[k] += self.len[k] * x;
        if(self.lval[k] != Self::inf) {
          self.lval[k] += x;
        } else {
          self.ladd[k] += x;
        }
    }
    fn update_all(&mut self) {
        self.max_v[k] = x; self.smax_v[k] = -Self::inf;
        self.min_v[k] = x; self.smin_v[k] = Self::inf;
        self.max_c[k] = self.len[k];
        self.min_c[k] = self.len[k];
    
        self.sum[k] = x * len[k];
        self.lval[k] = x; self.ladd[k] = 0;
    }
    fn _add_val(&mut self) {
        if(b <= l || r <= a) {
            return;
          }
          if(a <= l && r <= b) {
            addall(k, x);
            return;
          }
      
          push(k);
          _add_val(x, a, b, 2*k+1, l, (l+r)/2);
          _add_val(x, a, b, 2*k+2, (l+r)/2, r);
          update(k);
    }
    fn _update_val(&mut self) {
        if(b <= l || r <= a) {
            return;
          }
          if(a <= l && r <= b) {
            updateall(k, x);
            return;
          }
      
          push(k);
          _update_val(x, a, b, 2*k+1, l, (l+r)/2);
          _update_val(x, a, b, 2*k+2, (l+r)/2, r);
          update(k);
    }
    fn _query_max(&mut self) {
        if(b <= l || r <= a) {
            return -inf;
          }
          if(a <= l && r <= b) {
            return max_v[k];
          }
          push(k);
          ll lv = _query_max(a, b, 2*k+1, l, (l+r)/2);
          ll rv = _query_max(a, b, 2*k+2, (l+r)/2, r);
          return max(lv, rv);
    }
    fn _query_min(&mut self) {
        if(b <= l || r <= a) {
            return inf;
          }
          if(a <= l && r <= b) {
            return min_v[k];
          }
          push(k);
          ll lv = _query_min(a, b, 2*k+1, l, (l+r)/2);
          ll rv = _query_min(a, b, 2*k+2, (l+r)/2, r);
          return min(lv, rv);
    }
    fn _query_sum(&mut self) {
        if(b <= l || r <= a) {
            return 0;
          }
          if(a <= l && r <= b) {
            return sum[k];
          }
          push(k);
          ll lv = _query_sum(a, b, 2*k+1, l, (l+r)/2);
          ll rv = _query_sum(a, b, 2*k+2, (l+r)/2, r);
          return lv + rv;
    }
}