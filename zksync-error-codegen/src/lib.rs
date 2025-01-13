pub mod arguments;
pub mod codegen;
pub mod error;
pub mod error_database;
pub mod loader;

use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;

use arguments::Backend;
use arguments::GenerationArguments;
use error::ProgramError;

use crate::codegen::file::File;
use crate::codegen::html::config::HtmlBackendConfig;
use crate::codegen::html::HtmlBackend;
use crate::codegen::mdbook::config::MDBookBackendConfig;
use crate::codegen::mdbook::MDBookBackend;
use crate::codegen::rust::config::RustBackendConfig;
use crate::codegen::rust::RustBackend;
use crate::codegen::Backend as _;
use crate::loader::error::FileFormatError;
use crate::loader::error::LoadError;
use crate::loader::load;
use crate::loader::ErrorBasePart;
use crate::loader::builder::{translate_model, ModelTranslationContext};
use zksync_error_model::validator::validate;

pub fn default_load_and_generate(root_error_package_name: &str) {
    if let Err(e) = load_and_generate(GenerationArguments {
        verbose: true,
        root_link: format!("cargo://{root_error_package_name}@@errors.json"),
        outputs: vec![
            ("../zksync_error".into(), Backend::Rust),
            ("../doc-mdbook".into(), Backend::MDBook),
        ],
    }) {
        eprintln!("{e:#?}")
    };
}
pub fn load_and_generate(arguments: GenerationArguments) -> Result<(), ProgramError> {
    let GenerationArguments {
        verbose,
        root_link,
        outputs,
    } = &arguments;
    if *verbose {
        eprintln!("Reading config from \"{root_link}\"");
    }
    match load(root_link)? {
        ErrorBasePart::Domain(_) => Err(LoadError::FileFormatError(
            FileFormatError::ExpectedFullGotDomain(root_link.to_string()),
        )
        .into()),
        ErrorBasePart::Component(_) => Err(LoadError::FileFormatError(
            FileFormatError::ExpectedFullGotComponent(root_link.to_string()),
        )
        .into()),
        ErrorBasePart::Root(config) => {
            if *verbose {
                eprintln!("Successfully parsed config from \"{root_link}\":\n{config:#?}");
                eprintln!("Building model...");
            }

            let model = translate_model(&config, ModelTranslationContext { origin: root_link })?;
            if *verbose {
                eprintln!("Model: {model:#?}");
                eprintln!("Model validation...");
            }
            validate(&model).unwrap();
            if *verbose {
                eprintln!("Model validation successful.");
            }

            for (output_directory, backend_type) in outputs {
                if *verbose {
                    eprintln!("Selected backend: {backend_type:?}. \nGenerating files...");
                }
                let result = match backend_type {
                    arguments::Backend::DocHtml => {
                        let mut backend = HtmlBackend::new(&model);
                        backend.generate(&HtmlBackendConfig {})?
                    }
                    arguments::Backend::Rust => {
                        let mut backend = RustBackend::new(&model);
                        backend.generate(&RustBackendConfig {})?
                    }
                    arguments::Backend::MDBook => {
                        let mut backend = MDBookBackend::new(&model);
                        backend.generate(&MDBookBackendConfig)?
                    }
                };

                if *verbose {
                    eprintln!("Generation successful. Files: ");
                    for file in &result {
                        eprintln!("- {}", file.relative_path.to_str().unwrap());
                    }
                    eprintln!("Writing files to disk...");
                }

                create_files_in_result_directory(output_directory, result)?;
                if *verbose {
                    eprintln!("Writing successful.");
                }
            }
            Ok(())
        }
    }
}
fn create_files_in_result_directory(result_dir: &PathBuf, files: Vec<File>) -> std::io::Result<()> {
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
