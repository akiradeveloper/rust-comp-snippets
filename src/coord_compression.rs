use cargo_snippet::snippet;

#[snippet("CoordCompression")]
struct CoordCompression {
    comp: std::collections::HashMap<i64,usize>,
    dcmp: std::collections::HashMap<usize,i64>,
}
#[snippet("CoordCompression")]
impl CoordCompression {
    pub fn new(xs: &[i64], start: usize) -> CoordCompression {
        let mut xs = xs.to_owned();
        xs.sort();
        let mut comp = std::collections::HashMap::new();
        let mut dcmp = std::collections::HashMap::new();
        let mut acc = start;
        for x in xs {
            if comp.contains_key(&x) { continue; }
            comp.insert(x,acc);
            dcmp.insert(acc,x);
            acc+=1;
        }
        CoordCompression {
            comp: comp,
            dcmp: dcmp,
        }
    }
    pub fn compress(&self, x: i64) -> usize {
        *self.comp.get(&x).unwrap()
    }
    pub fn decompress(&self, x: usize) -> i64 {
        *self.dcmp.get(&x).unwrap()
    }
}
#[test]
fn test_coord_compression() {
    let v = vec![-2,3,99999,1000];
    let cc = CoordCompression::new(&v,0);
    assert_eq!(cc.compress(-2),0);
    assert_eq!(cc.compress(1000),2);
    assert_eq!(cc.decompress(1), 3);
    assert_eq!(cc.decompress(3),99999);
}