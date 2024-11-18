pub mod error;

use error::ModelBuildingError;
use error::TakeFromError;
use maplit::hashmap;

use crate::json::Config;

use super::loader::fetch_file;
use super::ComponentDescription;
use super::DomainDescription;
use super::ErrorDescription;
use super::FieldDescription;
use super::FullyQualifiedTargetLanguageType;
use super::Model;
use super::TargetLanguageType;
use super::TypeBindings;
use super::TypeDescription;
use super::TypeMetadata;

impl TryFrom<&crate::json::ErrorNameMapping> for TypeBindings<TargetLanguageType> {
    type Error = ModelBuildingError;

    fn try_from(value: &crate::json::ErrorNameMapping) -> Result<Self, Self::Error> {
        let mut result: Self = Default::default();
        if let Some(crate::json::ErrorType { name }) = &value.rust {
            result
                .bindings
                .insert("rust".into(), TargetLanguageType { name: name.clone() });
        }
        Ok(result)
    }
}

impl TryFrom<&crate::json::TypeMappings> for TypeBindings<FullyQualifiedTargetLanguageType> {
    type Error = ModelBuildingError;

    fn try_from(value: &crate::json::TypeMappings) -> Result<Self, Self::Error> {
        let mut result: Self = Default::default();
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
}

impl TryFrom<&crate::json::Type> for TypeDescription {
    type Error = ModelBuildingError;

    fn try_from(value: &crate::json::Type) -> Result<Self, Self::Error> {
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
            bindings: codegen.try_into()?,
        })
    }
}

impl TryFrom<&crate::json::Config> for Model {
    type Error = ModelBuildingError;

    fn try_from(value: &crate::json::Config) -> Result<Self, Self::Error> {
        let mut result = Model::default();
        let crate::json::Config { types, domains } = value;
        for t in types {
            result.types.insert(t.name.clone(), t.try_into()?);
        }

        for domain in domains {
            let transformed_domain: DomainDescription = domain.try_into()?;
            result
                .domains
                .insert(transformed_domain.name.clone(), transformed_domain);
        }

        Ok(result)
    }
}

impl TryFrom<&crate::json::Field> for FieldDescription {
    type Error = ModelBuildingError;

    fn try_from(value: &crate::json::Field) -> Result<Self, Self::Error> {
        let crate::json::Field { name, r#type } = value;
        Ok(FieldDescription {
            name: name.clone(),
            r#type: r#type.clone(),
        })
    }
}

impl TryFrom<&crate::json::Error> for ErrorDescription {
    type Error = ModelBuildingError;

    fn try_from(value: &crate::json::Error) -> Result<Self, Self::Error> {
        let crate::json::Error {
            name,
            code,
            message,
            bindings: codegen,
            fields,
            doc: _,
        } = value;
        let transformed_fields: Result<_, _> = fields
            .iter()
            .map(|f| TryInto::<FieldDescription>::try_into(f))
            .collect();
        let transformed_mappings: TypeBindings<TargetLanguageType> = codegen.try_into()?;
        Ok(ErrorDescription {
            name: name.clone(),
            code: code.clone(),
            message: message.clone(),
            fields: transformed_fields?,
            documentation: None, //FIXME
            bindings: transformed_mappings,
        })
    }
}

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
        .map(|e| TryInto::<ErrorDescription>::try_into(e))
        .collect::<Result<Vec<_>, _>>()?;

    for take_from_address in takeFrom {
        match fetch_file(take_from_address) {
            Ok(fetched_file_contents) => {
                //fixme dirty
                let config: Config = serde_json::from_str(&fetched_file_contents)
                    .expect(&format!("Error fetching the file {take_from_address} "));

                let fetched_component: &crate::json::Component = config
                    .get_component(&domain_name, &component_name)
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
        code: component_code.clone(),
        bindings: maplit::hashmap! {
            "rust".into() => bindings.rust.clone(),
        },
        identifier: identifier_encoding.clone(),
        description: description.clone().unwrap_or_default(),
        errors: transformed_errors,
    })
}

impl TryFrom<&crate::json::Domain> for DomainDescription {
    type Error = ModelBuildingError;

    fn try_from(value: &crate::json::Domain) -> Result<Self, Self::Error> {
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
}
