pub mod error;

use error::ModelBuildingError;
use error::TakeFromError;
use maplit::hashmap;

use crate::json::Config;
use crate::loader::fetch_file;

use super::ComponentDescription;
use super::DomainDescription;
use super::ErrorDescription;
use super::ErrorDocumentation;
use super::FieldDescription;
use super::FullyQualifiedTargetLanguageType;
use super::LikelyCause;
use super::Model;
use super::TargetLanguageType;
use super::TypeBindings;
use super::TypeDescription;
use super::TypeMetadata;
use super::VersionedOwner;

fn translate_type_bindings(
    value: &crate::json::ErrorNameMapping,
) -> Result<TypeBindings<TargetLanguageType>, ModelBuildingError> {
    let mut result = TypeBindings::<TargetLanguageType>::default();
    if let Some(crate::json::ErrorType { name }) = &value.rust {
        result
            .bindings
            .insert("rust".into(), TargetLanguageType { name: name.clone() });
    }
    Ok(result)
}

fn translate_type_mappings(
    value: &crate::json::TypeMappings,
) -> Result<TypeBindings<FullyQualifiedTargetLanguageType>, ModelBuildingError> {
    let mut result: TypeBindings<FullyQualifiedTargetLanguageType> = Default::default();
    if let Some(crate::json::FullyQualifiedType { name, path }) = &value.rust {
        result.bindings.insert(
            "rust".into(),
            FullyQualifiedTargetLanguageType {
                name: name.clone(),
                path: path.clone(),
            },
        );
    }
    Ok(result)
}

fn translate_type(value: &crate::json::Type) -> Result<TypeDescription, ModelBuildingError> {
    let crate::json::Type {
        name,
        description,
        bindings: codegen,
    } = value;
    Ok(TypeDescription {
        name: name.clone(),
        meta: TypeMetadata {
            description: description.clone(),
        },
        bindings: translate_type_mappings(codegen)?,
    })
}

pub fn translate_model(model: &crate::json::Config) -> Result<Model, ModelBuildingError> {
    let mut result = Model::default();
    let crate::json::Config { types, domains } = model;
    for t in types {
        result.types.insert(t.name.clone(), translate_type(t)?);
    }

    for domain in domains {
        let transformed_domain: DomainDescription = translate_domain(domain)?;
        result
            .domains
            .insert(transformed_domain.name.clone(), transformed_domain);
    }

    Ok(result)
}

fn translate_field(value: &crate::json::Field) -> Result<FieldDescription, ModelBuildingError> {
    let crate::json::Field { name, r#type } = value;
    Ok(FieldDescription {
        name: name.clone(),
        r#type: r#type.clone(),
    })
}

struct ErrorTranslationContext {
    component_context: ComponentTranslationContext,
    component_name: String,
}

fn translate_likely_cause(
    lc: &crate::json::LikelyCause,
) -> Result<LikelyCause, ModelBuildingError> {
    let crate::json::LikelyCause {
        cause,
        fixes,
        report,
        owner: crate::json::VersionedOwner { name, version },
        references,
    } = lc;

    let owner = VersionedOwner {
        name: name.clone(),
        version: version.clone(),
    };
    Ok(LikelyCause {
        cause: cause.clone(),
        fixes: fixes.clone(),
        report: report.clone(),
        owner,
        references: references.clone(),
    })
}
fn translate_error_documentation(
    doc: &crate::json::ErrorDocumentation,
) -> Result<ErrorDocumentation, ModelBuildingError> {
    let &crate::json::ErrorDocumentation {
        description,
        short_description,
        likely_causes,
    } = &doc;

    let likely_causes: Vec<_> = likely_causes
        .iter()
        .flat_map(translate_likely_cause)
        .collect();

    Ok(ErrorDocumentation {
        description: description.clone(),
        short_description: short_description.clone(),
        likely_causes,
    })
}
fn translate_error(
    error: &crate::json::Error,
    ctx: &ErrorTranslationContext,
) -> Result<ErrorDescription, ModelBuildingError> {
    let crate::json::Error {
        name,
        code,
        message,
        bindings: codegen,
        fields,
        doc,
    } = error;
    let transformed_fields: Result<_, _> = fields.iter().map(translate_field).collect();
    let transformed_mappings: TypeBindings<TargetLanguageType> = translate_type_bindings(codegen)?;

    let documentation = if let Some(doc) = doc {
        Some(translate_error_documentation(doc)?)
    } else {
        None
    };
    Ok(ErrorDescription {
        name: name.clone(),
        code: *code,
        message: message.clone(),
        fields: transformed_fields?,
        documentation,
        bindings: transformed_mappings,
        domain: ctx.component_context.domain_name.clone(),
        component: ctx.component_name.clone(),
    })
}

#[derive(Clone)]
struct ComponentTranslationContext {
    domain_name: String,
}

fn translate_component(
    component: &crate::json::Component,
    ctx: &ComponentTranslationContext,
) -> Result<ComponentDescription, ModelBuildingError> {
    let crate::json::Component {
        component_name,
        component_code,
        identifier_encoding,
        description,
        takeFrom,
        errors,
        bindings,
    } = component;
    let domain_name = ctx.domain_name.clone();
    let mut transformed_errors: Vec<ErrorDescription> = errors
        .iter()
        .map(|e| {
            translate_error(
                e,
                &ErrorTranslationContext {
                    component_context: ctx.clone(),
                    component_name: component_name.clone(),
                },
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    for take_from_address in takeFrom {
        match fetch_file(take_from_address) {
            Ok(fetched_file_contents) => {
                //fixme dirty
                let config: Config = serde_json::from_str(&fetched_file_contents)
                    .unwrap_or_else(|_| panic!("Error fetching the file {take_from_address} "));

                let fetched_component: &crate::json::Component = config
                    .get_component(&domain_name, component_name)
                    .ok_or(ModelBuildingError::TakeFrom {
                        address: take_from_address.clone(),
                        inner: TakeFromError::MissingComponent {
                            domain_name: domain_name.clone(),
                            component_name: component_name.clone(),
                        },
                    })?;
                let translated_fetched_component = translate_component(fetched_component, ctx)?;
                transformed_errors.extend(translated_fetched_component.errors);
            }
            Err(e) => {
                return Err(ModelBuildingError::TakeFrom {
                    address: take_from_address.clone(),
                    inner: TakeFromError::IOError(e),
                })
            }
        }
    }

    Ok(ComponentDescription {
        name: component_name.clone(),
        code: *component_code,
        bindings: maplit::hashmap! {
            "rust".into() => bindings.rust.clone(),
        },
        identifier: identifier_encoding.clone(),
        description: description.clone().unwrap_or_default(),
        errors: transformed_errors,
    })
}

fn translate_domain(value: &crate::json::Domain) -> Result<DomainDescription, ModelBuildingError> {
    let crate::json::Domain {
        domain_name,
        domain_code,
        identifier_encoding,
        description,
        components,
        bindings,
    } = value;
    let ctx = ComponentTranslationContext {
        domain_name: domain_name.clone(),
    };
    let transformed_components: Result<Vec<ComponentDescription>, _> = components
        .iter()
        .map(|c| translate_component(c, &ctx))
        .collect();
    Ok(DomainDescription {
        name: domain_name.clone(),
        code: *domain_code,
        identifier: identifier_encoding.clone(),
        description: description.clone().unwrap_or_default(),
        bindings: hashmap! {
            "rust".into() => bindings.rust.clone(),
        },
        components: transformed_components?
            .iter()
            .cloned()
            .map(|c| (c.name.clone(), c))
            .collect(),
    })
}
