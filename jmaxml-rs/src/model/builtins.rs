use super::generated;
use serde::{Deserialize, Serialize};

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Duration = iso8601_duration::Duration;

use super::generated::{MeteBody, SeisBody, VolcBody};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    /// 伝送情報
    pub control: generated::Control,

    /// ヘッダー部
    pub head: generated::IbHead,

    /// 気象関連のボディー部
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mete_body: Option<Box<MeteBody>>,

    /// 地震関連のボディー部
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seis_body: Option<Box<SeisBody>>,

    /// 火山関連のボディー部
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volc_body: Option<Box<VolcBody>>,
}

/// Internal representation of a report
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

impl From<Option<DateTime>> for NullableDateTime {
    fn from(value: Option<DateTime>) -> Self {
        NullableDateTime { value }
    }
}

impl From<DateTime> for NullableDateTime {
    fn from(value: DateTime) -> Self {
        NullableDateTime { value: Some(value) }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(into = "Vec<String>")]
pub struct StringList {
    #[serde(rename(deserialize = "$text"))]
    values: Vec<String>,
}

impl From<Vec<String>> for StringList {
    fn from(values: Vec<String>) -> Self {
        StringList { values }
    }
}

impl From<StringList> for Vec<String> {
    fn from(val: StringList) -> Self {
        val.values
    }
}
