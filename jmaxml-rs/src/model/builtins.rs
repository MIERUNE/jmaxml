use super::generated;
use serde::{Deserialize, Serialize};

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Duration = iso8601_duration::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct NullableDateTime {
    #[serde(rename(deserialize = "$text", serialize = "values"))]
    values: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringList {
    #[serde(rename(deserialize = "$text", serialize = "values"))]
    values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report<B> {
    #[serde(rename(deserialize = "Control", serialize = "control"))]
    pub control: generated::Control,
    #[serde(rename(deserialize = "Head", serialize = "head"))]
    pub head: generated::IbHead,
    #[serde(rename(deserialize = "Body", serialize = "body"))]
    pub body: B,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Body {
    Meteorology(generated::MeteBody),
    Seismology(generated::SeisBody),
    Volcanology(generated::VolcBody),
}
