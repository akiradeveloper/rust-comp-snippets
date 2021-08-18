use cargo_snippet::snippet;

/// 互換 (x,y)
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Transposition {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Clone)]
/// 巡回置換
pub struct CyclicPermutation {
    member: Vec<usize>,
}
impl CyclicPermutation {
    fn new(xs: Vec<usize>) -> Self {
        CyclicPermutation { member: xs }
    }
    /// 巡回置換(a0,a1,a2...,an)は、
    /// 互換の列(a0,an),...,(a0,a2),(a0,a1)
    /// と等価である。
    /// 
    /// ある置換を構成する互換の個数が
    /// 偶数個の時、偶置換
    /// 奇数個の時、奇置換
    /// という。
    pub fn into_transpositions(self) -> Vec<Transposition> {
        if self.member.len() == 0 {
            vec![]
        } else {
            let mut ret = vec![];
            let x = self.member[0];
            let n = self.member.len();
            for i in 1..n {
                ret.push(Transposition { x, y: self.member[i] });
            }
            ret.reverse();
            ret
        }
    }
}


#[snippet("Permutation")]
/// 置換の二行記法
#[derive(Debug, Clone)]
pub struct Permutation {
    a: Vec<usize>,
    b: Vec<usize>
}
#[snippet("Permutation")]
impl Permutation {
    #[doc = "permutation a[i] -> b[i]"]
    pub fn new(a: Vec<usize>, b: Vec<usize>) -> Self {
        Permutation {
            a: a,
            b: b,
        }
    }
    /// 巡回置換に分解する。
    /// 計算量: O(NlogN)
    pub fn into_cycles(self) -> Vec<CyclicPermutation> {
        let mut ab = vec![];
        let mut next = std::collections::HashMap::new();
        let n = self.a.len();
        for i in 0..n {
            next.insert(self.a[i], self.b[i]);
            ab.push((self.a[i], self.b[i]));
        }
        ab.sort();
        ab.reverse();
    
        let mut res = vec![];
    
        let mut visited = vec![false; n];
        loop {
            if ab.is_empty() {
                break
            }
            let (head, _) = ab.pop().unwrap();
            if visited[head] {
                continue
            }
            let mut chain = vec![];
            let mut a = head;
            loop {
                chain.push(a);
                visited[a] = true;
                let b = next.get(&a).cloned().unwrap();
                if b == head {
                    break
                }
                a = b;
            }
            res.push(CyclicPermutation::new(chain))
        }
        res
    }
}
#[test]
fn test_cyclic_permutation() {
    use CyclicPermutation as C;

    let p = Permutation::new(vec![0,1,2,3,4], vec![2,0,3,1,4]);
    assert_eq!(p.into_cycles(), vec![C::new(vec![0,2,3,1]), C::new(vec![4])]);

    let p = Permutation::new(vec![0,1,2,3,4], vec![2,0,1,4,3]);
    assert_eq!(p.into_cycles(), vec![C::new(vec![0,2,1]), C::new(vec![3,4])]);

    let p = Permutation::new(vec![0,5,1,6,3,4,2], vec![5,0,6,3,4,1,2]);
    assert_eq!(p.into_cycles(), vec![C::new(vec![0,5]), C::new(vec![1,6,3,4]), C::new(vec![2])]);
}
#[test]
fn test_transpositions() {
    let p = Permutation::new(vec![0,5,1,3,2,4], vec![5,1,3,2,4,0]);
    let mut cs = p.into_cycles();
    assert_eq!(cs.len(), 1);
    let c = cs.remove(0);
    assert_eq!(c.into_transpositions(), vec![
        Transposition { x: 0, y: 4 },
        Transposition { x: 0, y: 2 },
        Transposition { x: 0, y: 3 },
        Transposition { x: 0, y: 1 },
        Transposition { x: 0, y: 5 },
    ]);
}