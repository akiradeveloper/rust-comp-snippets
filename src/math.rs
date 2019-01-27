fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(24, 4), 4);
    assert_eq!(gcd(9, 6), 3);
    assert_eq!(gcd(8, 3), 1);
}

fn do_extgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) -> i32 {
    let mut d = a;
    if b != 0 {
        d = do_extgcd(b, a % b, y, x);
        *y -= (a / b) * *x;
    } else {
        *x = 1; *y = 0;
    }
    d
}
// ax + by = gcd(a,b)
fn extgcd(a: i32, b: i32) -> (i32, i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let gcd_a_b = do_extgcd(a, b, &mut x, &mut y);
    (x, y, gcd_a_b)
}

#[test]
fn test_extgcd() {
    assert_eq!(extgcd(4,11), (3,-1,1));
}