pub fn factors(n: usize) -> Vec<(usize, usize)> {
    vec![]
}

pub fn eratosthenes(max_n: usize) -> Vec<usize> {
    vec![]
}

// compute the maximum factor for each number
// e.g 5 for 60 (2x2x3x5)
#[snippet = "factor_table"]
#[allow(dead_code)]
pub fn factor_table(max_n: usize) -> Vec<usize> {
    let mut res = vec![0; max_n + 1];
    // res[1] = 1;
    for i in 2..max_n + 1 {
        if res[i] == 0 {
            let mut j = i;
            while j <= max_n {
                res[j] = i;
                j += i;
            }
        }
    }

    res
}

#[test]
fn test_factor_table() {
    let n = 1000;
    let table = factor_table(n);
    for i in 2..n + 1 {
        assert_eq!(i % table[i], 0);
    }
}