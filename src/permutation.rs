use cargo_snippet::snippet;

/// 置換の二行記法
/// 
/// サイクルに分割することが出来る。
/// 
/// 計算量 O(N logN)

#[snippet("Permutation")]
struct Permutation {
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
    pub fn find_cycles(&self) -> Vec<Vec<usize>> {
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
            res.push(chain)
        }
        res
    }
}
#[test]
fn test_cyclic_permutation() {
    let p = Permutation::new(vec![0,1,2,3,4], vec![2,0,3,1,4]);
    assert_eq!(p.find_cycles(), vec![vec![0,2,3,1], vec![4]]);
    let p = Permutation::new(vec![0,1,2,3,4], vec![2,0,1,4,3]);
    assert_eq!(p.find_cycles(), vec![vec![0,2,1], vec![3,4]]);
}
