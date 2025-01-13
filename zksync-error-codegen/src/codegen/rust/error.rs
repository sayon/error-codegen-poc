use zksync_error_model::error::ModelError;

#[derive(Debug)]
pub enum GenerationError {
    ModelError(ModelError),
    ModelSerialization(serde_json::Error),
}

impl From<serde_json::Error> for GenerationError {
    fn from(v: serde_json::Error) -> Self {
        Self::ModelSerialization(v)
    }
}

impl std::fmt::Display for GenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:#?}"))
    }
}
impl std::error::Error for GenerationError {}

impl From<ModelError> for GenerationError {
    fn from(value: ModelError) -> Self {
        GenerationError::ModelError(value)
    }
}
