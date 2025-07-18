#![doc = include_str!("../README.md")]

// contains all function API's
mod accent;
mod sentence;

// export
pub use crate::accent::*;
pub use crate::sentence::*;
