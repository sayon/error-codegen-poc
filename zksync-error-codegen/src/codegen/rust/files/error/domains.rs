use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;
use crate::model::DomainDescription;

impl RustBackend {
    pub fn generate_file_error_domains(&mut self) -> Result<File, GenerationError> {
        let mut gen = PrettyPrinter::default();

        Self::preamble(&mut gen);

        gen.push_str(
            "
use crate::error::ICustomError;
use crate::error::IUnifiedError;
use crate::kind::Kind;
use strum_macros::EnumDiscriminants;
use strum_macros::FromRepr;
",
        );

        let domains = || self.model.domains.values();
        let components = || {
            self.model
                .domains
                .values()
                .flat_map(|domain| domain.components.values())
        };

        for component in components() {
            let component_error_type = Self::component_type_name(&component)?;
            gen.push_line(&format!(
                "use crate::error::definitions::{component_error_type};"
            ));
            gen.push_line(&format!(
                "use crate::error::definitions::{component_error_type}Code;"
            ));
        }

        gen.push_line(
            r#"
#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ZksyncError {"#,
        );

        gen.indent_more();
        for domain in domains() {
            let constructor = Self::domain_type_name(&domain)?;
            let domain_type = &constructor;
            gen.push_line(&format!("{constructor}({domain_type}), "));
        }

        gen.indent_less();

        gen.push_line("}");

        gen.push_line(
            r#"
impl ZksyncError {
    pub fn get_kind(&self) -> crate::kind::Kind {
        match self {"#,
        );

        gen.indent_more_by(3);

        for domain_description in domains() {
            for component_description in domain_description.components.values() {
                let domain = Self::domain_type_name(&domain_description)?;
                let component = Self::component_type_name(component_description)?;
                let component_code = Self::component_code_type_name(component_description)?;
                let domain_code = Self::domain_code_type_name(domain_description)?;

                gen.push_line(&format!(
                    "ZksyncError::{domain}({domain}::{component}(_)) => {{ Kind::{domain}({domain_code}::{component}) }},"
                ));
            }
        }

        for _ in 0..2 {
            gen.indent_less();
            gen.push_line("}");
        }

        gen.push_line(
            r#"
    pub fn get_code(&self) -> i32 {
        match self {"#,
        );

        gen.indent_more_by(2);
        for domain_description in domains() {
            for component_description in domain_description.components.values() {
                let domain = Self::domain_type_name(&domain_description)?;
                let component = Self::component_type_name(component_description)?;
                let component_code = Self::component_code_type_name(component_description)?;
                gen.push_line(&format!("ZksyncError::{domain}({domain}::{component}(error)) => {{ Into::<{component_code}>::into(error) as i32 }},"));
            }
        }
        for _ in 0..3 {
            gen.indent_less();
            gen.push_line("}");
        }

        gen.push_line(
            r#"
impl std::fmt::Display for ZksyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}
impl IUnifiedError<ZksyncError> for ZksyncError {}
impl std::error::Error for ZksyncError {}
"#,
        );

        for domain_description in domains() {
            gen.push_block(&Self::define_domain_enum(domain_description)?);
        }

        for domain_description in domains() {
            for component_description in domain_description.components.values() {
                let domain = Self::domain_type_name(&domain_description)?;
                let component = Self::component_type_name(component_description)?;
                gen.push_line(&format!(
                    r#"
impl ICustomError<ZksyncError, ZksyncError> for {component} {{
    fn to_unified(&self) -> ZksyncError {{
        ZksyncError::{domain}({domain}::{component}(self.clone()))
    }}
}}
"#
                ));
            }
        }

        Ok(File {
            relative_path: vec!["error".into(), "domains.rs".into()],
            content: gen.get_buffer(),
        })
    }
    fn define_domain_enum(
        domain_description: &DomainDescription,
    ) -> Result<String, GenerationError> {
        let mut gen = PrettyPrinter::default();
        let domain = Self::domain_type_name(&domain_description)?;
        let domain_code = Self::domain_code_type_name(domain_description)?;

        gen.push_line(&format!(
            r#"
#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name({domain_code}))]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum {domain} {{"#
        ));

        gen.indent_more();

        for component in domain_description.components.values() {
            let component_name = Self::component_type_name(component)?;
            gen.push_line(&format!("{component_name}({component_name}),"));
        }
        gen.indent_less();
        gen.push_line("}\n");

        Ok(gen.get_buffer())
    }
}

/*

#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(CompilerCode))]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum Compiler {
    Zksolc(Zksolc),
    Solc(Solc),
}

#[repr(i32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(ToolingCode))]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum Tooling {
    RustSDK(RustSDK),
}

impl ICustomError<ZksyncError, ZksyncError> for Zksolc {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::Compiler(Compiler::Zksolc(self.clone()))
    }
}
impl ICustomError<ZksyncError, ZksyncError> for Solc {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::Compiler(Compiler::Solc(self.clone()))
    }
}
impl ICustomError<ZksyncError, ZksyncError> for RustSDK {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::Tooling(Tooling::RustSDK(self.clone()))
    }
}

impl std::fmt::Display for ZksyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}
impl std::error::Error for ZksyncError {}
*/
/*

    pub fn get_code(&self) -> i32 {
        match self {
            ZksyncError::CompilerError(compiler_error) => match compiler_error {
                CompilerError::Zksolc(zksolc_error) => {
                    Into::<ZksolcErrorCode>::into(zksolc_error) as i32
                }
                CompilerError::Solc(solc_error) => Into::<SolcErrorCode>::into(solc_error) as i32,
            },
            ZksyncError::ToolingError(tooling_error) => match tooling_error {
                ToolingError::RustSDK(rust_sdkerror) => {
                    Into::<RustSDKErrorCode>::into(rust_sdkerror) as i32
                }
            },
        }
    }
}
impl IUnifiedError<ZksyncError> for ZksyncError {}

#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(CompilerComponentCode))]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum CompilerError {
    Zksolc(ZksolcError),
    Solc(SolcError),
}

#[repr(i32)]
#[derive(Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name(ToolingComponentCode))]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum ToolingError {
    RustSDK(RustSDKError),
}

impl ICustomError<ZksyncError, ZksyncError> for ZksolcError {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::CompilerError(CompilerError::Zksolc(self.clone()))
    }
}
impl ICustomError<ZksyncError, ZksyncError> for SolcError {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::CompilerError(CompilerError::Solc(self.clone()))
    }
}
impl ICustomError<ZksyncError, ZksyncError> for RustSDKError {
    fn to_unified(&self) -> ZksyncError {
        ZksyncError::ToolingError(ToolingError::RustSDK(self.clone()))
    }
}

impl std::fmt::Display for ZksyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}
impl std::error::Error for ZksyncError {}
*/
