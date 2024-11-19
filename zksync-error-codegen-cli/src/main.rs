pub mod arguments;
pub mod error;

use std::fs;
use std::io::Write as _;
use std::path::Path;

use arguments::Arguments;
use error::ProgramError;

use structopt::StructOpt as _;
use zksync_error_codegen::codegen::rust::config::RustBackendConfig;
use zksync_error_codegen::codegen::rust::RustBackend;
use zksync_error_codegen::codegen::{Backend as _, File};
use zksync_error_codegen::json::Config;
use zksync_error_codegen::model::validator::validate;
use zksync_error_codegen::model::Model;

fn main_inner(arguments: Arguments) -> Result<(), ProgramError> {
    let json_path = &arguments.definitions;
    let verbose = arguments.verbose;
    let backend_type = arguments.backend;

    let content = fs::read_to_string(json_path)?;

    let config: Config = serde_json::from_str(&content)?;

    if verbose {
        eprintln!("Read config from \"{json_path}\":\n{config:#?}");
        eprintln!("Building model...");
    }

    let model = Model::try_from(&config)?;
    if verbose {
        eprintln!("Model: {model:#?}");
        eprintln!("Model validation...");
    }
    validate(&model).unwrap();
    if verbose {
        eprintln!("Model validation successful.");
    }

    if verbose {
        eprintln!("Selected backend: {backend_type:?}. \nGenerating files...");
    }
    let result = match backend_type {
        arguments::Backend::Doc => todo!(),
        arguments::Backend::Rust => {
            let mut backend = RustBackend::new(model);
            backend.generate(&RustBackendConfig {})?
        }
    };

    if verbose {
        eprintln!("Generation successful. Files: ");
        for file in &result {
            eprintln!("- {}", file.relative_path.to_str().unwrap());
        }
        eprintln!("Writing files to disk...");
    }

    create_files_in_result_directory("zksync-error", result)?;
    if verbose {
        eprintln!("Writing successful.");
    }
    Ok(())
}

fn create_files_in_result_directory(result_dir: &str, files: Vec<File>) -> std::io::Result<()> {
    let result_dir = Path::new(result_dir);

    if result_dir.exists() {
        fs::remove_dir_all(result_dir)?;
    }

    fs::create_dir(result_dir)?;

    for file in files {
        let path = result_dir.join(file.relative_path);

        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        let mut output_file = std::fs::File::create(&path)?;
        output_file.write_all(file.content.as_bytes())?;
    }

    Ok(())
}

fn main() {
    let arguments = Arguments::from_args();

    if let Err(error) = main_inner(arguments) {
        eprintln!("{error:?}")
    }
}
