#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Num {
    x: i64
}
macro_rules! impl_into_for_num {
    ($($t:ty)*) => {
        $(
            impl Into<$t> for Num {
                fn into(self) -> $t {
                    self.x as $t
                }
            }
        )*
    }
}
impl_into_for_num! {
    usize i64 u64 i32 u32
}
macro_rules! impl_from_for_num {
    ($($t:ty)*) => {
        $(
            impl From<$t> for Num {
                fn from(x: $t) -> Self {
                    Num { x: x as i64 }
                }
            }
        )*
    }
}
impl_from_for_num! {
    usize i64 u64 i32 u32
} 
 
#[test]
fn test_num() {
    let a: Num = 1.into();
    let b: usize = a.into();
    let c: Num = b.into();
    let d: i32 = c.into();
    assert!(a==c);
    assert!(a<2.into());
}