use std::fs;

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

    println!("{model:#?}");
    let mut backend = RustBackend::new(model);

    let result = backend.generate(&RustBackendConfig {}).unwrap();
    //println!("{result:#?}");
    for File { relative_path, content } in result {
        let relative_path = relative_path.join("/");
        println!("\n\n---------- File {relative_path}--------\n{content}");
    }

}
