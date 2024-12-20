pub mod error;

use std::collections::HashMap;
use std::rc::Rc;

use error::MissingComponent;
use error::ModelBuildingError;
use error::TakeFromError;
use maplit::hashmap;

use crate::loader::load;
use crate::loader::ErrorBasePart;

use super::merger::Merge as _;
use super::structure::ComponentDescription;
use super::structure::ComponentMetadata;
use super::structure::DomainDescription;
use super::structure::DomainMetadata;
use super::structure::ErrorDescription;
use super::structure::ErrorDocumentation;
use super::structure::ErrorName;
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
    pub parent: &'a ModelTranslationContext<'a>,
}

struct ComponentTranslationContext<'a> {
    pub domain: Rc<DomainMetadata>,
    pub parent: &'a DomainTranslationContext<'a>,
}

impl<'a> ComponentTranslationContext<'a> {
    fn get_domain(&self) -> String {
        self.domain.name.to_string()
    }
}

struct ErrorTranslationContext<'a> {
    pub component: Rc<ComponentMetadata>,
    pub parent: &'a ComponentTranslationContext<'a>,
}
impl<'a> ErrorTranslationContext<'a> {
    fn get_component(&self) -> String {
        self.component.name.to_string()
    }
    fn get_domain(&self) -> String {
        self.parent.get_domain()
    }
}

fn translate_type_bindings(
    value: &crate::error_database::ErrorNameMapping,
    error_name: &ErrorName,
) -> Result<TypeBindings<TargetLanguageType>, ModelBuildingError> {
    let mut result = TypeBindings::<TargetLanguageType>::default();
    let rust_name = match &value.rust {
        Some(crate::error_database::ErrorType { name }) => name,
        None => error_name,
    }
    .to_string();
    let typescript_name = match &value.typescript {
        Some(crate::error_database::ErrorType { name }) => name,
        None => error_name,
    }
    .to_string();

    result
        .bindings
        .insert("rust".into(), TargetLanguageType { name: rust_name });
    result.bindings.insert(
        "typescript".into(),
        TargetLanguageType {
            name: typescript_name,
        },
    );
    Ok(result)
}

fn translate_type_mappings(
    value: &crate::error_database::TypeMappings,
) -> Result<TypeBindings<FullyQualifiedTargetLanguageType>, ModelBuildingError> {
    let mut result: TypeBindings<FullyQualifiedTargetLanguageType> = Default::default();
    if let Some(crate::error_database::FullyQualifiedType { name, path }) = &value.rust {
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
    value: &crate::error_database::Type,
    _ctx: &TypeTranslationContext,
) -> Result<TypeDescription, ModelBuildingError> {
    let crate::error_database::Type {
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
    model: &crate::error_database::Root,
    ctx: ModelTranslationContext<'_>,
) -> Result<Model, ModelBuildingError> {
    let mut result = Model::default();
    let crate::error_database::Root { types, domains } = model;
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
        let ctx = DomainTranslationContext { parent: &ctx };
        let transformed_domain: DomainDescription = translate_domain(domain, &ctx)?;
        result
            .domains
            .insert(transformed_domain.meta.name.clone(), transformed_domain);
    }

    Ok(result)
}

fn translate_field(
    value: &crate::error_database::Field,
) -> Result<FieldDescription, ModelBuildingError> {
    let crate::error_database::Field { name, r#type } = value;
    Ok(FieldDescription {
        name: name.clone(),
        r#type: r#type.clone(),
    })
}

fn translate_likely_cause(
    lc: &crate::error_database::LikelyCause,
) -> Result<LikelyCause, ModelBuildingError> {
    let crate::error_database::LikelyCause {
        cause,
        fixes,
        report,
        owner: crate::error_database::VersionedOwner { name, version },
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
    doc: &crate::error_database::ErrorDocumentation,
) -> Result<ErrorDocumentation, ModelBuildingError> {
    let &crate::error_database::ErrorDocumentation {
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
    error: &crate::error_database::Error,
    ctx: &ErrorTranslationContext,
) -> Result<ErrorDescription, ModelBuildingError> {
    let crate::error_database::Error {
        name,
        code,
        message,
        bindings,
        fields,
        doc,
    } = error;
    let transformed_fields: Result<_, _> = fields.iter().map(translate_field).collect();
    let transformed_bindings: TypeBindings<TargetLanguageType> =
        translate_type_bindings(bindings, &error.name)?;

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
        domain: ctx.parent.domain.clone(),
        component: ctx.component.clone(),
    })
}

fn fetch_named_component<'a>(
    address: &str,
    name: &str,
    ctx: &'a ComponentTranslationContext<'a>,
) -> Result<ComponentDescription, TakeFromError> {
    let error_base = load(address)?;
    let component: crate::error_database::Component = match error_base {
        ErrorBasePart::Root(root) => {
            root.get_component(&ctx.domain.name, name)
                .cloned()
                .ok_or(MissingComponent {
                    domain_name: ctx.get_domain(),
                    component_name: name.to_string(),
                })?
        }
        ErrorBasePart::Domain(domain) => domain
            .components
            .iter()
            .find(|c| c.component_name == name)
            .cloned()
            .ok_or(MissingComponent {
                domain_name: ctx.get_domain(),
                component_name: name.to_string(),
            })?,
        ErrorBasePart::Component(component) => {
            if component.component_name == name {
                component
            } else {
                return Err(MissingComponent {
                    domain_name: ctx.get_domain(),
                    component_name: name.to_string(),
                }
                .into());
            }
        }
    };
    translate_component(&component, ctx).map_err(Into::<TakeFromError>::into)
}
fn translate_component<'a>(
    component: &crate::error_database::Component,
    ctx: &'a ComponentTranslationContext<'a>,
) -> Result<ComponentDescription, ModelBuildingError> {
    let crate::error_database::Component {
        component_name,
        component_code,
        identifier_encoding,
        description,
        takeFrom,
        errors,
        bindings,
    } = component;

    let component_meta: Rc<ComponentMetadata> = Rc::new(ComponentMetadata {
        name: component_name.clone(),
        code: *component_code,
        bindings: maplit::hashmap! {
            "rust".into() => bindings.rust.clone().unwrap_or(component_name.clone()),
            "typescript".into() => bindings.typescript.clone().unwrap_or(component_name.clone()),
        },
        identifier: identifier_encoding
            .clone()
            .unwrap_or(component_name.clone()),
        description: description.clone().unwrap_or_default(),
    });
    let mut transformed_errors = Vec::default();
    for error in errors {
        let ctx = ErrorTranslationContext {
            parent: ctx,
            component: component_meta.clone(),
        };
        transformed_errors.push(translate_error(error, &ctx)?);
    }

    let mut result = ComponentDescription {
        meta: component_meta,
        errors: transformed_errors,
    };
    for take_from_address in takeFrom {
        let component = fetch_named_component(take_from_address, component_name, ctx)
            .map_err(|e| e.from_address(take_from_address))?;
        result
            .merge(&component)
            .map_err(|e| TakeFromError::MergeError(e).from_address(take_from_address))?;
    }

    Ok(result)
}

fn translate_domain<'a>(
    value: &crate::error_database::Domain,
    ctx: &'a DomainTranslationContext<'a>,
) -> Result<DomainDescription, ModelBuildingError> {
    let crate::error_database::Domain {
        domain_name,
        domain_code,
        identifier_encoding,
        description,
        components,
        bindings,
    } = value;
    let mut new_components: HashMap<_, _> = HashMap::default();
    let metadata = Rc::new(DomainMetadata {
        name: domain_name.clone(),
        code: *domain_code,
        identifier: identifier_encoding.clone().unwrap_or(domain_name.clone()),
        description: description.clone().unwrap_or_default(),
        bindings: hashmap! {
            "rust".into() => bindings.rust.clone().unwrap_or(domain_name.clone()),
            "typescript".into() => bindings.typescript.clone().unwrap_or(domain_name.clone()),
        },
    });
    for component in components {
        let ctx = ComponentTranslationContext {
            domain: metadata.clone(),
            parent: ctx,
        };

        let translated_component = translate_component(component, &ctx)?;
        new_components.insert(translated_component.meta.name.clone(), translated_component);
    }
    Ok(DomainDescription {
        meta: metadata,
        components: new_components,
    })
}
