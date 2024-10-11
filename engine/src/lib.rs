//! # Decision table editor engine

// #![warn(missing_docs)]
// #![warn(rustdoc::broken_intra_doc_links)]
// #![warn(rustdoc::missing_crate_level_docs)]

mod controller;
mod model;
mod region;
mod updates;

pub use controller::*;
pub use model::*;
pub use region::*;
pub use updates::*;
