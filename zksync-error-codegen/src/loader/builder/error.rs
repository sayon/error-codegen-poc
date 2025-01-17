use crate::loader::error::LoadError;
use zksync_error_model::merger::error::MergeError;

#[derive(Debug)]
pub struct MissingComponent {
    pub domain_name: String,
    pub component_name: String,
}

#[derive(Debug)]
pub enum TakeFromError {
    IOError(LoadError),
    ParsingError(serde_json::Error),
    MissingComponent(MissingComponent),
    ModelBuildingError(Box<ModelBuildingError>),
    MergeError(MergeError),
}

impl From<MergeError> for TakeFromError {
    fn from(v: MergeError) -> Self {
        Self::MergeError(v)
    }
}

impl From<ModelBuildingError> for TakeFromError {
    fn from(v: ModelBuildingError) -> Self {
        Self::ModelBuildingError(Box::new(v))
    }
}

impl From<MissingComponent> for TakeFromError {
    fn from(v: MissingComponent) -> Self {
        Self::MissingComponent(v)
    }
}

impl From<LoadError> for TakeFromError {
    fn from(v: LoadError) -> Self {
        Self::IOError(v)
    }
}
impl TakeFromError {
    pub fn from_address(self, address: &str) -> ModelBuildingError {
        ModelBuildingError::TakeFrom {
            address: address.to_string(),
            inner: self,
        }
    }
}
impl From<serde_json::Error> for TakeFromError {
    fn from(v: serde_json::Error) -> Self {
        Self::ParsingError(v)
    }
}
#[derive(Debug)]
pub enum ModelBuildingError {
    TakeFrom {
        address: String,
        inner: TakeFromError,
    },
}

impl std::fmt::Display for TakeFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TakeFromError::IOError(error) => f.write_fmt(format_args!("IO error: {error}")),
            TakeFromError::MissingComponent(MissingComponent {
                domain_name,
                component_name,
            }) => f.write_fmt(format_args!(
                "Unable to find a matching component {component_name} in the domain {domain_name}."
            )),
            TakeFromError::ParsingError(error) => {
                f.write_fmt(format_args!("Unable to parse errors: {error}."))
            }
            TakeFromError::ModelBuildingError(error) => f.write_fmt(format_args!(
                "Error while building model following a `takeFrom` link: {error}"
            )),
            TakeFromError::MergeError(error) => f.write_fmt(format_args!(
                "Error while merging with the error base fetched from `takeFrom` link: {error}"
            )),
        }
    }
}
impl std::fmt::Display for ModelBuildingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelBuildingError::TakeFrom { address, inner } => {
                f.write_fmt(format_args!("Failed to import a file {address}: {inner}"))
            }
        }
    }
}
impl std::error::Error for ModelBuildingError {}
