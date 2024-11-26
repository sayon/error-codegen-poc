pub mod arguments;
pub mod error;

use std::io::Write as _;
use std::path::Path;

use arguments::Arguments;
use error::ProgramError;

use structopt::StructOpt as _;
use zksync_error_codegen::codegen::file::File;
use zksync_error_codegen::codegen::html::config::HtmlBackendConfig;
use zksync_error_codegen::codegen::html::HtmlBackend;
use zksync_error_codegen::codegen::mdbook::config::MDBookBackendConfig;
use zksync_error_codegen::codegen::mdbook::MDBookBackend;
use zksync_error_codegen::codegen::rust::config::RustBackendConfig;
use zksync_error_codegen::codegen::rust::RustBackend;
use zksync_error_codegen::codegen::Backend as _;
use zksync_error_codegen::loader::load;
use zksync_error_codegen::loader::ErrorBasePart;
use zksync_error_codegen::loader::FileFormatError;
use zksync_error_codegen::loader::LoadError;
use zksync_error_codegen::model::builder::{translate_model, ModelTranslationContext};
use zksync_error_codegen::model::validator::validate;

fn main_inner(arguments: Arguments) -> Result<(), ProgramError> {
    let json_path = &arguments.definitions;
    let verbose = arguments.verbose;
    let backend_type = arguments.backend;

    if verbose {
        eprintln!("Reading config from \"{json_path}\"");
    }
    match load(json_path)? {
        ErrorBasePart::Domain(_) => Err(LoadError::FileFormatError(
            FileFormatError::ExpectedFullGotDomain(json_path.to_string()),
        )
        .into()),
        ErrorBasePart::Component(_) => Err(LoadError::FileFormatError(
            FileFormatError::ExpectedFullGotComponent(json_path.to_string()),
        )
        .into()),
        ErrorBasePart::Root(config) => {
            if verbose {
                eprintln!("Successfully parsed config from \"{json_path}\":\n{config:#?}");
                eprintln!("Building model...");
            }

            let model = translate_model(&config, ModelTranslationContext { origin: json_path })?;
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
                arguments::Backend::DocHtml => {
                    let mut backend = HtmlBackend::new(&model);
                    backend.generate(&HtmlBackendConfig {})?
                }
                arguments::Backend::Rust => {
                    let mut backend = RustBackend::new(model);
                    backend.generate(&RustBackendConfig {})?
                }
                arguments::Backend::MDBook => {
                    let mut backend = MDBookBackend::new(model);
                    backend.generate(&MDBookBackendConfig::default())?
                },
            };

            if verbose {
                eprintln!("Generation successful. Files: ");
                for file in &result {
                    eprintln!("- {}", file.relative_path.to_str().unwrap());
                }
                eprintln!("Writing files to disk...");
            }

            create_files_in_result_directory(&arguments.output_directory, result)?;
            if verbose {
                eprintln!("Writing successful.");
            }
            Ok(())
        }
    }
}

fn create_files_in_result_directory(result_dir: &str, files: Vec<File>) -> std::io::Result<()> {
    let result_dir = Path::new(result_dir);

    if result_dir.exists() {
        std::fs::remove_dir_all(result_dir)?;
    }

    std::fs::create_dir(result_dir)?;

    for file in files {
        let path = result_dir.join(file.relative_path);

        if let Some(parent_dir) = path.parent() {
            std::fs::create_dir_all(parent_dir)?;
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
