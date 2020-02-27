use crate::union_find::UnionFind;
use std::collections::HashMap;

#[derive(Debug)]
struct Connected {
    n: usize,
    e: Vec<(usize,usize)>,
    nodeid: Vec<usize>,
}
struct Partition {
    n: usize,
    e: Vec<(usize,usize)>,
    uf: UnionFind
}
impl Partition {
    fn new(n: usize) -> Partition {
        Partition {
            n: n,
            e: vec![],
            uf: UnionFind::new(n),
        }
    }
    fn connect(&mut self, u: usize, v: usize) {
        let mi = std::cmp::min(u,v);
        let ma = std::cmp::max(u,v);
        self.e.push((mi,ma));
        self.uf.merge(mi, ma);
    }
    fn build(&mut self) -> Vec<Connected> {
        let mut E = vec![vec![];self.n];
        for &(u,v) in &self.e {
            E[u].push((u,v));
            E[v].push((u,v));
        }
        let mut grp = HashMap::new();
        for i in 0..self.n {
            let root = self.uf.root(i);
            grp.entry(root).or_insert(vec![]).push(i);
        }
        dbg!(&grp);
        let mut res = vec![];
        for (_, nodeid) in grp {
            let n = nodeid.len();
            let mut es = vec![];
            let mut rev = HashMap::new();
            for i in 0..n {
                let v = nodeid[i];
                rev.insert(v,i);
                for &(a,b) in &E[v] {
                    es.push((a,b));
                }
            }
            dbg!(&es);
            es.sort();
            es.dedup();
            let mut nes = vec![];
            for (u,v) in es {
                let a = *rev.get(&u).unwrap();
                let b = *rev.get(&v).unwrap();
                nes.push((a,b));
            }
            res.push(Connected {
                n: n,
                e: nes,
                nodeid: nodeid,
            })
        }
        res
    }
}
#[test]
fn test_partition() {
    let e = vec![
        (1,2),(2,3),(3,4),(4,1),(5,6),(6,7),(7,5),
    ];
    let mut p = Partition::new(8);
    for (u,v) in e {
        p.connect(u, v);
    }
    dbg!(p.build());
}