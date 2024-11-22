use crate::loader::LoadError;

#[derive(Debug)]
pub enum TakeFromError {
    IOError(LoadError),
    ParsingError(serde_json::Error),
    MissingComponent {
        domain_name: String,
        component_name: String,
    },
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
            TakeFromError::MissingComponent {
                domain_name,
                component_name,
            } => f.write_fmt(format_args!(
                "Unable to find a matching component {component_name} in the domain {domain_name}."
            )),
            TakeFromError::ParsingError(error) => {
                f.write_fmt(format_args!("Unable to parse errors: {error}."))
            }
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
