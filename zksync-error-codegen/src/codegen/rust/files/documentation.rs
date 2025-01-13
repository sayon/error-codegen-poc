use std::path::PathBuf;

use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_documentation(&mut self) -> Result<File, GenerationError> {
        let mut gen = PrettyPrinter::default();
        Self::preamble(&mut gen);
        gen.push_line(
            r#"
use lazy_static::lazy_static;
use zksync_error_model::ErrorModel;

lazy_static! {
   pub static ref model : ErrorModel = get_model();
}


fn get_model() -> ErrorModel {
    let serialized_model = include_str!("../resources/model.json");
    serde_json::from_str(serialized_model).expect("Always valid")

}
"#,
        );

        Ok(File {
            content: gen.get_buffer(),
            relative_path: PathBuf::from("src/documentation.rs"),
        })
    }
}
