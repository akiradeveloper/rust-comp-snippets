use cargo_snippet::snippet;

// https://shino16.github.io/blog/post/algo/%E3%82%AA%E3%83%BC%E3%83%88%E3%83%9E%E3%83%88%E3%83%B3/

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;

#[snippet("Dfa")]
trait Dfa {
    type Alphabet;
    type State;
    fn init(&self) -> Self::State;
    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State;
    fn accept(&self, s: Self::State) -> bool;
}

#[snippet("Dfa")]
pub fn count<X: Dfa>(dfa: &X, n: usize, alphabet: &[X::Alphabet], modulo: u32) -> u32
where
    X::Alphabet: Copy,
    X::State: Eq + Hash + Copy,
{
    let mut dp = HashMap::new();
    let mut dp2 = HashMap::new();
    dp.insert(dfa.init(), 1_u64);
    for i in 0..n {
        for (s, k) in dp.drain() {
            let k = k % modulo as u64;
            for &a in alphabet {
                let s1 = dfa.next(s, a, i);
                *dp2.entry(s1).or_insert(0) += k;
            }
        }
        std::mem::swap(&mut dp, &mut dp2);
    }
    let mut sum = 0;
    for (s, k) in dp {
        if dfa.accept(s) {
            sum += k;
            sum %= modulo as u64
        }
    }
    sum as u32
}

#[snippet("Dfa")]
struct Leq<'a>(&'a [u8]);
#[snippet("Dfa")]
impl Dfa for Leq<'_> {
    type Alphabet = u8;
    type State = Ordering;
    fn init(&self) -> Self::State {
        Ordering::Equal
    }
    // assumes i moves from 0 to self.0.len() - 1
    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State {
        s.then(a.cmp(&self.0[i]))
    }
    fn accept(&self, s: Self::State) -> bool {
        s != Ordering::Greater
    }
}

#[snippet("Dfa")]
struct And<X, Y>(X, Y);
#[snippet("Dfa")]
impl<X: Dfa<Alphabet = A>, Y: Dfa<Alphabet = A>, A: Copy> Dfa for And<X, Y> {
    type Alphabet = A;
    type State = (X::State, Y::State);
    fn init(&self) -> Self::State {
        (self.0.init(), self.1.init())
    }
    fn next(&self, (s0, s1): Self::State, a: Self::Alphabet, i: usize) -> Self::State {
        (self.0.next(s0, a, i), self.1.next(s1, a, i))
    }
    fn accept(&self, (s0, s1): Self::State) -> bool {
        self.0.accept(s0) && self.1.accept(s1)
    }
}

#[snippet("Dfa")]
struct Not<X>(X);
#[snippet("Dfa")]
impl<X: Dfa> Dfa for Not<X> {
    type Alphabet = X::Alphabet;
    type State = X::State;
    fn init(&self) -> Self::State {
        self.0.init()
    }
    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State {
        self.0.next(s, a, i)
    }
    fn accept(&self, s: Self::State) -> bool {
        !self.0.accept(s)
    }
}

#[snippet("Dfa")]
struct Prod<X, Y>(X, Y);
#[snippet("Dfa")]
impl<X: Dfa, Y: Dfa> Dfa for Prod<X, Y> {
    type Alphabet = (X::Alphabet, Y::Alphabet);
    type State = (X::State, Y::State);
    fn init(&self) -> Self::State {
        (self.0.init(), self.1.init())
    }
    fn next(&self, (s0, s1): Self::State, (a0, a1): Self::Alphabet, i: usize) -> Self::State {
        (self.0.next(s0, a0, i), self.1.next(s1, a1, i))
    }
    fn accept(&self, (s0, s1): Self::State) -> bool {
        self.0.accept(s0) && self.1.accept(s1)
    }
}