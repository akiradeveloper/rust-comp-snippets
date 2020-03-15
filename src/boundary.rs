#[snippet = "valid4"]
pub fn valid4u(h: usize, w: usize, ps: Vec<(Option<usize>, Option<usize>)>) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for (x, y) in ps {
        if x.is_none() || y.is_none() {
            continue;
        }
        let x = x.unwrap();
        let y = y.unwrap();
        if x >= h || y >= w {
            res.push((x,y));
        }
    }
    res
}

#[snippet = "incl"]
#[doc = "0..=n in old compilers"]
pub fn incl(n: usize) -> usize {
    n+1
}

#[snippet = "sub_or_max"]
#[doc = "max(x-y, z)"]
pub fn sub_or_max(x: usize, y: usize, z: usize) -> usize {
    // x-y > z
    if x > y+z {
        x-y
    } else {
        z
    }
}
#[test]
fn test_sub_or_max() {
    assert_eq!(sub_or_max(3, 2, 0), 1);
    assert_eq!(sub_or_max(3, 4, 0), 0);
}

#[snippet = "add_or_min"]
#[doc = "min(x+y, z)"]
pub fn add_or_min(x: usize, y: usize, z: usize) -> usize {
    // x+y < z
    if x+y < z {
        x+y
    } else {
        z
    }
}
#[test]
fn test_add_or_min() {
    assert_eq!(add_or_min(4, 6, 8),8);
    assert_eq!(add_or_min(4, 6, 12),10);
}