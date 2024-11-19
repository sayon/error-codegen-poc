use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use serde_json;
use zksync_error_codegen::codegen::rust::config::RustBackendConfig;
use zksync_error_codegen::codegen::rust::RustBackend;
use zksync_error_codegen::codegen::{Backend as _, File};
use zksync_error_codegen::json::Config;
use zksync_error_codegen::model::validator::validate;
use zksync_error_codegen::model::Model;

fn main() {
    // Specify the file path
    let file_path = "example.json";
    // Read the entire file contents into a string
    let content = fs::read_to_string(file_path).unwrap();

    let config: Config = serde_json::from_str(&content).unwrap();

    //println!("{config:#?}");

    let model = Model::try_from(&config).unwrap();

    validate(&model).unwrap();

    //println!("{model:#?}");
    let mut backend = RustBackend::new(model);

    let result = backend.generate(&RustBackendConfig {}).unwrap();

    write_to_file("zksync-error/Cargo.toml",
        r#"
[package]
name = "zksync_error"
version = "0.1.0"
edition = "2021"
[lib]
[dependencies]
serde = "1.0.213"
serde_json = "1.0.132"
strum = "0.26.3"
strum_macros = "0.26.4"
typify = "0.2.0"
"#,
    );
    let _ = create_files_in_result_directory("zksync-error/src", result);
}

fn write_to_file(filename:&str, content: &str) -> std::io::Result<()> {
    let path = Path::new(filename);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn create_files_in_result_directory(result_dir: &str, files: Vec<File>) -> std::io::Result<()> {
    let result_dir = Path::new(result_dir);

    if result_dir.exists() {
        fs::remove_dir_all(&result_dir)?;
    }

    fs::create_dir(&result_dir)?;

    for file in files {
        let mut path = PathBuf::from(&result_dir);
        for part in &file.relative_path {
            path.push(part);
        }

        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        let mut output_file = std::fs::File::create(&path)?;
        output_file.write_all(file.content.as_bytes())?;
    }

    Ok(())
}
