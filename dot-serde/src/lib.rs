mod error;
mod ser;
mod util;

pub use error::{Error, Result};
pub use ser::{to_string, to_vec, to_writer, Serializer};
