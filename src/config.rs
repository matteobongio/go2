use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn get_config() -> PathBuf {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("goto/");
    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }
    config_dir.push("config.txt");
    if !config_dir.exists() {
        let _ = fs::File::create(&config_dir);
        // return Vec::new();
    }
    config_dir
}

pub fn add_path(path: String) {
    let config = get_config();
    let mut file = fs::File::options().append(true).open(config).unwrap();
    file.write_all(path.as_bytes()).unwrap();
}

pub fn get_paths() -> Vec<String> {
    let file = fs::File::open(get_config()).unwrap();
    BufReader::new(file).lines().map(|a| a.unwrap()).collect()
}
