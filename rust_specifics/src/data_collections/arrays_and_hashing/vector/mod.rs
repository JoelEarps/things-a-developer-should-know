//! Custom Vec implementation split into:
//! - raw_vec: allocation/deallocation (buffer ownership)
//! - custom_vec: logical operations (len, push, pop, insert, remove, Deref, IntoIter)

pub mod raw_vec;
pub mod custom_vec;

pub use custom_vec::{CustomVec, Drain, IntoIter};
pub use raw_vec::RawVec;
