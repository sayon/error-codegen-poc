use std::collections::HashMap;


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

#[non_exhaustive]
#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TargetLanguageType {
    pub name: String,
    pub path: String,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeMetadata {
    pub description: String,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeDescription {
    pub name: TypeName,
    pub meta: TypeMetadata,
    pub bindings: HashMap<LanguageName, TargetLanguageType>,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct DomainMetadata {
    pub name: DomainName,
    pub code: DomainCode,
    pub components: Vec<ComponentName>,
    pub bindings: HashMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}
#[non_exhaustive]
#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorHierarchy {
    pub types: HashMap<TypeName, TypeDescription>,
    pub domains: HashMap<DomainName, DomainMetadata>,
    pub components: HashMap<ComponentName, ComponentMetadata>,
    pub errors: HashMap<ErrorName, ErrorDescription>,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComponentMetadata {
    pub name: ComponentName,
    pub code: ComponentCode,
    pub domain_name: DomainName,
    pub bindings: HashMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorDescription {
    pub domain: DomainName,
    pub component: ComponentName,
    pub name: ErrorName,
    pub code: ErrorCode,
    pub identifier: String,
    pub message: ErrorMessageTemplate,
    pub fields: Vec<FieldDescription>,
    pub documentation: ErrorDocumentation,
    pub bindings: HashMap<LanguageName, TargetLanguageType>,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldDescription {
    pub name: FieldName,
    pub r#type: TypeName,
}

#[non_exhaustive]
#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorDocumentation {
    pub description: String,
    pub short_description: String,
    pub likely_causes: Vec<LikelyCause>,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct LikelyCause {
    pub cause: String,
    pub fixes: Vec<String>,
    pub report: String,
    pub owner: VersionedOwner,
    pub references: Vec<String>,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionedOwner {
    pub name: String,
    pub version: Semver,
}
