use std::path::PathBuf;

use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

fn replace_non_alphanumeric(input: &str, replacement: char) -> String {
    input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { replacement })
        .collect()
}

impl RustBackend {
    pub fn generate_file_lib(&mut self) -> Result<File, GenerationError> {
        let mut gen = PrettyPrinter::default();
        Self::preamble(&mut gen);
        gen.push_line(
            r#"
#![allow(unused)]

pub mod error;
pub mod identifier;
pub mod kind;
pub mod packed;
pub mod serialized;
pub mod untyped;
pub mod documentation;


pub use crate::error::domains::ZksyncError;
"#,
        );

        fn sanitize(s: &str) -> String { replace_non_alphanumeric(s.into(), '_') }

        for domain in self.model.domains.values() {
            gen.push_line(&format!("pub mod {} {{", sanitize(&domain.meta.identifier)));
            gen.indent_more();
            for component in domain.components.values() {
                gen.push_line(&format!("pub mod {} {{", sanitize(&component.meta.identifier)));
                gen.indent_more();

                for error in &component.errors {
                    let enum_name = component
                        .meta
                        .bindings
                        .get("rust")
                        .expect("Internal model error");
                    let enum_variant = sanitize(
                        &error
                            .bindings
                            .bindings
                            .get("rust")
                            .expect("Internal model error")
                            .name);
                    gen.push_line(&format!(
                        "pub use crate::error::definitions::{enum_name}::{enum_variant};"
                    ));
                }

                gen.push_block(r#"
macro_rules! generic_error {
    ($($arg:tt)*) => {
        GenericError { message: format!($($arg)*) }
    };
}
"#);

                gen.push_line("}");
                gen.indent_less();
            }
            gen.push_line("}");
            gen.indent_less();
        }
        Ok(File {
            content: gen.get_buffer(),
            relative_path: PathBuf::from("src/lib.rs"),
        })
    }
}
