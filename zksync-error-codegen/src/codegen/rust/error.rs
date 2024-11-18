use crate::model::error::ModelError;

#[derive(Debug)]
pub enum GenerationError {
    ModelError(ModelError),
}

impl std::fmt::Display for GenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => f.write_fmt(format_args!("{self:#?}")),
        }
    }
}
impl std::error::Error for GenerationError {}

impl From<ModelError> for GenerationError {
    fn from(value: ModelError) -> Self {
        GenerationError::ModelError(value)
    }
}
