struct SEGBeats {
    max_v: Vec<i64>, smax_v: Vec<i64>, max_c: Vec<i64>,
    min_v: Vec<i64>, smin_v: Vec<i64>, min_c: Vec<i64>,
    sum: Vec<i64>,
    len: Vec<usize>, ladd: Vec<i64>, lval: Vec<i64>,
    n0: usize,
}
impl SEGBeats {
    pub fn new(n: usize) -> SEGBeats {
        let inf = std::i64::MAX / 4;

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
            lval[i] = inf;
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
            smax_v[j] = -inf;
            smin_v[j] = inf;
            max_c[j] = 1;
            min_c[j] = 1;
        }
        for i in n..n0 {
            let j = n0-1+i;
            max_v[j] = -inf;
            smax_v[j] = -inf;
            min_v[j] = inf;
            smin_v[j] = inf;
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
    fn update(&mut self, k: usize) {

    }
}