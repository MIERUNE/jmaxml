// mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = parseXml)]
pub fn parse_xml(content: &str) -> Result<JsValue, String> {
    let report = match jmaxml::from_str(content) {
        Ok(report) => report,
        Err(err) => return Err(err.to_string()),
    };
    serde_wasm_bindgen::to_value(&report).map_err(|err| err.to_string())
}
