#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub types: Vec<Type>,
    pub domains: Vec<Domain>,
}

#[derive(Debug, Deserialize)]
pub struct Type {
    pub name: String,
    pub description: String,
    pub bindings: TypeMappings,
}

#[derive(Debug, Deserialize)]
pub struct ErrorNameMapping {
    pub rust: Option<ErrorType>,
    #[serde(default)]
    pub typescript: Option<ErrorType>,
}

#[derive(Debug, Deserialize)]
pub struct TypeMappings {
    pub rust: Option<FullyQualifiedType>,
    #[serde(default)]
    pub typescript: Option<FullyQualifiedType>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorType {
    pub name: String,
}
#[derive(Debug, Deserialize)]
pub struct FullyQualifiedType {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct NameBindings {
    pub rust: String,
}

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub domain_name: String,
    pub domain_code: u32,
    pub identifier_encoding: String,
    pub description: Option<String>,
    pub components: Vec<Component>,
    pub bindings: NameBindings,
}

#[derive(Debug, Deserialize)]
pub struct Component {
    pub component_name: String,
    pub component_code: u32,

    pub bindings: NameBindings,
    pub identifier_encoding: String,
    pub description: Option<String>,
    #[serde(default)]
    pub takeFrom: Vec<String>,

    #[serde(default)]
    pub errors: Vec<Error>,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub name: String,
    pub code: u32,
    pub message: String,
    pub bindings: ErrorNameMapping,
    pub fields: Vec<Field>,

    #[serde(default)]
    pub doc: Option<ErrorDocumentation>,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDocumentation {
    pub description: String,
    pub short_description: Option<String>,
    pub likely_causes: Vec<LikelyCause>,
}

#[derive(Debug, Deserialize)]
pub struct LikelyCause {
    pub cause: String,
    pub fixes: Vec<String>,
    #[serde(default)]
    pub report: String,
    pub owner: VersionedOwner,
    #[serde(default)]
    pub references: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct VersionedOwner {
    pub name: String,
    #[serde(default)]
    pub version: String,
}

impl Config {
    pub fn get_component(&self, domain: &str, component: &str) -> Option<&Component> {
        let domain = self.domains.iter().find(|d| d.domain_name == domain)?;
        let component = domain
            .components
            .iter()
            .find(|c| c.component_name == component)?;

        Some(component)
    }
}
