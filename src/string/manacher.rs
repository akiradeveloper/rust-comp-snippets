use cargo_snippet::snippet;

/// 各iを中心にした回文半径を計算する。
/// 回文半径は、(直径+1)/2で計算される。（つまり中心を含む）
/// ここで直径は常に奇数

#[snippet("Manacher")]
fn manacher(s: &[u64]) -> Vec<usize> {
    let mut r = vec![0; s.len()];
    let mut i = 0;
    let mut j = 0;
    while i < s.len() {
        while i >= j && i+j < s.len() && s[i-j] == s[i+j] {
            j += 1;
        }
        r[i] = j;
        let mut k = 1;
        while i >= k && i+k < s.len() && k+r[i-k] < j {
            r[i+k] = r[i-k]; 
            k += 1;
        }
        i += k;
        j -= k;
    }
    r
}
#[test]
fn test_manacher_detect_only_even() {
    let s = vec![1,1];
    // 奇数長回文しか検出しない（長さ3でやっと半径2になる）
    assert_eq!(manacher(&s), [1,1]);
}
#[test]
fn test_manacher_raw() {
    let s = vec![0,1,0,0,0,1,0,1,0,0,1];
    assert_eq!(manacher(&s), [1,2,1,4,1,2,4,2,1,1,1]);
}

#[snippet("Manacher")]
struct Manacher {
    a: Vec<usize>,
}
#[snippet("Manacher")]
impl Manacher {
    pub fn new(s: Vec<u64>) -> Self {
        let sep = 1<<60;
        let n = s.len();
        let m = 2*n+1;
        // sepを挟む工夫
        // sep,s[0],sep,s[1],...,s[n-1],sep
        let mut t = vec![sep; m];
        for i in 0..n {
            let k = (2*i)+1;
            t[k] = s[i];
        }
        let a = manacher(&t);
        Manacher {
            a: a,
        }
    }
    /// 通常のmanacher
    /// ある文字を中心とした半径
    pub fn radius_odd(&self, i: usize) -> usize {
        let k = (2*i)+1;
        let r = self.a[k];
        r/2
    }
    /// 仕切りを中心とした半径
    pub fn radius_even(&self, i: usize) -> usize {
        let k = 2*i;
        let r = self.a[k];
        r/2
    } 
    pub fn is_pelimdrome(&self, l: usize, r: usize) -> bool {
        let len = r-l;
        if len%2 == 0 {
            let i = (l+r)/2;
            let r = len/2;
            self.radius_even(i) >= r
        } else {
            let i = (l+r)/2;
            let r = (len+1)/2;
            self.radius_odd(i) >= r
        }
    }
}
#[test]
fn test_manacher() {
    let s = vec![0,1,0,0,0,1,0,1,0,0,1];
    let m = Manacher::new(s);
    assert_eq!(m.radius_odd(3),4);
    assert_eq!(m.radius_odd(1),2);
    assert_eq!(m.radius_even(3),1);
    assert_eq!(m.radius_even(2),0);
    assert_eq!(m.radius_even(9),2);

    assert_eq!(m.is_pelimdrome(0, 6),false);
    assert_eq!(m.is_pelimdrome(0, 7),true);
    assert_eq!(m.is_pelimdrome(2, 4),true);
    assert_eq!(m.is_pelimdrome(2, 5),true);
    assert_eq!(m.is_pelimdrome(3, 5),true);
    assert_eq!(m.is_pelimdrome(7, 11),true);
    assert_eq!(m.is_pelimdrome(8, 10),true);
}