[![Crates.io](https://img.shields.io/crates/v/timeit.svg)](https://crates.io/crates/timeit)

# Timeit for Rust

This crate provides macros that make it easy to benchmark blocks of code. It is
inspired and named after [timeit](https://docs.python.org/3/library/timeit.html) from Python.

## Examples

```rust
#[macro_use]
extern crate timeit;

fn main() {
    timeit!({
        let mut x: Vec<u64> = Vec::new();
        for i in 0..1000 {
            x.push(i);
        }
    });
}
```

This will output something like:

```text
10000 loops: 2.4843 µs
```

It will determine the number of loops automatically. To run a specified number of loops and
save the elapsed time to a variable, use the `timeit_loops!` macro:

```rust
let sec = timeit_loops!(100, {
    let mut x: Vec<u64> = Vec::new();
    for i in 0..1000 {
        x.push(i);
    }
});
```
