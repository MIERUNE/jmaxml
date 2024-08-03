use quick_xml::events::Event;
use quick_xml::name::{Namespace, ResolveResult};
use quick_xml::reader::NsReader;

use crate::model::{Body, MeteBody, Report, SeisBody, VolcBody};

const NS_SEIS: &[u8] = b"http://xml.kishou.go.jp/jmaxml1/body/seismology1/";
const NS_VOLC: &[u8] = b"http://xml.kishou.go.jp/jmaxml1/body/volcanology1/";
const NS_METE: &[u8] = b"http://xml.kishou.go.jp/jmaxml1/body/meteorology1/";

#[derive(Debug)]
pub enum BodyKind {
    Meteorology,
    Seismology,
    Volcanology,
}

impl BodyKind {
    pub fn from_namespace(namespace: &[u8]) -> Option<Self> {
        match namespace {
            NS_METE => Some(Self::Meteorology),
            NS_SEIS => Some(Self::Seismology),
            NS_VOLC => Some(Self::Volcanology),
            _ => None,
        }
    }

    pub fn from_content(content: &str) -> Option<BodyKind> {
        let mut reader = NsReader::from_str(content);
        loop {
            match reader.read_event() {
                Ok(event) => match event {
                    Event::Start(e) | Event::Empty(e) => {
                        let (ns, local) = reader.resolve_element(e.name());
                        if local.as_ref() == b"Body" {
                            if let ResolveResult::Bound(Namespace(namespace)) = ns {
                                return Self::from_namespace(namespace);
                            }
                        }
                    }
                    Event::Eof => {
                        break;
                    }
                    _ => {}
                },
                Err(_) => return None,
            }
        }
        None
    }
}

pub fn from_str(content: &str) -> Result<Report<Body>, quick_xml::de::DeError> {
    let body_kind = BodyKind::from_content(content);
    match body_kind {
        Some(BodyKind::Meteorology) => {
            let report: Report<MeteBody> = quick_xml::de::from_str(content)?;
            Ok(Report::<Body> {
                control: report.control,
                head: report.head,
                body: Body::Meteorology(report.body),
            })
        }
        Some(BodyKind::Seismology) => {
            let report: Report<SeisBody> = quick_xml::de::from_str(content)?;
            Ok(Report::<Body> {
                control: report.control,
                head: report.head,
                body: Body::Seismology(report.body),
            })
        }
        Some(BodyKind::Volcanology) => {
            let report: Report<VolcBody> = quick_xml::de::from_str(content)?;
            Ok(Report::<Body> {
                control: report.control,
                head: report.head,
                body: Body::Volcanology(report.body),
            })
        }
        None => Err(quick_xml::de::DeError::Custom(
            "failed to determine body kind".to_string(),
        )),
    }
}
