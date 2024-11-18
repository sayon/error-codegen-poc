//!
//! Immutable for user
//!
//!
use crate::error::ICustomError;
use crate::error::IUnifiedError;

use crate::kind::Kind;

use strum_macros::EnumDiscriminants;
use strum_macros::FromRepr;

use crate::error::definitions::RustSDKError;
use crate::error::definitions::RustSDKErrorCode;
use crate::error::definitions::SolcError;
use crate::error::definitions::SolcErrorCode;
use crate::error::definitions::ZksolcError;
use crate::error::definitions::ZksolcErrorCode;

#[repr(i32)]
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ZksyncError {
    CompilerError(CompilerError),
    ToolingError(ToolingError),
}

impl ZksyncError {
    pub fn get_kind(&self) -> crate::kind::Kind {
        match self {
            ZksyncError::CompilerError(CompilerError::Zksolc(_)) => {
                Kind::CompilerError(CompilerComponentCode::Zksolc)
            }
            ZksyncError::CompilerError(CompilerError::Solc(_)) => {
                Kind::CompilerError(CompilerComponentCode::Solc)
            }
            ZksyncError::ToolingError(ToolingError::RustSDK(_)) => {
                Kind::ToolingError(ToolingComponentCode::RustSDK)
            }
        }
    }
    pub fn get_code(&self) -> i32 {
        match self {
            ZksyncError::CompilerError(CompilerError::Zksolc(error)) => {
                Into::<ZksolcErrorCode>::into(error) as i32
            }
            ZksyncError::CompilerError(CompilerError::Solc(error)) => {
                Into::<SolcErrorCode>::into(error) as i32
            }
            ZksyncError::ToolingError(ToolingError::RustSDK(error)) => {
                Into::<RustSDKErrorCode>::into(error) as i32
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
