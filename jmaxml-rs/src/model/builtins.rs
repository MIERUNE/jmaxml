use super::generated;
use serde::{Deserialize, Serialize};

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Duration = iso8601_duration::Duration;

use super::generated::{MeteBody, SeisBody, VolcBody};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub control: generated::Control,
    pub head: generated::IbHead,

    /// Body for meteorological information (jmx_mete:Body)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mete_body: Option<Box<MeteBody>>,

    /// Body for seismological information (jmx_seis:Body)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seis_body: Option<Box<SeisBody>>,

    /// Body for volcanological information (jmx_volc:Body)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volc_body: Option<Box<VolcBody>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ReportInternal<B> {
    pub control: generated::Control,
    pub head: generated::IbHead,
    pub body: B,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(into = "Option<DateTime>")]
pub struct NullableDateTime {
    #[serde(rename(deserialize = "$text"))]
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
    #[serde(rename(deserialize = "$text"))]
    values: Vec<String>,
}

impl From<StringList> for Vec<String> {
    fn from(val: StringList) -> Self {
        val.values
    }
}
