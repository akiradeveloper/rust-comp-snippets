use cargo_snippet::snippet;

#[snippet("EularTour")]
#[derive(Debug)]
struct EularTour {
    g: Vec<Vec<usize>>,
    n: usize,
    vid: usize,
    enter: Vec<usize>,
    leave: Vec<usize>,
}
#[snippet("EularTour")]
impl EularTour {
    pub fn new(n: usize) -> EularTour {
        EularTour {
            g: vec![vec![];n],
            n: n,
            vid: 0,
            enter: vec![n;n],
            leave: vec![n;n],
        }
    }
    pub fn connect(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
    }
    fn alloc_vid(&mut self) -> usize {
        let tmp = self.vid;
        self.vid += 1;
        tmp
    }
    pub fn dfs(&mut self, u: usize) {
        self.rec(u,self.n);
    }
    fn rec(&mut self, u: usize, p: usize) {
        self.enter[u] = self.alloc_vid();
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            if v == p { continue; }
            self.rec(v,u);
        }
        self.leave[u] = self.alloc_vid();
    }
}
#[test]
fn test_eular_tour() {
    let mut g = EularTour::new(5);
    g.connect(0, 1);
    g.connect(0, 2);
    g.connect(1, 3);
    g.connect(1, 4);
    g.dfs(0);
    assert_eq!(g.enter, vec![0,1,7,2,4]);
    assert_eq!(g.leave, vec![9,6,8,3,5]);
}