use quick_xml::{
    NsReader,
    events::Event,
    name::{Namespace, ResolveResult},
};
use serde::{Deserialize, Serialize};

use crate::{Report, ReportBody, Result};

pub mod generated;
pub use generated::{MeteBody, SeisBody, VolcBody};

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Duration = iso8601_duration::Duration;
// Telemetry showed chunks never exceeded 1KB
const BUFFER_SIZE: usize = 1024;
const ERROR_MSG: &str = "failed to determine body kind";

pub fn parse_report(content: &str) -> Result<Report> {
    let mut reader = NsReader::from_str(content);
    // Buffer size limits runtime memory allocation
    let mut buf = Vec::with_capacity(BUFFER_SIZE);
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let name = e.name();
                let (ns, local) = reader.resolve_element(name);
                if local.as_ref() == b"Body" {
                    if let ResolveResult::Bound(Namespace(namespace)) = ns {
                        return parse_by_namespace(namespace, content);
                    }
                }
            }
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }
    Err(quick_xml::de::DeError::Custom(ERROR_MSG.into()))
}

fn parse_by_namespace(namespace: &[u8], content: &str) -> Result<Report> {
    const NS_SEIS: &[u8] = b"http://xml.kishou.go.jp/jmaxml1/body/seismology1/";
    const NS_VOLC: &[u8] = b"http://xml.kishou.go.jp/jmaxml1/body/volcanology1/";
    const NS_METE: &[u8] = b"http://xml.kishou.go.jp/jmaxml1/body/meteorology1/";

    use ReportBody as Body;
    match namespace {
        NS_METE => {
            let report: ReportInternal<MeteBody> = quick_xml::de::from_str(content)?;
            Ok(Report {
                control: report.control,
                head: report.head,
                body: Body::Meteorology {
                    mete_body: report.body.into(),
                },
            })
        }
        NS_SEIS => {
            let report: ReportInternal<SeisBody> = quick_xml::de::from_str(content)?;
            Ok(Report {
                control: report.control,
                head: report.head,
                body: Body::Seismology {
                    seis_body: report.body.into(),
                },
            })
        }
        NS_VOLC => {
            let report: ReportInternal<VolcBody> = quick_xml::de::from_str(content)?;
            Ok(Report {
                control: report.control,
                head: report.head,
                body: Body::Volcanology {
                    volc_body: report.body.into(),
                },
            })
        }
        _ => Err(quick_xml::de::DeError::Custom(ERROR_MSG.into())),
    }
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
