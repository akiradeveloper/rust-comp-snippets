#[snippet = "modint"]
mod modint {
    use std::ops::*;
    pub trait Mod: Copy { fn m() -> i64; }
    #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt<M> { pub x: i64, phantom: ::std::marker::PhantomData<M> }
    impl<M: Mod> ModInt<M> {
        // x >= 0
        pub fn new(x: i64) -> Self { ModInt::new_internal(x % M::m()) }
        pub fn unwrap(&self) -> i64 { self.x % M::m() }
        fn new_internal(x: i64) -> Self {
            ModInt { x: x, phantom: ::std::marker::PhantomData }
        }
        pub fn pow(self, mut e: i64) -> Self {
            debug_assert!(e >= 0);
            let mut sum = ModInt::new_internal(1);
            let mut cur = self;
            while e > 0 {
                if e % 2 != 0 { sum *= cur; }
                cur *= cur;
                e /= 2;
            }
            sum
        }
        #[allow(dead_code)]
        pub fn inv(self) -> Self { self.pow(M::m() - 2) }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Add<T> for ModInt<M> {
        type Output = Self;
        fn add(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x + other.x;
            if sum >= M::m() { sum -= M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
        type Output = Self;
        fn sub(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x - other.x;
            if sum < 0 { sum += M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
        type Output = Self;
        fn mul(self, other: T) -> Self { ModInt::new(self.x * other.into().x % M::m()) }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Div<T> for ModInt<M> {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output {
            self * rhs.into().inv()
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> DivAssign<T> for ModInt<M> {
        fn div_assign(&mut self, rhs: T) {
            *self = *self / rhs;
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> AddAssign<T> for ModInt<M> {
        fn add_assign(&mut self, other: T) { *self = *self + other; }
    }
    impl<M: Mod, T: Into<ModInt<M>>> SubAssign<T> for ModInt<M> {
        fn sub_assign(&mut self, other: T) { *self = *self - other; }
    }
    impl<M: Mod, T: Into<ModInt<M>>> MulAssign<T> for ModInt<M> {
        fn mul_assign(&mut self, other: T) { *self = *self * other; }
    }
    impl<M: Mod> Neg for ModInt<M> {
        type Output = Self;
        fn neg(self) -> Self { ModInt::new(0) - self }
    }
    impl<M> ::std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M> ::std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(x: i64) -> Self { Self::new(x) }
    }
}
#[snippet = "modint"]
macro_rules! define_mod {
    ($struct_name: ident, $modulo: expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $struct_name {}
        impl modint::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
// default modint for library compile
#[snippet = "modint"]
define_mod!(P, 1_000_000_007);
#[snippet = "modint"]
pub type Mod = modint::ModInt<P>;

#[test]
fn test_modint() {
    const mo: i64 = 1_000_000_007;
    define_mod!(P, mo);
    let a = 1000000000;
    let b = 2000000000;
    let x: Mod = a.into();
    let y: Mod = b.into();
    assert_eq!((x*y).unwrap(), ((a%mo)*(b%mo))%mo);
}

// tanakh version (very simple but enough)
pub mod modular {
    const M: i64 = 1_000_000_007;
 
    #[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
    pub struct Mod(i64);
 
    impl ::std::fmt::Display for Mod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
 
    impl Mod {
        pub fn new(v: i64) -> Mod {
            Mod(v % M)
        }
 
        pub fn pow(self, mut r: i64) -> Mod {
            let mut k = self;
            let mut ret = 1.into();
 
            while r > 0 {
                if r % 2 != 0 {
                    ret = ret * k;
                }
                r /= 2;
                k = k * k;
            }
 
            ret
        }
 
        // This requires M is prime
        pub fn recip(self) -> Mod {
            self.pow(M - 2)
        }
    }
 
    use std::ops::*;
 
    impl<T: Into<Mod>> Add<T> for Mod {
        type Output = Mod;
        fn add(self, rhs: T) -> Self::Output {
            Mod::new(self.0 + rhs.into().0)
        }
    }
    impl<T: Into<Mod>> AddAssign<T> for Mod {
        fn add_assign(&mut self, rhs: T) {
            *self = *self + rhs;
        }
    }
 
    impl<T: Into<Mod>> Sub<T> for Mod {
        type Output = Mod;
        fn sub(self, rhs: T) -> Self::Output {
            Mod::new(self.0 - rhs.into().0 + M)
        }
    }
    impl<T: Into<Mod>> SubAssign<T> for Mod {
        fn sub_assign(&mut self, rhs: T) {
            *self = *self - rhs;
        }
    }
 
    impl<T: Into<Mod>> Mul<T> for Mod {
        type Output = Mod;
        fn mul(self, rhs: T) -> Self::Output {
            Mod::new(self.0 * rhs.into().0)
        }
    }
    impl<T: Into<Mod>> MulAssign<T> for Mod {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs;
        }
    }
 
    impl<T: Into<Mod>> Div<T> for Mod {
        type Output = Mod;
        fn div(self, rhs: T) -> Self::Output {
            self * rhs.into().recip()
        }
    }
    impl<T: Into<Mod>> DivAssign<T> for Mod {
        fn div_assign(&mut self, rhs: T) {
            *self = *self / rhs;
        }
    }
 
    impl Neg for Mod {
        type Output = Mod;
        fn neg(self) -> Self::Output {
            Mod(0) - self
        }
    }
 
    impl<T: ::std::convert::Into<i64>> ::std::convert::From<T> for Mod {
        fn from(v: T) -> Self {
            Mod::new(v.into())
        }
    }
}