use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn make_json() {
    use std::fs;

    let out_dir = PathBuf::from_str("../assets/sample_json").unwrap();
    fs::create_dir_all(&out_dir).unwrap();

    for path in glob::glob("../assets/sample_xmls/*.xml")
        .unwrap()
        .filter_map(Result::ok)
    {
        println!("{:?}", path);
        let content = fs::read_to_string(&path).unwrap();
        let report = jmaxml::from_str(&content).unwrap();

        let json = serde_json::to_string_pretty(&report).unwrap();
        let out_path = out_dir
            .join(path.file_stem().unwrap())
            .with_extension("json");
        fs::write(out_path, json).unwrap();
    }
}
