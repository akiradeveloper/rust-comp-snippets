use std::collections::HashMap;

#[snippet = "TreeCentroid"]
pub struct Centroid {
    g: Vec<Vec<usize>>,
}
#[snippet = "TreeCentroid"]
impl Centroid {
    pub fn new(n: usize) -> Centroid {
        Centroid {
            g: vec![vec![];n],
        }
    }
    pub fn connect(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
    }
    #[doc = "O(nlogn)"]
    pub fn find_centroid(&mut self) -> usize {
        0
    }
}

#[snippet = "split_tree"]
pub struct SubTree {
    n: usize,
    e: Vec<(usize,usize)>,
    nodeid: Vec<usize>,
}
#[snippet = "split_tree"]
#[doc = "O(n)"]
pub fn split_tree(tree: SubTree, root: usize) -> Vec<SubTree> {
    struct Rec<'a> {
        g: &'a Vec<Vec<usize>>,
        vs: Vec<usize>,
        es: Vec<(usize,usize)>,
    }
    impl <'a> Rec<'a> {
        fn solve(&mut self, u: usize, par: usize) {
            self.vs.push(u);
            for i in 0..self.g[u].len() {
                let v = self.g[u][i];
                if v != par {
                    self.es.push((u,v));
                    self.solve(v,u);
                }
            }
        }
    }

    let mut res = vec![];
    let n = tree.n;
    let mut g = vec![vec![];n];
    for (u,v) in tree.e {
        g[u].push(v);
        g[v].push(u);
    }

    for i in 0..g[root].len() {
        let u = g[root][i];
        let mut rec = Rec {
            g: &g,
            vs: vec![],
            es: vec![],
        };
        rec.solve(u, root);

        let m = rec.vs.len();
        let mut rev = HashMap::new();
        for i in 0..m {
            rev.insert(rec.vs[i], i);
        }
        let mut nodeid = vec![m;m];
        for i in 0..m {
            let u = rec.vs[i];
            nodeid[i] = tree.nodeid[u];
        }

        let mut e = vec![];
        for (u,v) in rec.es {
            let uu = *rev.get(&u).unwrap();
            let vv = *rev.get(&v).unwrap();
            e.push((uu,vv));
        }

        res.push(SubTree {
            n: m,
            e: e,
            nodeid: nodeid,
        });
    }
    res
}