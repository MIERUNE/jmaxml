use super::generated;
use serde::{Deserialize, Serialize};

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Duration = iso8601_duration::Duration;

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
    #[serde(rename(serialize = "meteorology"))]
    Meteorology(generated::MeteBody),
    #[serde(rename(serialize = "seismology"))]
    Seismology(generated::SeisBody),
    #[serde(rename(serialize = "volcanology"))]
    Volcanology(generated::VolcBody),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(into = "Option<DateTime>")]
pub struct NullableDateTime {
    #[serde(rename(deserialize = "$text", serialize = "value"))]
    value: Option<DateTime>,
}

impl From<NullableDateTime> for Option<DateTime> {
    fn from(val: NullableDateTime) -> Self {
        val.value
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(into = "Vec<String>")]
pub struct StringList {
    #[serde(rename(deserialize = "$text", serialize = "values"))]
    values: Vec<String>,
}

impl From<StringList> for Vec<String> {
    fn from(val: StringList) -> Self {
        val.values
    }
}
