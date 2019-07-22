// Declaring our library as `no-std` unconditionally lets us be consistent
// in how we `use` items from `std` or `core`
#![no_std]

// We always pull in `std` during tests, because it's just easier
// to write tests when you can assume you're on a capable platform
#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

// When we're building for a no-std target, we pull in `core`, but alias
// it as `std` so the `use` statements are the same between `std` and `core`.
#[cfg(all(not(feature = "std"), not(test)))]
#[macro_use]
extern crate core as std;

use crate::std::fmt;

pub fn write(mut w: impl fmt::Write, v: impl fmt::Debug) -> fmt::Result {
    write!(w, "{:?}", v)
}

// When we are building with `std` there may be additional features
// our library supports.
#[cfg(feature = "std")]
mod std_support {
    use crate::{
        std::{
            string::String,
            fmt,
        },
        write,
    };

    pub fn to_string(v: impl fmt::Debug) -> String {
        let mut buf = String::new();
        write(&mut buf, v).expect("failed to write to string");

        buf
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            assert_eq!("42", to_string(42));
        }
    }
}

#[cfg(feature = "std")]
pub use self::std_support::*;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::std::string::String;

    #[test]
    fn it_works() {
        let mut buf = String::new();
        write(&mut buf, 42).expect("failed to write to string");

        assert_eq!("42", buf);
    }
}
