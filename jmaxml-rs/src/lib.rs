pub mod model;
mod parser;

pub use model::{Control, IbHead, MeteBody, Report, SeisBody, VolcBody};

pub type Result<T> = std::result::Result<T, quick_xml::de::DeError>;
