use std::{fs, path::PathBuf, str::FromStr};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use jmaxml::Report;

fn main() {
    let out_dir = PathBuf::from_str("sample_json").unwrap();
    fs::create_dir_all(&out_dir).unwrap();

    glob::glob("sample_xmls/*.xml")
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|path| {
            // Deserialize from XML
            println!("make_json: {path:?}");
            let content = fs::read_to_string(path).unwrap();
            let report = Report::from_str(&content).unwrap();

            // Serialize to JSON
            let json = serde_json::to_string_pretty(&report).unwrap();
            let out_path = out_dir
                .join(path.file_stem().unwrap())
                .with_extension("json");
            fs::write(&out_path, json).unwrap();
        });
}
