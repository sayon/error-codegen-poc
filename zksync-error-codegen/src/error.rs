use crate::codegen::html::error::GenerationError as HtmlGenerationError;
use crate::codegen::mdbook::error::GenerationError as MarkdownGenerationError;
use crate::codegen::rust::error::GenerationError as RustGenerationError;
use crate::loader::error::LoadError;
use crate::model::builder::error::ModelBuildingError;
use crate::model::error::ModelError;

#[derive(Debug)]
pub enum ProgramError {
    ModelError(ModelError),
    ModelBuildingError(ModelBuildingError),
    JsonDeserializationError(serde_json::Error),
    RustGenerationError(RustGenerationError),
    HtmlGenerationError(HtmlGenerationError),
    MarkdownGenerationError(MarkdownGenerationError),
    IOError(std::io::Error),
    LoadError(LoadError),
}

impl From<MarkdownGenerationError> for ProgramError {
    fn from(v: MarkdownGenerationError) -> Self {
        Self::MarkdownGenerationError(v)
    }
}

impl From<LoadError> for ProgramError {
    fn from(v: LoadError) -> Self {
        Self::LoadError(v)
    }
}

impl From<HtmlGenerationError> for ProgramError {
    fn from(v: HtmlGenerationError) -> Self {
        Self::HtmlGenerationError(v)
    }
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

impl From<RustGenerationError> for ProgramError {
    fn from(v: RustGenerationError) -> Self {
        Self::RustGenerationError(v)
    }
}

impl From<std::io::Error> for ProgramError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}
