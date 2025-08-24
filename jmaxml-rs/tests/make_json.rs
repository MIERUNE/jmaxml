use jmaxml::model::Report;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[test]
fn make_json() {
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    let xml_dir = PathBuf::from_str("../assets/sample_xmls").unwrap();
    let json_dir = PathBuf::from_str("../assets/sample_json").unwrap();

    glob::glob(xml_dir.join("*.xml").to_str().unwrap())
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|path| {
            println!("test_report_json_round_trip: {path:?}");
            let xml_content = fs::read_to_string(path).unwrap();
            let xml_report = jmaxml::from_str(&xml_content).unwrap();

            let json_path = json_dir
                .join(path.file_stem().unwrap())
                .with_extension("json");
            let json_content = fs::read_to_string(json_path).unwrap();
            let json_report: Report = serde_json::from_str(&json_content).unwrap();

            let xml_value: serde_json::Value = serde_json::to_value(&xml_report).unwrap();
            let json_value: serde_json::Value = serde_json::to_value(&json_report).unwrap();
            assert_eq!(xml_value, json_value, "Round-trip failed for {path:?}",);
        });
}
