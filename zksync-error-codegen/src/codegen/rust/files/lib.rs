use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_lib(&mut self) -> Result<File, GenerationError> {
        let mut gen = PrettyPrinter::default();
        Self::preamble(&mut gen);
        gen.push_line(
            r#"
pub mod error;
pub mod identifier;
pub mod kind;
pub mod packed;
pub mod serialized;
pub mod untyped;
"#,
        );

        Ok(File {
            content: gen.get_buffer(),
            relative_path: vec!["src".into(), "lib.rs".into()],
        })
    }
}
