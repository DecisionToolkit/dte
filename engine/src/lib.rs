//! # Decision table editor engine

// #![warn(missing_docs)]
// #![warn(rustdoc::broken_intra_doc_links)]
// #![warn(rustdoc::missing_crate_level_docs)]

mod controller;
mod cursor;
mod model;
mod region;

pub use controller::Controller;
pub use region::Region;
