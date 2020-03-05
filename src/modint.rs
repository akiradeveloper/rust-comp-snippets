// #[snippet = "modint"]
// pub mod modular {
//     const M: i64 = 1_000_000_007;
//     #[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
//     pub struct Mod(pub i64);
//     impl ::std::fmt::Display for Mod {
//         fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
//             write!(f, "{}", self.0)
//         }
//     }
//     impl Mod {
//         pub fn new(v: i64) -> Mod {
//             Mod((v+M) % M)
//         }
//         pub fn pow(self, mut r: i64) -> Mod {
//             let mut k = self;
//             let mut ret = 1.into();
//             while r > 0 {
//                 if r % 2 != 0 {
//                     ret = ret * k;
//                 }
//                 r /= 2;
//                 k = k * k;
//             }
//             ret
//         }
//         // This requires M is prime
//         pub fn recip(self) -> Mod {
//             self.pow(M - 2)
//         }
//     }
//     use std::ops::*;
//     impl<T: Into<Mod>> Add<T> for Mod {
//         type Output = Mod;
//         fn add(self, rhs: T) -> Self::Output {
//             Mod::new(self.0 + rhs.into().0)
//         }
//     }
//     impl<T: Into<Mod>> AddAssign<T> for Mod {
//         fn add_assign(&mut self, rhs: T) {
//             *self = *self + rhs;
//         }
//     }
//     impl<T: Into<Mod>> Sub<T> for Mod {
//         type Output = Mod;
//         fn sub(self, rhs: T) -> Self::Output {
//             Mod::new(self.0 - rhs.into().0 + M)
//         }
//     }
//     impl<T: Into<Mod>> SubAssign<T> for Mod {
//         fn sub_assign(&mut self, rhs: T) {
//             *self = *self - rhs;
//         }
//     }
//     impl<T: Into<Mod>> Mul<T> for Mod {
//         type Output = Mod;
//         fn mul(self, rhs: T) -> Self::Output {
//             Mod::new(self.0 * rhs.into().0)
//         }
//     }
//     impl<T: Into<Mod>> MulAssign<T> for Mod {
//         fn mul_assign(&mut self, rhs: T) {
//             *self = *self * rhs;
//         }
//     }
//     impl<T: Into<Mod>> Div<T> for Mod {
//         type Output = Mod;
//         fn div(self, rhs: T) -> Self::Output {
//             self * rhs.into().recip()
//         }
//     }
//     impl<T: Into<Mod>> DivAssign<T> for Mod {
//         fn div_assign(&mut self, rhs: T) {
//             *self = *self / rhs;
//         }
//     }
//     impl Neg for Mod {
//         type Output = Mod;
//         fn neg(self) -> Self::Output {
//             Mod(0) - self
//         }
//     }
//     impl<T: ::std::convert::Into<i64>> ::std::convert::From<T> for Mod {
//         fn from(v: T) -> Self {
//             Mod::new(v.into())
//         }
//     }
// }

// #[snippet = "modint"]
// pub type Mod = modular::Mod;

#[snippet = "modint"]
pub trait ModI:
    Sized
    + PartialEq
    + Copy
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + std::default::Default
    + std::fmt::Display
    + std::fmt::Debug
{
    fn m() -> i64;
    fn new(x: i64) -> Self;
    fn pow(self, n: i64) -> Self;
    fn inv(&self) -> Self;
}
#[snippet = "modint"]
macro_rules! define_modint {
    ($n:ident,$m:expr) => {
        #[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
        pub struct $n(i64);

        #[allow(dead_code)]
        impl ModI for $n {
            fn m() -> i64 {
                $m
            }
            fn new(x: i64) -> $n {
                $n(x % $m)
            }

            fn pow(self, mut n: i64) -> $n {
                let mut ret = $n::new(1);
                let mut base = self;
                while n > 0 {
                    if n & 1 == 1 {
                        ret *= base;
                    }
                    base *= base;
                    n >>= 1;
                }
                ret
            }

            fn inv(&self) -> $n {
                self.pow($m - 2)
            }
        }

        impl std::default::Default for $n {
            fn default() -> $n {
                $n::new(0i64)
            }
        }

        impl std::convert::From<i64> for $n {
            fn from(x: i64) -> $n {
                $n::new(x)
            }
        }

        // Binary operator
        impl <T: Into<$n>> std::ops::Add<T> for $n {
            type Output = $n;
            fn add(self, rhs: T) -> Self::Output {
                $n::new(self.0 + rhs.into().0)
            }
        }

        impl <T: Into<$n>> std::ops::Sub<T> for $n {
            type Output = $n;
            fn sub(self, rhs: T) -> Self::Output {
                let rhs_mod: $n = rhs.into();
                if self.0 >= rhs_mod.0 {
                    $n::new(self.0 - rhs_mod.0)
                } else {
                    $n::new($m - rhs_mod.0 + self.0)
                }
            }
        }

        impl <T: Into<$n>> std::ops::Mul<T> for $n {
            type Output = $n;
            fn mul(self, rhs: T) -> Self::Output {
                $n::new(self.0 + rhs.into().0)
            }
        }

        impl <T: Into<$n>> std::ops::Div<T> for $n {
            type Output = $n;
            fn div(self, rhs: T) -> Self::Output {
                self * rhs.into().inv()
            }
        }

        // Assign
        impl <T: Into<$n>> std::ops::AddAssign<T> for $n {
            fn add_assign(&mut self, rhs: T) {
                *self = *self + rhs;
            }
        }

        impl <T: Into<$n>> std::ops::SubAssign<T> for $n {
            fn sub_assign(&mut self, rhs: T) {
                *self = *self - rhs;
            }
        }

        impl <T: Into<$n>> std::ops::MulAssign<T> for $n {
            fn mul_assign(&mut self, rhs: T) {
                *self = *self * rhs;
            }
        }

        impl <T: Into<$n>> std::ops::DivAssign<T> for $n {
            fn div_assign(&mut self, rhs: T) {
                *self = *self / rhs;
            }
        }

        impl std::fmt::Display for $n {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl std::fmt::Debug for $n {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
define_modint!(Mod, 1000000007);