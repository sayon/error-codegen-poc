use std::collections::HashMap;

use super::{
    identifier::ErrorIdentifier, ComponentCode, ComponentName, DomainCode, DomainName, ErrorCode,
    ErrorMessageTemplate, ErrorName, FieldName, LanguageName, Model, Semver, TypeName,
};

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct TargetLanguageType {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct TypeMetadata {
    pub description: String,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct TypeDescription {
    pub name: TypeName,
    pub meta: TypeMetadata,
    pub bindings: HashMap<LanguageName, TargetLanguageType>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct DomainMetadata {
    pub name: DomainName,
    pub code: DomainCode,
    pub components: Vec<ComponentName>,
    pub bindings: HashMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}
#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct FlatModel {
    pub types: HashMap<TypeName, TypeDescription>,
    pub domains: HashMap<DomainName, DomainMetadata>,
    pub components: HashMap<ComponentName, ComponentMetadata>,
    pub errors: HashMap<ErrorName, ErrorDescription>,
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

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct FieldDescription {
    pub name: FieldName,
    pub r#type: TypeName,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct ErrorDocumentation {
    pub description: String,
    pub short_description: String,
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

fn translate_domain_metadata(
    meta: &super::DomainMetadata,
    components: Vec<ComponentName>,
) -> DomainMetadata {
    let super::DomainMetadata {
        name,
        code,
        bindings,
        identifier,
        description,
    } = meta.clone();
    DomainMetadata {
        name,
        code,
        bindings,
        identifier,
        description,
        components,
    }
}

fn translate_component_metadata(meta: &super::ComponentMetadata) -> ComponentMetadata {
    let super::ComponentMetadata {
        name,
        code,
        bindings,
        identifier,
        description,
    } = meta.clone();
    ComponentMetadata {
        name,
        code,
        bindings,
        identifier,
        description,
    }
}
fn translate_field(field: &super::FieldDescription) -> FieldDescription {
    let super::FieldDescription { name, r#type } = field.clone();
    FieldDescription { name, r#type }
}
fn translate_error(meta: &super::ErrorDescription) -> ErrorDescription {
    let super::ErrorDescription {
        domain,
        component,
        name,
        code,
        message,
        fields,
        documentation,
        bindings,
    } = meta;
    let new_bindings: HashMap<_, _> = bindings
        .bindings
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                TargetLanguageType {
                    name: v.name.to_string(),
                    path: String::default(),
                },
            )
        })
        .collect();
    let identifier = ErrorIdentifier {
        domain: domain.identifier.clone(),
        component: component.identifier.clone(),
        code: code.clone(),
    }
    .to_string();
    ErrorDescription {
        domain: domain.name.clone(),
        component: component.name.clone(),
        name: name.clone(),
        code: code.clone(),
        identifier,
        message: message.clone(),
        fields: fields.iter().map(translate_field).collect(),
        documentation: documentation
            .clone()
            .map(|d| translate_documentation(&d))
            .unwrap_or_default(),
        bindings: new_bindings,
    }
}

fn translate_owner(doc: &super::VersionedOwner) -> VersionedOwner {
    let super::VersionedOwner { name, version } = doc.clone();
    VersionedOwner { name, version }
}
fn translate_likely_cause(doc: &super::LikelyCause) -> LikelyCause {
    let super::LikelyCause {
        cause,
        fixes,
        report,
        owner,
        references,
    } = doc.clone();

    LikelyCause {
        cause,
        fixes,
        report,
        owner: translate_owner(&owner),
        references,
    }
}
fn translate_documentation(doc: &super::ErrorDocumentation) -> ErrorDocumentation {
    let super::ErrorDocumentation {
        description,
        short_description,
        likely_causes,
    } = doc.clone();

    ErrorDocumentation {
        description,
        short_description: short_description.unwrap_or_default(),
        likely_causes: likely_causes.iter().map(translate_likely_cause).collect(),
    }
}

fn translate_type(typ: &super::TypeDescription) -> TypeDescription {
    let super::TypeDescription {
        name,
        meta: super::TypeMetadata { description },
        bindings,
    } = typ.clone();

    let new_bindings: HashMap<_, _> = bindings
        .bindings
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                TargetLanguageType {
                    name: v.name.to_string(),
                    path: String::default(),
                },
            )
        })
        .collect();
    TypeDescription {
        name,
        meta: TypeMetadata { description },
        bindings: new_bindings,
    }
}
pub fn flatten(model: &Model) -> FlatModel {
    let Model { types, domains } = model;
    let mut result = FlatModel::default();
    for (name, typ) in types {
        result.types.insert(name.clone(), translate_type(&typ));
    }

    for (domain_name, super::DomainDescription { meta, components }) in domains {
        let component_names: Vec<_> = components.keys().cloned().collect();
        result.domains.insert(
            domain_name.to_string(),
            translate_domain_metadata(&meta, component_names),
        );
        result.components.extend(
            components
                .iter()
                .map(|(n, c)| (n.to_string(), translate_component_metadata(&c.meta))),
        );

        for component in components.values() {
            result.errors.extend(
                component
                    .errors
                    .iter()
                    .map(|e| (e.name.to_string(), translate_error(e))),
            )
        }
    }

    result
}
