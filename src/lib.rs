// Declare our library as `no-std` unless the user opts in for std
// features.
#![cfg_attr(not(feature = "std"), no_std)]

// We always pull in `std` during tests, because it's just easier to
// write tests when you can assume you're on a capable platform
#[cfg(all(test, not(feature = "std")))]
compile_error!("Can only run tests with `--features std`");

// Pull in a lightweight no-std "compatibility layer" in order to
// avoid having to trouble ourselves with supporting both std and
// core.
extern crate no_std_compat as std;

// We often also have to import the prelude to obtain all the standard
// functions, something that is normally automatically injected by the
// compiler. On std this isn't needed, but usually it can be imported
// unconditionally either way.

// use std::prelude::v1::*;

use std::fmt;

pub fn write(mut w: impl fmt::Write, v: impl fmt::Debug) -> fmt::Result {
    write!(w, "{:?}", v)
}

// When we are building with memory allocations or the standard
// library (which includes memory allocations), there may be
// additional features our library supports.
#[cfg(any(feature = "alloc", feature = "std"))]
mod alloc_support {
    use crate::write;
    use std::{
        string::String,
        fmt,
    };

    pub fn to_string(v: impl fmt::Debug) -> String {
        let mut buf = String::new();
        write(&mut buf, v).expect("failed to write to string");

        buf
    }

    #[cfg(all(test, feature = "std"))]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            assert_eq!("42", to_string(42));
        }
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
pub use self::alloc_support::*;

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    // We also don't need to use the prelude when compiling with std,
    // which we are when we're doing tests.

    // use std::prelude::v1::*;

    #[test]
    fn it_works() {
        let mut buf = String::new();
        write(&mut buf, 42).expect("failed to write to string");

        assert_eq!("42", buf);
    }
}
