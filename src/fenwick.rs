struct BIT {
    n: usize,
    dat: Vec<i32>,
}

impl BIT {
    fn new(n: usize) -> BIT {
        BIT {
            n: n,
            dat: vec![0; n+1], // [1,n]
        }
    }
    fn last_bit(&self, i: usize) -> usize {
        let i = i as isize;
        (i & -i) as usize
    }
    fn sum(&self, i: usize) -> i32 {
        self._sum(i+1)
    }
    fn _sum(&self, i: usize) -> i32 {
        let mut i = i;
        let mut s = 0;
        while i > 0 {
            s += self.dat[i];
            i -= self.last_bit(i);
        }
        s
    }
    fn add(&mut self, i: usize, x: i32) {
        self._add(i+1, x)
    }
    fn _add(&mut self, i: usize, x: i32) {
        let mut i = i;
        while i <= self.n {
            self.dat[i] += x;
            i += self.last_bit(i);
        }
    }
}

#[test]
fn test() {

}