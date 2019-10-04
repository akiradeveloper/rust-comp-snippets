#[snippet = "SA"]
struct SA {
    // sのうち後ろからSA[i]個消したやつが辞書順i番目のsuffixである
    sa: Vec<usize>,
    s: Vec<usize>
}
#[snippet = "SA"]
impl SA {
    fn new(s: Vec<usize>) -> Self {
        let mut s = s;
        s.push('$' as usize);
        let mut sa = Self::sort_cyclic_shifts(&s);
        sa.remove(0);
        s.remove(s.len()-1);
        SA {
            sa: sa,
            s: s
        }
    }
    fn sort_cyclic_shifts(s: &[usize]) -> Vec<usize> {
        let n = s.len();
        const alphabet: usize = 256;
        let mut p = vec![0;n];
        let mut c = vec![0;n];
        let mut cnt = vec![0;std::cmp::max(alphabet,n)];

        for i in 0..n {
            cnt[s[i]] += 1;
        }
        for i in 1..alphabet {
            cnt[i] += cnt[i-1];
        }
        for i in 0..n {
            cnt[s[i]] -= 1;
            p[cnt[s[i]]] = i;
        }
        c[p[0]] = 0;
        let mut classes = 1;
        for i in 1..n {
            if s[p[i]] != s[p[i-1]] {
                classes += 1;
            }
            c[p[i]] = classes - 1;
        }

        let mut pn = vec![0;n];
        let mut cn = vec![0;n];
        for k in 0.. { // OK in 1.15
            if (1<<k) >= n { break; }

            for i in 0..n {
                if p[i] >= (1<<k) {
                    pn[i] = p[i] - (1<<k);
                } else {
                    pn[i] = p[i] + n - (1<<k);
                }
            }
            for i in 0..classes {
                cnt[i] = 0;
            }
            for i in 0..n {
                cnt[c[pn[i]]] += 1;
            }
            for i in 1..classes {
                cnt[i] += cnt[i-1];
            }
            for i in (0..n).rev() {
                cnt[c[pn[i]]] -= 1;
                p[cnt[c[pn[i]]]] = pn[i];
            }
            cn[p[0]] = 0;
            classes = 1;
            for i in 1..n {
                let cur = (c[p[i]], c[(p[i] + (1<<k)) %n]);
                let prev = (c[p[i-1]], c[(p[i-1] + (1<<k)) %n]);
                if cur != prev {
                    classes += 1;
                }
                cn[p[i]] = classes - 1;
            }
            let tmp = c;
            c = cn;
            cn = tmp
        }
        p
    }
    fn lt_substr(&self, t: &[usize], si: usize, ti: usize) -> bool {
        let sn = self.s.len();
        let tn = t.len();
        false
    }
    pub fn range(&self, t: &[usize]) -> (usize, usize) {
        (0,0)
    }
}
#[test]
fn test_sa() {
    let mut s = "abracadabra";
    let mut v = vec![];
    for c in s.chars() {
        v.push(c as usize);
    }
    let mut sa = SA::new(v);
    assert_eq!(sa.sa, [10,7,0,3,5,8,1,4,6,9,2]);
}