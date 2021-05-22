use cargo_snippet::snippet;
use std::collections::HashMap;

/// 木の重心分解
/// 
/// 木の重心とは、その頂点の周りにある部分木が
/// どれも元の木の半分以下の大きさになってる頂点のことである。
/// 
/// アイデア:
/// 適当な頂点から木DPをして、部分木のサイズを求める。
/// そして、全頂点のうち、全部分木がN/2以下のサイズになってるものを列挙する。
/// 自分を頂点とした部分木のサイズが求まれば、親ノード側の部分木のサイズも求まる。

#[snippet("TreeCentroid")]
pub struct Centroid {
    pub g: Vec<Vec<usize>>,
    pub centroid: Vec<usize>,
    subsize: Vec<usize>,
}
#[snippet("TreeCentroid")]
impl Centroid {
    pub fn new(n: usize) -> Centroid {
        Centroid {
            g: vec![vec![];n],
            centroid: vec![],
            subsize: vec![0;n],
        }
    }
    pub fn connect(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
    }
    #[doc = "O(nlogn)"]
    pub fn build(&mut self) {
        let n = self.g.len();
        self.rec(0, n);
    }
    fn rec(&mut self, u: usize, par: usize) {
        let n = self.g.len();
        self.subsize[u] = 1;
        let mut is_centroid = true;
        for i in 0..self.g[u].len() {
            let ch = self.g[u][i];
            if ch == par { continue; }
            self.rec(ch,u);
            if self.subsize[ch] > n/2 {
                is_centroid = false;
            }
            self.subsize[u] += self.subsize[ch];
        }
        if n - self.subsize[u] > n/2 {
            is_centroid = false;
        }
        if is_centroid {
            self.centroid.push(u);
        }
    }
}

/// 木をある頂点を中心に分解する
/// nodeidは、初期の木における頂点番号を保持していて、
/// eは、nodeid配列内のindexを用いて無向辺を表現する。

#[snippet("split_tree")]
#[derive(Debug)]
pub struct SubTree {
    nodeid: Vec<usize>,
    e: Vec<(usize,usize)>,
}
#[snippet("split_tree")]
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
    let n = tree.nodeid.len();
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
            nodeid: nodeid,
            e: e,
        });
    }
    res
}

#[test]
fn test_centroid() {
    let e = vec![
        (0,1),
        (1,2),
        (3,2),
        (4,2),
        (5,4),
        (6,4),
        (7,2),
        (8,7),
        (9,8),
        (10,2),
        (11,10),
        (12,2),
        (13,12),
        (14,13),
        (15,13),
        (16,12),
        (17,16),
        (18,16),
        (19,18),
        (20,12),
        (21,20),
        (22,21),
        (23,21),
    ];
    let n = 24;
    let mut g = Centroid::new(n);
    for &(u,v) in &e {
        g.connect(u,v);
    }
    g.build();
    g.centroid.sort();
    assert_eq!(g.centroid, vec![2,12]);

    let mut nodeid = vec![0;24];
    for i in 0..24 {
        nodeid[i] = i;
    }
    let tree = SubTree {
        nodeid: nodeid,
        e: e,
    };
    let subtrees = split_tree(tree, 2);
    dbg!(subtrees);
}