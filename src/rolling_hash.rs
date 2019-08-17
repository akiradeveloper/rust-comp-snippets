pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
struct RollingHashRaw {
    hash: Vec<usize>,
    m: usize, // len(t)
    b: usize,
    h: usize,
    b_pow: Vec<usize>, // b^0 .., b^m
}
impl RollingHashRaw {
    fn calc_hash(s: &[usize], m: usize, b_pow: &[usize], h: usize) -> usize {
        let mut res = 0;
        for i in 0..m {
            res += s[i] * b_pow[m-1-i];
            res %= h;
        } 
        res
    }
    fn new(s: &[usize], m: usize, b: usize, h: usize) -> RollingHashRaw {
        let n = s.len();
        assert_eq!(gcd(b, h), 1);
        assert!(n>=m);
        let mut b_pow = vec![1];
        let mut acc = 1;
        for _ in 1..m+1 {
            acc *= b;
            acc %= h;
            b_pow.push(acc);
        }
        let mut hash = vec![];
        let mut cur_hash = Self::calc_hash(&s, m, &b_pow, h);
        hash.push(cur_hash);
        for i in 1..n+1-m {
            let k = i-1;
            cur_hash *= b;
            cur_hash %= h;
            cur_hash -= s[k] * b_pow[m];
            cur_hash %= h;
            cur_hash += s[k+m];
            cur_hash %= h;
            hash.push(cur_hash);
        }
        RollingHashRaw {
            hash: hash,
            m: m,
            b: b,
            h: h,
            b_pow: b_pow,
        }
    }
    fn find(&self, t: &[usize], from: usize) -> Option<usize> {
        let th = Self::calc_hash(t, t.len(), &self.b_pow, self.h);
        let mut res = None;
        for k in from..self.hash.len() {
            if self.hash[k] == th {
                res = Some(k);
                break;
            }
        }
        res
    }
    fn find_all(&self, t: &[usize]) -> Vec<usize> {
        let th = Self::calc_hash(t, t.len(), &self.b_pow, self.h);
        let mut res = vec![];
        for k in 0..self.hash.len() {
            if self.hash[k] == th {
                res.push(k);
            }
        }
        res
    }
    fn hash_len(&self) -> usize {
        self.hash.len()
    }
}

#[test]
fn test_rolling_hash_raw() {
    let s = vec![1,2,1,2,1,2,1,2];
    let rh = RollingHashRaw::new(&s, 3, 1009, 1_000_000_007);
    let t1 = vec![1,2,1];
    assert_eq!(rh.find_all(&t1), [0,2,4]);
    let t2 = vec![1,2,3];
    assert_eq!(rh.find_all(&t2), []);

    assert_eq!(rh.find(&t1, 0), Some(0));
    assert_eq!(rh.find(&t1, 1), Some(2));
    assert_eq!(rh.find(&t1, 2), Some(2));
    assert_eq!(rh.find(&t1, 5), None);
}

struct RollingHash {
    bh_set: Vec<(usize, usize)>,
    rhs: Vec<RollingHashRaw>,
}
impl RollingHash {
    fn new(s: &[usize], m: usize) -> RollingHash {
        let bh_set = vec![(1009, 1_000_000_007), (1007, 1_000_000_009)];
        let mut rhs = vec![];
        for i in 0..bh_set.len() {
            let (b,h) = bh_set[i];
            rhs.push(RollingHashRaw::new(s, m, b, h));
        }
        RollingHash {
            bh_set,
            rhs,
        }
    }
    // O(m+n)
    fn find(&self, t: &[usize], from: usize) -> Option<usize> {
        let mut results = vec![];
        for rh in &self.rhs {
            results.push(rh.find(t, from));
        }
        let base = results[0];
        for result in results {
            if result != base {
                return None;
            }
        }
        base
    }
    // O(m+n)
    fn find_all(&self, t: &[usize]) -> Vec<usize> {
        let mut results = vec![];
        for rh in &self.rhs {
            results.push(rh.find_all(t));
        }
        let mut cnt = vec![0; self.rhs[0].hash_len()];
        for result in results {
            for i in result {
                cnt[i] += 1;
            }
        }
        let mut res = vec![];
        for i in 0..cnt.len() {
            if cnt[i] == self.rhs.len() {
                res.push(i);
            }
        }
        res
    }
}

#[test]
fn test_rolling_hash() {
    let s = vec![1,2,1,2,1,2,1,2];
    let rh = RollingHash::new(&s, 3);
    let t1 = vec![1,2,1];
    assert_eq!(rh.find_all(&t1), [0,2,4]);
    let t2 = vec![1,2,3];
    assert_eq!(rh.find_all(&t2), []);

    assert_eq!(rh.find(&t1, 0), Some(0));
    assert_eq!(rh.find(&t1, 1), Some(2));
    assert_eq!(rh.find(&t1, 2), Some(2));
    assert_eq!(rh.find(&t1, 5), None);
}