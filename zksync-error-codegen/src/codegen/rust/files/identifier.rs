use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_identifier(&mut self) -> Result<File, GenerationError> {
        let domains = || self.model.domains.values();
        let mut gen = PrettyPrinter::default();
        Self::preamble(&mut gen);

        for domain_description in domains() {
            let domain_code = Self::domain_code_type_name(&domain_description)?;
            gen.push_line(&format!("use crate::error::domains::{domain_code};"));
        }

        gen.push_line(
            r#"
use crate::kind::DomainCode;
use crate::kind::Kind;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Identifier {
    pub kind: Kind,
    pub code: i32,
}

impl Identifier {
    pub fn new(kind: Kind, code: i32) -> Self {
        Self { kind, code }
    }

    pub fn encode(&self) -> i32 {
        let domain_code: i32 = self.kind.domain_code();
        let component_code: i32 = self.kind.component_code();
        domain_code * 10000 + component_code * 1000 + self.code
    }

    pub fn decode(raw_code: i32) -> Option<Self> {
        let code = raw_code % 1000;
        let component_code = (raw_code / 1000) % 10;
        let domain = DomainCode::from_repr((raw_code / 10000) % 10)?;
        let kind: Kind = match domain {"#,
        );

        gen.indent_more_by(3);

        let domains = || self.model.domains.values();

        for domain_description in domains() {
            let domain = Self::domain_type_name(&domain_description)?;
            let domain_code = Self::domain_code_type_name(&domain_description)?;

            gen.push_line(&format!("DomainCode::{domain} => {{"));
            gen.indent_more();

            for component_description in domain_description.components.values() {
                let component = Self::component_type_name(component_description)?;
                gen.push_line(&format!(
                    "Kind::{domain}({domain_code}::from_repr(component_code)?)"
                ));
            }

            gen.indent_less();
            gen.push_line("},");
        }
        gen.indent_less_by(3);
        gen.push_line(
            r#"
       };
        Some(Identifier { kind, code })
    }
}"#,
        );

        Ok(File {
            content: gen.get_buffer(),
            relative_path: vec!["identifier.rs".into()],
        })
    }
}
