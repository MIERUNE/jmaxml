mod generated;

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Result, parser::parse_report};
pub use generated::*;

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Duration = iso8601_duration::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    /// 伝送情報
    pub control: Control,
    /// ヘッダー部
    pub head: IbHead,
    /// ボディー部
    pub body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Body {
    /// 気象関連のボディー部
    Meteorology(Box<MeteBody>),
    /// 地震関連のボディー部
    Seismology(Box<SeisBody>),
    /// 火山関連のボディー部
    Volcanology(Box<VolcBody>),
}

impl FromStr for Report {
    type Err = quick_xml::de::DeError;

    /// Tries to parse a Report from a string.
    fn from_str(content: &str) -> Result<Self> {
        parse_report(content)
    }
}

/// Internal representation of a report for `quick_xml::de`
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ReportInternal<B> {
    pub control: generated::Control,
    pub head: generated::IbHead,
    pub body: B,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(into = "Option<DateTime>", from = "Option<DateTime>")]
pub struct NullableDateTime(Option<DateTime>);

impl From<NullableDateTime> for Option<DateTime> {
    fn from(val: NullableDateTime) -> Self {
        val.0
    }
}

impl From<Option<DateTime>> for NullableDateTime {
    fn from(value: Option<DateTime>) -> Self {
        NullableDateTime(value)
    }
}

impl From<DateTime> for NullableDateTime {
    fn from(value: DateTime) -> Self {
        NullableDateTime(Some(value))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(into = "Vec<String>", from = "Vec<String>")]
pub struct StringList(Vec<String>);

impl From<Vec<String>> for StringList {
    fn from(values: Vec<String>) -> Self {
        let values = values
            .iter()
            .filter(|s| !s.trim().is_empty())
            .flat_map(|s| s.split_ascii_whitespace())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Self(values)
    }
}

impl From<StringList> for Vec<String> {
    fn from(val: StringList) -> Self {
        val.0
    }
}
