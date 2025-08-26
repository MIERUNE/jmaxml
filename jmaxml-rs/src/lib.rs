use serde::{Deserialize, Serialize};

mod parser;

pub use parser::generated::{Control, IbHead, MeteBody, SeisBody, VolcBody};
use parser::parse_report;

pub type Result<T> = std::result::Result<T, quick_xml::de::DeError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    /// 伝送情報
    pub control: Control,
    /// ヘッダー部
    pub head: IbHead,
    #[serde(flatten)]
    pub body: ReportBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReportBody {
    /// 気象関連のボディー部
    Meteorology {
        #[serde(alias = "MeteBody", rename = "meteBody")]
        mete_body: MeteBody,
    },
    /// 地震関連のボディー部
    Seismology {
        #[serde(alias = "SeisBody", rename = "seisBody")]
        seis_body: SeisBody,
    },
    /// 火山関連のボディー部
    Volcanology {
        #[serde(alias = "VolcBody", rename = "volcBody")]
        volc_body: VolcBody,
    },
}

impl Report {
    pub fn new(content: &str) -> Result<Self> {
        parse_report(content)
    }
}

#[deprecated(note = "Use Report::new")]
pub fn from_str(content: &str) -> Result<Report> {
    Report::new(content)
}
