pub mod identifier;

use std::{collections::HashMap, rc::Rc};

use identifier::ErrorIdentifier;

use super::error::ModelError;

pub type LanguageName = String;
pub type TypeName = String;
pub type FieldName = String;
pub type ComponentName = String;
pub type DomainName = String;
pub type ErrorName = String;
pub type ErrorCode = u32;
pub type ComponentCode = u32;
pub type DomainCode = u32;
pub type ErrorMessageTemplate = String;
pub type Semver = String;

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct FullyQualifiedTargetLanguageType {
    pub name: String,
    pub path: String,
}
#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct TargetLanguageType {
    pub name: String,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct TypeMetadata {
    pub description: String,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct TypeBindings<T>
where
    T: Eq,
{
    pub bindings: HashMap<LanguageName, T>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct TypeDescription {
    pub name: TypeName,
    pub meta: TypeMetadata,
    pub bindings: TypeBindings<FullyQualifiedTargetLanguageType>,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct Model {
    pub types: HashMap<TypeName, TypeDescription>,
    pub domains: HashMap<DomainName, DomainDescription>,
}

impl Model {
    pub fn new(
        types: HashMap<TypeName, TypeDescription>,
        domains: HashMap<DomainName, DomainDescription>,
    ) -> Self {
        Self { types, domains }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct DomainMetadata {
    pub name: DomainName,
    pub code: DomainCode,
    pub bindings: HashMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct DomainDescription {
    pub meta: Rc<DomainMetadata>,
    pub components: HashMap<ComponentName, ComponentDescription>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct ComponentMetadata {
    pub name: ComponentName,
    pub code: ComponentCode,
    pub bindings: HashMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct ComponentDescription {
    pub meta: Rc<ComponentMetadata>,
    pub errors: Vec<ErrorDescription>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct ErrorDescription {
    pub domain: Rc<DomainMetadata>,
    pub component: Rc<ComponentMetadata>,
    pub name: ErrorName,
    pub code: ErrorCode,
    pub message: ErrorMessageTemplate,
    pub fields: Vec<FieldDescription>,
    pub documentation: Option<ErrorDocumentation>,
    pub bindings: TypeBindings<TargetLanguageType>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct FieldDescription {
    pub name: FieldName,
    pub r#type: TypeName,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct ErrorDocumentation {
    pub description: String,
    pub short_description: Option<String>,
    pub likely_causes: Vec<LikelyCause>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct LikelyCause {
    pub cause: String,
    pub fixes: Vec<String>,
    pub report: String,
    pub owner: VersionedOwner,
    pub references: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct VersionedOwner {
    pub name: String,
    pub version: Semver,
}

impl Model {
    pub fn get_type(
        &self,
        language: &str,
        name: &str,
    ) -> Result<&FullyQualifiedTargetLanguageType, ModelError> {
        let type_description = self
            .types
            .get(name)
            .ok_or(ModelError::UnknownType(name.to_string()))?;
        let mapped_type = type_description
            .bindings
            .bindings
            .get(language)
            .ok_or(ModelError::UnmappedType(name.to_string()))?;
        Ok(mapped_type)
    }
}
