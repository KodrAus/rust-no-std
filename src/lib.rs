#![no_std]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
#[macro_use]
extern crate core as std;

use crate::std::fmt;

pub fn write(mut w: impl fmt::Write, v: impl fmt::Debug) -> fmt::Result {
    write!(w, "{:?}", v)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::std::prelude::v1::*;

    #[test]
    fn it_works() {
        let mut buf = String::new();
        write(&mut buf, 42).expect("failed to write to string");

        assert_eq!("42", buf);
    }
}
