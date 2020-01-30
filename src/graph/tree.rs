#[snippet = "SubTree"]
struct SubTree {
    memo: Vec<Option<usize>>,
    g: Vec<Vec<usize>>,
}
#[doc = "Get the size of subtree"]
#[snippet = "SubTree"]
impl SubTree {
    #[doc = "directed / undirected"]
    pub fn new(tree: Vec<Vec<usize>>) -> SubTree {
        let g = tree;
        let n = g.len();
        SubTree {
            memo: vec![None;n],
            g: g,
        }
    }
    fn rec(&mut self, v: usize, pa: Option<usize>) -> usize {
        if let Some(x) = self.memo[v] {
            return x;
        }
        let mut tot = 1;
        for i in 0..self.g[v].len() {
            let u = self.g[v][i];
            if let Some(x) = pa {
                if x == u {
                    continue;
                }
            }
            tot += self.rec(u, Some(v));
        }
        self.memo[v] = Some(tot);
        tot
    }
    pub fn build(&mut self, root: usize) {
        self.rec(root, None);
    }
    pub fn size(&self, v: usize) -> usize {
        self.memo[v].unwrap()
    }
}
#[test]
fn test_subtree() {
    let e = vec![
        (0,1),
        (1,2),
        (2,3),
        (0,4),
    ];
    let mut g = vec![vec![];5];
    for (a,b) in e {
        g[a].push(b);
        g[b].push(a);
    }
    let mut st = SubTree::new(g);
    st.build(0);
    assert_eq!(st.size(3), 1);
    assert_eq!(st.size(4), 1);
    assert_eq!(st.size(0), 5);
    assert_eq!(st.size(1), 3);
    assert_eq!(st.size(2), 2);
}