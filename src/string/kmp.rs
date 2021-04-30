use cargo_snippet::snippet;

/// パターン文字列についてテーブルを構築
/// 構築計算量 O(|P|)
/// 
/// 文字列SからPを全検索
/// 計算量 O(|S|)

#[snippet("KMP")]
struct KMP {
    pat: Vec<u64>,
    tbl: Vec<i64>
}
#[snippet("KMP")]
impl KMP {
    #[doc = "O(K)"]
    fn new(pat: Vec<u64>) -> Self {
        let n = pat.len();
        let mut tbl = vec![0; n+1];
        tbl[0] = -1;
        let mut j: i64 = -1;
        for i in 0..n {
            while j>=0 && pat[i] != pat[j as usize] {
                j = tbl[j as usize];
            }
            j+=1;

            // ここは単にtbl[i+1]=jでもverify出来る
            if i+1<n && pat[i+1] == pat[j as usize] {
                tbl[i+1] = tbl[j as usize];
            } else {
                tbl[i+1] = j;
            }
        }
        KMP {
            pat: pat,
            tbl: tbl,
        }
    }
    pub fn search(&self, s: &[u64]) -> Vec<usize> {
        let mut res = vec![];
        let mut head: i64 = 0;
        let mut j: i64 = 0;
        while head + j < s.len() as i64 {
            if self.pat[j as usize] == s[(head + j) as usize] {
                j += 1;
                if j != self.pat.len() as i64 { continue; }
                res.push(head as usize);
            }
            head += j - self.tbl[j as usize];
            j = std::cmp::max(self.tbl[j as usize], 0);
        }
        res
    }
}

#[test]
fn test_kmp() {
    let s = "abababa";
    let w = "aba";
    let mut ss = vec![];
    for c in s.chars() {
        let n = (c as u64) - 'a' as u64;
        ss.push(n);
    }
    let mut ww = vec![];
    for c in w.chars() {
        let n = (c as u64) - 'a' as u64;
        ww.push(n);
    }
    let kmp = KMP::new(ww);
    dbg!(&kmp.tbl);
    assert_eq!(kmp.search(&ss), [0,2,4]);
}

#[bench] 
fn bench_kmp(b: &mut test::Bencher) {
    let mut p = vec![0];
    let mut s = vec![0;1000000];
    let kmp = KMP::new(p);
    b.iter(||
        kmp.search(&s)
    );
}