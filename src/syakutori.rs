struct Syakutori<F, G> {
    n: usize,
    f: F,
    g: G,
}
#[doc = "[i,j)"]
impl <F: FnMut(usize, usize) -> bool, G: FnMut(usize, usize)> Syakutori<F, G> {
    fn new(n: usize, f: F, g: G) -> Self {
        Syakutori {
            n: n,
            f: f,
            g: g,
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
            (self.g)(i, j);
        }
    }
}