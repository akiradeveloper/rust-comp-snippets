use cargo_snippet::snippet;

#[snippet("Kadane")]
struct Kadane {
    lmax_table: Vec<(usize,i64)>,
    rmax_table: Vec<(usize,i64)>,
}
#[snippet("Kadane")]
impl Kadane {
    fn new(a: Vec<i64>) -> Kadane {
        let L = Self::build_lmax(a.clone());
        let mut a = a;
        a.reverse();
        let mut R = Self::build_lmax(a);
        R.reverse();
        Kadane {
            lmax_table: L,
            rmax_table: R,
        }
    }
    #[doc = "max{sum[l,*)}"]
    pub fn rmax(&self, l: usize) -> (usize, i64) {
        self.rmax_table[l]
    }
    #[doc = "max{sum[*,r)}"]
    pub fn lmax(&self, r: usize) -> (usize, i64) {
        self.lmax_table[r]
    }
    fn build_lmax(a: Vec<i64>) -> Vec<(usize, i64)> {
        let n = a.len();
        let mut res = vec![(0,0)];
        for r in 1..n+1 {
            let (L,ma) = res[r-1];
            let i = r-1;
            let x = a[i];
            let y = ma+a[i];
            let z = 0;
            if z > x && z > y {
                res.push((0,0));
            } else if x > y {
                res.push((1,x));
            } else {
                res.push((L+1,y));
            }
        }
        res
    }
}
#[test]
fn test_kadane() {
    let a = vec![1,-2,3,-4,5,6];
    let kdn = Kadane::new(a);
    assert_eq!(kdn.rmax(0), (6,9));
    assert_eq!(kdn.lmax(6), (2,11));
    assert_eq!(kdn.lmax(5), (1,5));
    assert_eq!(kdn.lmax(2), (0,0));
    assert_eq!(kdn.lmax(4), (0,0));
    assert_eq!(kdn.lmax(3), (1,3));
}