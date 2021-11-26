//! A macro to debug a variable with its type.
//!
//! version 0.1.0
//! https://github.com/wufangjie/learn/blob/main/src/dbgt.rs
//!
//! usage: dbgt!(&anything);

// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
// NOTE: must be used for a debug purpose only:
pub fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

// NOTE: dbg!(var1, var2) is ok, but dbgt! cannot
#[macro_export]
macro_rules! dbgt {
    ($val:expr) => {
        match $val {
            tmp => {
                eprintln!(
                    "[{}:{}] ({}: {}) = {:#?}",
                    file!(),
                    line!(),
                    stringify!($val),
                    $crate::dbgt::type_of(tmp), // not $val, &tmp
                    &tmp
                );
                tmp
            }
        }
    };
}
