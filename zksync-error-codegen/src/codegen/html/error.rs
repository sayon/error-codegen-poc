use crate::model::error::ModelError;

#[derive(Debug)]
pub enum GenerationError {
    ModelError(ModelError),
    TemplateError(tera::Error),
}

impl From<tera::Error> for GenerationError {
    fn from(v: tera::Error) -> Self {
        Self::TemplateError(v)
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