pub mod error;

use std::collections::HashMap;

use error::MissingComponent;
use error::ModelBuildingError;
use error::TakeFromError;
use maplit::hashmap;

use crate::loader::load;
use crate::loader::ErrorBasePart;

use super::merger::Merge as _;
use super::structure::ComponentDescription;
use super::structure::DomainDescription;
use super::structure::ErrorDescription;
use super::structure::ErrorDocumentation;
use super::structure::FieldDescription;
use super::structure::FullyQualifiedTargetLanguageType;
use super::structure::LikelyCause;
use super::structure::Model;
use super::structure::TargetLanguageType;
use super::structure::TypeBindings;
use super::structure::TypeDescription;
use super::structure::TypeMetadata;
use super::structure::VersionedOwner;

pub struct ModelTranslationContext<'a> {
    pub origin: &'a str,
}
struct TypeTranslationContext<'a> {
    pub type_name: &'a str,
    pub parent: &'a ModelTranslationContext<'a>,
}
struct DomainTranslationContext<'a> {
    pub domain_name: &'a str,
    pub parent: &'a ModelTranslationContext<'a>,
}

impl<'a> DomainTranslationContext<'a> {
    fn get_domain(&self) -> String {
        self.domain_name.to_string()
    }
}
struct ComponentTranslationContext<'a> {
    pub component_name: &'a str,
    pub parent: &'a DomainTranslationContext<'a>,
}

impl<'a> ComponentTranslationContext<'a> {
    fn get_component(&self) -> String {
        self.component_name.to_string()
    }
    fn get_domain(&self) -> String {
        self.parent.get_domain()
    }
}

struct ErrorTranslationContext<'a> {
    pub error_name: &'a str,
    pub parent: &'a ComponentTranslationContext<'a>,
}
impl<'a> ErrorTranslationContext<'a> {
    fn get_component(&self) -> String {
        self.parent.get_component()
    }
    fn get_domain(&self) -> String {
        self.parent.get_domain()
    }
    fn get_name(&self) -> String {
        self.error_name.to_string()
    }
}

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

fn translate_type(
    value: &crate::json::Type,
    _ctx: &TypeTranslationContext,
) -> Result<TypeDescription, ModelBuildingError> {
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

pub fn translate_model(
    model: &crate::json::Root,
    ctx: ModelTranslationContext<'_>,
) -> Result<Model, ModelBuildingError> {
    let mut result = Model::default();
    let crate::json::Root { types, domains } = model;
    for t in types {
        let ctx = TypeTranslationContext {
            type_name: &t.name,
            parent: &ctx,
        };
        result
            .types
            .insert(t.name.clone(), translate_type(t, &ctx)?);
    }

    for domain in domains {
        let ctx = DomainTranslationContext {
            domain_name: &domain.domain_name,
            parent: &ctx,
        };
        let transformed_domain: DomainDescription = translate_domain(domain, &ctx)?;
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
        bindings,
        fields,
        doc,
    } = error;
    let transformed_fields: Result<_, _> = fields.iter().map(translate_field).collect();
    let transformed_bindings: TypeBindings<TargetLanguageType> = translate_type_bindings(bindings)?;

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
        bindings: transformed_bindings,
        domain: ctx.get_domain(),
        component: ctx.get_component(),
    })
}

// match serde_json::from_str::<Config>(&fetched_file_contents) {
//     Ok(config) => {
//         let fetched_component: &crate::json::Component = config
//             .get_component(&domain_name, component_name)
//             .ok_or(ModelBuildingError::TakeFrom {
//                 address: take_from_address.clone(),
//                 inner: TakeFromError::MissingComponent {
//                     domain_name: domain_name.clone(),
//                     component_name: component_name.clone(),
//                 },
//             })?;
//         let translated_fetched_component =
//             translate_component(fetched_component, ctx)?;
//         transformed_errors.extend(translated_fetched_component.errors);
//     }
//     Err(e) => {
//         return Err(ModelBuildingError::TakeFrom {
//             address: take_from_address.clone(),
//             inner: e.into(),
//         })
//     }
// }
//         }
//         Err(e) => {
//             return Err(ModelBuildingError::TakeFrom {
//                 address: take_from_address.clone(),
//                 inner: TakeFromError::IOError(e),
//             })
//         }
//     }
// }

fn fetch_component<'a>(
    address: &str,
    ctx: &'a ComponentTranslationContext<'a>,
) -> Result<ComponentDescription, TakeFromError> {
    let error_base = load(address)?;
    let component: crate::json::Component = match error_base {
        ErrorBasePart::Root(root) => root
            .get_component(ctx.parent.domain_name, ctx.component_name)
            .cloned()
            .ok_or(MissingComponent {
                domain_name: ctx.get_domain(),
                component_name: ctx.get_component(),
            })?,
        ErrorBasePart::Domain(_domain) => todo!(),
        ErrorBasePart::Component(component) => component,
    };
    translate_component(&component, ctx).map_err(Into::<TakeFromError>::into)
}
fn translate_component<'a>(
    component: &crate::json::Component,
    ctx: &'a ComponentTranslationContext<'a>,
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

    let mut transformed_errors = Vec::default();
    for error in errors {
        let ctx = ErrorTranslationContext {
            error_name: &error.name,
            parent: ctx,
        };
        transformed_errors.push(translate_error(error, &ctx)?);
    }

    let mut result = ComponentDescription {
        name: component_name.clone(),
        code: *component_code,
        bindings: maplit::hashmap! {
            "rust".into() => bindings.rust.clone(),
        },
        identifier: identifier_encoding.clone(),
        description: description.clone().unwrap_or_default(),
        errors: transformed_errors,
    };
    for take_from_address in takeFrom {
        let component = fetch_component(take_from_address, ctx)
            .map_err(|e| e.from_address(take_from_address))?;
        result
            .merge(&component)
            .map_err(|e| TakeFromError::MergeError(e).from_address(take_from_address))?;
    }

    Ok(result)
}

fn translate_domain<'a>(
    value: &crate::json::Domain,
    ctx: &'a DomainTranslationContext<'a>,
) -> Result<DomainDescription, ModelBuildingError> {
    let crate::json::Domain {
        domain_name,
        domain_code,
        identifier_encoding,
        description,
        components,
        bindings,
    } = value;
    let mut new_components: HashMap<_, _> = HashMap::default();

    for component in components {
        let ctx = ComponentTranslationContext {
            component_name: &component.component_name,
            parent: ctx,
        };

        let translated_component = translate_component(component, &ctx)?;
        new_components.insert(translated_component.name.clone(), translated_component);
    }
    Ok(DomainDescription {
        name: domain_name.clone(),
        code: *domain_code,
        identifier: identifier_encoding.clone(),
        description: description.clone().unwrap_or_default(),
        bindings: hashmap! {
            "rust".into() => bindings.rust.clone(),
        },
        components: new_components,
    })
}
