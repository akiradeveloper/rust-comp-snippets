use cargo_snippet::snippet;

#[snippet("Kadane")]
struct Kadane<T, Sum, Fold> {
    lmax_table: Vec<(usize,Sum)>,
    rmax_table: Vec<(usize,Sum)>,
    p1: std::marker::PhantomData<T>,
    p2: std::marker::PhantomData<Fold>,
}
#[snippet("Kadane")]
impl <T: Clone, Sum: std::cmp::PartialOrd + std::default::Default + Clone, Fold: Fn(&Sum,&T) -> Sum> Kadane<T, Sum, Fold> {
    pub fn new(a: Vec<T>, add: Fold) -> Kadane<T, Sum, Fold> {
        let L = Self::build_lmax(a.clone(), &add);
        let mut a = a;
        a.reverse();
        let mut R = Self::build_lmax(a, &add);
        R.reverse();
        Kadane {
            lmax_table: L,
            rmax_table: R,
            p1: std::marker::PhantomData,
            p2: std::marker::PhantomData,
        }
    }
    #[doc = "max{sum[l,*)}"]
    pub fn rmax(&self, l: usize) -> (usize, Sum) {
        let (len,sum) = self.rmax_table[l].clone();
        (l+len,sum)
    }
    #[doc = "max{sum[*,r)}"]
    pub fn lmax(&self, r: usize) -> (usize, Sum) {
        let (len,sum) = self.lmax_table[r].clone();
        (r-len,sum)
    }
    fn build_lmax(a: Vec<T>, fold: &Fold) -> Vec<(usize, Sum)> {
        let n = a.len();
        let mut res = vec![(0,Sum::default())];
        for r in 1..n+1 {
            let (L,ma) = res[r-1].clone();
            let i = r-1;
            let x = fold(&Sum::default(), &a[i]);
            let y = fold(&ma,&a[i]);
            let z = Sum::default();
            if z >= x && z >= y {
                res.push((0,Sum::default()));
            } else if x >= y {
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
    let kdn = Kadane::new(a, |&sum,&a| {sum+a});
    assert_eq!(kdn.rmax(0), (6,9));
    assert_eq!(kdn.lmax(6), (4,11));
    assert_eq!(kdn.lmax(5), (4,5));
    assert_eq!(kdn.lmax(2), (2,0));
    assert_eq!(kdn.lmax(4), (4,0));
    assert_eq!(kdn.lmax(3), (2,3));
}
#[test]
fn test_kadane_shortest_match() {
    let a = vec![0,-1,1];
    let kdn = Kadane::new(a, |&sum,&a| { sum+a });
    assert_eq!(kdn.rmax(0), (0,0));
}