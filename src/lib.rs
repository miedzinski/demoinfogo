#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", feature(plugin))]

#[macro_use]
extern crate bitflags;
extern crate csgoproto;
#[macro_use]
extern crate matches;
extern crate protobuf;

pub mod cmd;
pub mod error;
pub mod file;
pub mod frame;
pub mod message;
pub mod tables;
mod read;
pub mod vectors;

pub use error::{DemoError, DemoResult};
pub use file::Demo;  // XXX

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::Path;

    use protobuf::CodedInputStream;

    use error::DemoResult;
    use read::FromStream;

    pub(crate) const TESTS_DIR: &str = "test-data";

    pub(crate) fn test_case(fname: &str) -> File {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(TESTS_DIR)
            .join(fname);
        File::open(path).unwrap()
    }

    pub(crate) fn read<T: FromStream>(fname: &str) -> DemoResult<T> {
        let mut file = test_case(fname);
        let mut stream = CodedInputStream::new(&mut file);
        T::from_stream(&mut stream)
    }
}
