#![cargo_snippet::snippet("template")]

/// https://github.com/hatoo/competitive-rust-snippets

#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[macro_export]
macro_rules! chmax {
    ($x:expr, $($v:expr),+) => {
        $(
            $x = std::cmp::max($x,$v);
        )+
    };
}

#[macro_export]
macro_rules! chmin {
    ($x:expr, $($v:expr),+) => {
        $(
            $x = std::cmp::min($x,$v);
        )+
    };
}

#[macro_export]
macro_rules! max {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        std::cmp::max($x, max!( $($xs),+ ))
    };
}

#[macro_export]
macro_rules! min {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        std::cmp::min($x, min!( $($xs),+ ))
    };
}

#[macro_export]
macro_rules! dvec {
    ($t:expr ; $len:expr) => {
        vec![$t; $len]
    };
 
    ($t:expr ; $len:expr, $($rest:expr),*) => {
        vec![dvec!($t; $($rest),*); $len]
    };
}

#[macro_export]
macro_rules! cfor {
    // for (; ...; ...) { ... }
    (; $($rest: tt)*) => {
        cfor!((); $($rest)*)
    };
    // for ($init; ; ...) { ... }
    ($($init: stmt),+; ; $($rest: tt)*) => {
        // avoid the `while true` lint
        cfor!($($init),+; !false; $($rest)*)
    };

    // for ($init; $cond; ) { ... }
    ($($init: stmt),+; $cond: expr; ; $body: block) => {
        cfor!{$($init),+; $cond; (); $body}
    };

    // for ($init; $cond; $step) { $body }
    ($($init: stmt),+; $cond: expr; $($step: expr),+; $body: block) => {
        {
            $($init;)+
            while $cond {
                let mut _first = true;
                let mut _continue = false;
                // this loop runs once, allowing us to use `break` and
                // `continue` as `goto` to skip forward to the
                // condition.
                //
                // the booleans above are very transparent to the
                // optimiser, since they are modified exactly once,
                // with nice control flow, and this this optimises to
                // be similar to C for loop.
                loop {
                    // if we *don't* hit this, there was a `break` in
                    // the body (otherwise the loop fell-through or
                    // was `continue`d.)
                    if !_first { _continue = true; break }
                    _first = false;

                    $body
                }
                if !_continue {
                    // the `if` wasn't hit, so we should propagate the
                    // `break`.
                    break
                }

                $($step;)+
            }
        }
    };
}

/// main

#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};

// ref: tanakh <https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8>
// diff: using Parser
#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut parser = Parser::from_str($s);
        input_inner!{parser, $($r)*}
    };
    (parser = $parser:ident, $($r:tt)*) => {
        input_inner!{$parser, $($r)*}
    };
    (new_stdin_parser = $parser:ident, $($r:tt)*) => {
        let stdin = std::io::stdin();
        let reader = std::io::BufReader::new(stdin.lock());
        let mut $parser = Parser::new(reader);
        input_inner!{$parser, $($r)*}
    };
    ($($r:tt)*) => {
        input!{new_stdin_parser = parser, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($parser:ident) => {};
    ($parser:ident, ) => {};
    ($parser:ident, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($parser, $t);
        input_inner!{$parser $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($parser:ident, ( $($t:tt),* )) => {
        ( $(read_value!($parser, $t)),* )
    };
    ($parser:ident, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($parser, $t)).collect::<Vec<_>>()
    };
    ($parser:ident, chars) => {
        read_value!($parser, String).chars().collect::<Vec<char>>()
    };
    ($parser:ident, usize1) => {
        read_value!($parser, usize) - 1
    };
    ($parser:ident, $t:ty) => {
        $parser.next::<$t>().expect("Parse error")
    };
}

use std::io::BufRead;
use std::io;
use std::str;

// ref: tatsuya6502 <https://qiita.com/tatsuya6502/items/cd448486f7ef7b5b8c7e>
// ref: wariuni <https://qiita.com/tatsuya6502/items/cd448486f7ef7b5b8c7e#comment-7040a5ae96305e884eb9>
// diff: using std::io::BufRead::fill_buf()
pub struct Parser<R> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
}

impl Parser<io::Empty> {
    pub fn from_str(s: &str) -> Parser<io::Empty> {
        Parser {
            reader: io::empty(),
            buf: s.as_bytes().to_vec(),
            pos: 0,
        }
    }
}

impl<R:BufRead> Parser<R> {
    pub fn new(reader: R) -> Parser<R> {
        Parser {
            reader: reader,
            buf: vec![],
            pos: 0,
        }
    }
    pub fn update_buf(&mut self) {
        self.buf.clear();
        self.pos = 0;
        loop {
            let (len,complete) = {
                let buf2 = self.reader.fill_buf().unwrap();
                self.buf.extend_from_slice(buf2);
                let len = buf2.len();
                // diff: care when there is no empty space or line at the end
                if len == 0 {
                     break;
                }
                (len, buf2[len-1] <= 0x20)
            };
            self.reader.consume(len);
            if complete {
                break;
            }
        }
    }
    pub fn next<T:str::FromStr>(&mut self) -> Result<T, T::Err> {
        loop {
            let mut begin = self.pos;
            while begin < self.buf.len() && (self.buf[begin] <= 0x20) {
                begin += 1;
            }
            let mut end = begin;
            while end < self.buf.len() && (self.buf[end] > 0x20) {
                end += 1;
            }
            if begin != self.buf.len() {
                self.pos = end;
                return str::from_utf8(&self.buf[begin..end]).unwrap().parse::<T>();
            }
            else {
                self.update_buf();
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules ! debug { ( $ ( $ a : expr ) ,* ) => { eprintln ! ( concat ! ( $ ( stringify ! ( $ a ) , " = {:?}, " ) ,* ) , $ ( $ a ) ,* ) ; } }
#[doc = " https://github.com/hatoo/competitive-rust-snippets"]
const BIG_STACK_SIZE: bool = true;
#[allow(dead_code)]
fn main() {
    use std::thread;
    if BIG_STACK_SIZE {
        thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .name("solve".into())
            .spawn(solve)
            .unwrap()
            .join()
            .unwrap();
    } else {
        solve();
    }
}

fn solve() {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
}