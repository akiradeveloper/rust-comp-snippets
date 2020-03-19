### Parsing Input

```rust
fn solve() {
    input! {
       new_stdin_parser = parser,
       N: usize, D: usize, 
    }
    let mut xs = vec![];
    for i in 0..N {
        input! {
            parser = parser,
            x: [isize; D]
        }
        xs.push(x);
    }
}
```