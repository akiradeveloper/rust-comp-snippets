/// [bluss/permutohedron](https://github.com/bluss/permutohedron)

#[snippet = "LexicalPermutation"]
pub trait LexicalPermutation {
    fn next_permutation(&mut self) -> bool;
    fn prev_permutation(&mut self) -> bool;
}

#[snippet = "LexicalPermutation"]
impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i - 1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }

    fn prev_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }

        // Step 1: Identify the longest, rightmost weakly increasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the first-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Reverse the weakly increasing part
        self[i..].reverse();

        // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j - 1] < self[i - 1] {
            j -= 1;
        }

        // Step 4: Swap that element with the pivot
        self.swap(i - 1, j);

        true
    }
}
#[test]
fn test_lexical_permutation() {
    // use LexicalPermutation;
    let mut x = vec![0,1,2,3];
    loop {
        dbg!(&x);
        if !x.next_permutation() {
            break;
        }
    }
}

#[snippet = "cyclic_permutation"]
#[doc = "allows both 0/1-indexd. O(nlogn)"]
pub fn cyclic_permutation(a: &[usize], b: &[usize]) -> Vec<Vec<usize>> {
    let mut ab = vec![];
    let mut next = std::collections::HashMap::new();
    let n = a.len();
    for i in 0..n {
        next.insert(a[i], b[i]);
        ab.push((a[i], b[i]));
    }
    ab.sort();
    ab.reverse();

    let mut res = vec![];

    let mut visited = vec![false; n+1];
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

#[test]
fn test_cyclic_permutation() {
    assert_eq!(cyclic_permutation(&[0,1,2,3,4], &[2,0,3,1,4]), vec![vec![0,2,3,1], vec![4]]);
    assert_eq!(cyclic_permutation(&[0,1,2,3,4], &[2,0,1,4,3]), vec![vec![0,2,1], vec![3,4]]);
    assert_eq!(cyclic_permutation(&[1,2,3,4,5], &[3,1,2,5,4]), vec![vec![1,3,2], vec![4,5]]);
}