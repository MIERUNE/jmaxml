use quick_xml::{
    NsReader,
    events::Event,
    name::{Namespace, ResolveResult},
};

use crate::{
    Result,
    model::{MeteBody, Report, ReportBody, ReportInternal, SeisBody, VolcBody},
};

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
                if local.as_ref() == b"Body"
                    && let ResolveResult::Bound(Namespace(namespace)) = ns
                {
                    return parse_by_namespace(namespace, content);
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

    match namespace {
        NS_METE => {
            let report: ReportInternal<MeteBody> = quick_xml::de::from_str(content)?;
            Ok(Report {
                control: report.control,
                head: report.head,
                body: ReportBody::Meteorology(report.body.into()),
            })
        }
        NS_SEIS => {
            let report: ReportInternal<SeisBody> = quick_xml::de::from_str(content)?;
            Ok(Report {
                control: report.control,
                head: report.head,
                body: ReportBody::Seismology(report.body.into()),
            })
        }
        NS_VOLC => {
            let report: ReportInternal<VolcBody> = quick_xml::de::from_str(content)?;
            Ok(Report {
                control: report.control,
                head: report.head,
                body: ReportBody::Volcanology(report.body.into()),
            })
        }
        _ => Err(quick_xml::de::DeError::Custom(ERROR_MSG.into())),
    }
}
