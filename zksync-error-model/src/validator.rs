use super::error::ModelError;
use super::inner::Model;

pub fn validate(_model: &Model) -> Result<(), ModelError> {
    Ok(())
}
