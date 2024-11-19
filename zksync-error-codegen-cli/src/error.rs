use zksync_error_codegen::codegen::rust::error::GenerationError;
use zksync_error_codegen::model::builder::error::ModelBuildingError;
use zksync_error_codegen::model::error::ModelError;

#[derive(Debug)]
pub enum ProgramError {
    ModelError(ModelError),
    ModelBuildingError(ModelBuildingError),
    JsonDeserializationError(serde_json::Error),
    RustGenerationError(GenerationError),
    IOError(std::io::Error),
}

impl From<ModelBuildingError> for ProgramError {
    fn from(v: ModelBuildingError) -> Self {
        Self::ModelBuildingError(v)
    }
}

impl From<serde_json::Error> for ProgramError {
    fn from(v: serde_json::Error) -> Self {
        Self::JsonDeserializationError(v)
    }
}

impl From<ModelError> for ProgramError {
    fn from(v: ModelError) -> Self {
        Self::ModelError(v)
    }
}

impl From<GenerationError> for ProgramError {
    fn from(v: GenerationError) -> Self {
        Self::RustGenerationError(v)
    }
}

impl From<std::io::Error> for ProgramError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}
