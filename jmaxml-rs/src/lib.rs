pub mod model;
mod parser;

pub use model::{Body, Control, IbHead, Report};

pub type Result<T> = std::result::Result<T, quick_xml::de::DeError>;
