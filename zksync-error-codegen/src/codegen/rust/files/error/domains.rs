use crate::{
    codegen::{
        printer::PrettyPrinter,
        rust::{error::GenerationError, RustBackend},
        File,
    },
    model::ComponentDescription,
};

impl RustBackend {
    pub fn generate_domains(&mut self) -> Result<File, GenerationError> {
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

        let components = || {
            self.model
                .domains
                .values()
                .flat_map(|domain| domain.components.values())
        };
        let domains = || self.model.domains.values();

        for component in components() {
            let component_error_type = Self::component_type_name(&component)?;
            gen.push_line(&format!("use crate::{component_error_type}"));
            gen.push_line(&format!("use crate::{component_error_type}Code"));
        }

        gen.push_line(
            r#"
#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ZksyncError {"#,
        );

        gen.indent_increase();
        for domain in domains() {
            let constructor = Self::domain_type_name(&domain)?;
            let domain_type = &constructor;
            gen.push_line(&format!("{constructor}({domain_type}), "));
        }

        gen.indent_decrease();

        gen.push_line("}");

        gen.push_line(
            r#"
impl ZksyncError {
    pub fn get_kind(&self) -> crate::kind::Kind {
        match self {"#,
        );

        for domain in domains() {
            for component in domain.components.values() {
                    let domain_name = Self::domain_type_name(&domain)?;
                    let component_type_name = Self::component_type_name(component)?;
                    let component_error_name = Self::component_code_type_name(component)?;
                    let domain_code_type_name = Self::domain_code_type_name(domain)?;

                    gen.push_line(&format!(
                        "ZksyncError::{domain_name}({domain_name}::{component_type_name}(_)) => {{
Kind::{domain_name}({domain_code_type_name}::{component_error_name})}},"));
                }
            }

        Ok(File {
            relative_path: vec!["error".into(), "domains.rs".into()],
            content: gen.get_buffer(),
        })
    }
}

/*

        match self {
            ZksyncError::CompilerError(CompilerError::Zksolc(_)) => {
                Kind::Compiler(CompilerComponentCode::Zksolc)
            }
            ZksyncError::CompilerError(CompilerError::Solc(_)) => {
                Kind::Compiler(CompilerComponentCode::Solc)
            }
            ZksyncError::ToolingError(ToolingError::RustSDK(_)) => {
                Kind::Tooling(ToolingComponentCode::RustSDK)
            }
        }
            ZksyncError::CompilerError(compiler_error) => Kind::Compiler(match compiler_error {
                CompilerError::Zksolc(_) => CompilerComponentCode::Zksolc,
                CompilerError::Solc(_) => CompilerComponentCode::Solc,
            }),
            ZksyncError::ToolingError(tooling_error) => Kind::Tooling(match tooling_error {
                ToolingError::RustSDK(_) => ToolingComponentCode::RustSDK,
            }),
        }
    }
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
