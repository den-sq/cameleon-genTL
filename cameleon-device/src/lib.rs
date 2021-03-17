#![recursion_limit = "1024"]
#![allow(
    clippy::module_name_repetitions,
    clippy::clippy::similar_names,
    clippy::clippy::missing_errors_doc,
    clippy::cast_possible_truncation
)]

pub mod u3v;

mod pixel_format;

pub use pixel_format::PixelFormat;
