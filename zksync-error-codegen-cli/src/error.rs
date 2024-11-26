use zksync_error_codegen::codegen::html::error::GenerationError as HtmlGenerationError;
use zksync_error_codegen::codegen::rust::error::GenerationError as RustGenerationError;
use zksync_error_codegen::codegen::mdbook::error::GenerationError as MarkdownGenerationError;
use zksync_error_codegen::loader::LoadError;
use zksync_error_codegen::model::builder::error::ModelBuildingError;
use zksync_error_codegen::model::error::ModelError;

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
