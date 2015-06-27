//! This crate provides macros that make it easy to benchmark blocks of code. It is inspired and
//! named after timeit from Python.
//!
//! Example:
//!
//! ```
//! #[macro_use]
//! extern crate timeit;
//!
//! fn main() {
//!     timeit!({
//!         let mut x: Vec<u64> = Vec::new();
//!         for i in 0..1000 {
//!             x.push(i);
//!         }
//!     });
//! }
//! ```
//!
//! This will output something like:
//!
//! ```text
//! 10000 loops: 2.4843 µs
//! ```
//!
//! It will determine the number of loops automatically. To run a specified number of loops and
//! save the elapsed time to a variable, use the `timeit_loops!` macro:
//!
//! ```
//! let sec = timeit_loops!(100, {
//!     let mut x: Vec<u64> = Vec::new();
//!     for i in 0..1000 {
//!         x.push(i);
//!     }
//! });
//! ```
extern crate time;

#[allow(unused_imports)]
use std::num;
#[allow(unused_imports)]
use time::get_time;

#[macro_export]
/// Runs a block a specified number of times and returns the average time of execution.
macro_rules! timeit_loops {
    ($loops:expr, $code:block) => ({
        let mut times: Vec<f64> = Vec::new();
        let mut n = $loops;
        let mut i = 0;
        let start = get_time();
        for _ in 0..n {
            $code
        }
        let end = get_time();
        let sec = (end.sec - start.sec) as f64 +
                  (end.nsec - start.nsec) as f64 / 1_000_000_000.0;

        sec / (n as f64)
    })
}

#[macro_export]
/// Runs a block several times and outputs the average time per loop. The number of loops is
/// determined automatically.
macro_rules! timeit {
    ($code:block) => ({
        let mut n = 1;
        let mut sec = 0.0;
        sec = timeit_loops!(n, $code);
        let total = sec;
        let mut again = true;

        let l = total.log10().ceil() as isize;

        if -5 <= l && l <= 0 {
            n = 10isize.pow((-l) as u32);
        } else {
            again = false;
        }

        if again {
            sec = timeit_loops!(n, $code);
        }

        let (mult, unit_str) = if sec > 1.0 {
            (1.0, "s")
        } else if sec > 0.001 {
            (0.001, "ms")
        } else if sec > 0.000_001 {
            (0.000_001, "µs")
        } else {
            (0.000_000_001, "ns")
        };

        println!("{} loops: {} {}", n, sec / mult, unit_str);
    })
}
