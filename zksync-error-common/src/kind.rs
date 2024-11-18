use strum_macros::{EnumDiscriminants, FromRepr};

use crate::error::domains::CompilerComponentCode;
use crate::error::domains::ToolingComponentCode;

#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[strum_discriminants(name(DomainCode))]
#[strum_discriminants(derive(FromRepr))]
#[strum_discriminants(vis(pub))]
#[repr(i32)]
pub enum Kind {
    Compiler(CompilerComponentCode) = 2,
    Tooling(ToolingComponentCode) = 3,
}

impl Kind {
    pub fn domain_code(&self) -> i32 {
        let domain: DomainCode = self.clone().into();
        domain as i32
    }
    pub fn component_code(&self) -> i32 {
        match self {
            Kind::Compiler(component) => component.clone() as i32,
            Kind::Tooling(component) => component.clone() as i32,
        }
    }
}
