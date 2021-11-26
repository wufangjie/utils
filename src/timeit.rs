//! A marco to do simple time profile.
//!
//! version 0.1.0
//! https://github.com/wufangjie/learn/blob/main/src/timeit.rs
//!
//! use case 1: timeit!(n, code);
//! use case 2: timeit!(code); (=== timeit!(1, code))
//! code can be blocks, or just simple expressions

// seems no need to support statement,
// +put block before expr, block belongs to expr?+
// recursive macro is ok
#[macro_export]
macro_rules! timeit {
    ($loops:expr, $code:expr) => {
        let timeit_n = $loops;
        let timeit_start = std::time::Instant::now();
        for _ in 0..timeit_n {
            $code;
        }
        let timeit_cost = timeit_start.elapsed();
        println!(
            "[{}:{}] ({} loops, {:?} per loop)\t{{ {} }}",
            file!(),
            line!(),
            timeit_n,
            timeit_cost / timeit_n,
            match stringify!($code) {
                s if s.contains(";") => "...",
                s => s,
            }
        );
    };
    ($code:expr) => {
        timeit!(1, $code);
    };
}
