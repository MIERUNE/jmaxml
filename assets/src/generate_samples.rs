use std::path::PathBuf;
use std::str::FromStr;

use glob::glob;
use jmaxml::model::Report;

fn main() {
    use std::fs;

    println!("Starting...");

    let out_dir = PathBuf::from_str("sample_json").unwrap();
    fs::create_dir_all(&out_dir).unwrap();

    println!("Created directories");

    for path in glob("sample_xmls/*.xml").unwrap().filter_map(Result::ok) {
        // Deserialize from XML
        println!("{path:?}");
        let content = fs::read_to_string(&path).unwrap();
        let report = jmaxml::from_str(&content).unwrap();

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&report).unwrap();
        let out_path = out_dir
            .join(path.file_stem().unwrap())
            .with_extension("json");
        fs::write(&out_path, json).unwrap();

        // Deserialize from the serialized JSON
        let json_data = fs::read_to_string(&out_path).unwrap();
        serde_json::from_str::<Report>(&json_data).unwrap();
    }
}
