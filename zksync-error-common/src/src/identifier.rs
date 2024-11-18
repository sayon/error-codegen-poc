use crate::error::domains::CompilerComponentCode;
use crate::error::domains::ToolingComponentCode;
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
        let kind: Kind = match domain {
            DomainCode::CompilerError => {
                Kind::CompilerError(CompilerComponentCode::from_repr(component_code)?)
            }
            DomainCode::ToolingError => {
                Kind::ToolingError(ToolingComponentCode::from_repr(component_code)?)
            }
        };
        Some(Identifier { kind, code })
    }
}
