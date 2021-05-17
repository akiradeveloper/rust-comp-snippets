use cargo_snippet::snippet;

/// 範囲[l,r)の中の最小値のインデックスを求める
/// 構築 O(N logN)
/// クエリ O(1)

#[snippet("SparseTable")]
pub struct SparseTable {
    data: Vec<i64>,
    log_table: Vec<usize>,
    table: Vec<Vec<usize>>,
}
#[snippet("SparseTable")]
impl SparseTable {
    pub fn new(data: Vec<i64>) -> Self {
        let n = data.len();
        let mut log_table = vec![0; n+1]; // log(k) (0<=k<=n)
        for i in 2..n+1 {
            log_table[i] = log_table[i >> 1] + 1;
        }
        // dbg!(&log_table);

        let mut table = vec![vec![n; n]; log_table[n]+1];
        // 2^k
        for i in 0..n {
            table[0][i] = i;
        }

        for k in 1..table.len() {
            // dbg!(&table);
            let half_jmp = 1 << (k-1);
            for i in 0..n {
                let first = table[k-1][i];
                table[k][i] = first;
                
                if i+half_jmp < n {
                    let second = table[k-1][i+half_jmp];
                    assert!(first < n);
                    assert!(second < n);
                    if data[first] <= data[second] {
                        table[k][i] = first;
                    } else {
                        table[k][i] = second;
                    }
                }
            }
        }
        // dbg!(&table);

        Self {
            data,
            log_table,
            table,
        }
    }

    /// [a, b)
    pub fn query(&self, a: usize, b: usize) -> usize {
        let d = b - a;
        let k = self.log_table[d];
        let first = self.table[k][a];
        let second = self.table[k][b-(1<<k)];
        if self.data[first] <= self.data[second] {
            first
        } else {
            second
        }
    }
}


#[test]
fn test_sparse_table() {
    let data = vec![2,5,3,9,8,2,10,1,7,2,1,6];
    let st = SparseTable::new(data);
    assert_eq!(st.query(0,4),0);
    assert_eq!(st.query(3,9),7);
    assert_eq!(st.query(9,12),10);
}