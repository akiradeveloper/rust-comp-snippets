struct Syakutori<F> {
    n: usize,
    f: F,
}
#[doc = "[i,j)"]
impl <F: FnMut(usize, usize) -> bool> Syakutori<F> {
    fn new(n: usize, f: F) -> Self {
        Syakutori {
            n: n,
            f: f,
        }
    }
    fn run(&mut self) {
        let mut j = 0;
        for i in 0..self.n {
            while !(self.f)(i,j) {
                if j == self.n {
                    break;
                }
                j += 1;
            }
            (self.f)(i, j);
        }
    }
}