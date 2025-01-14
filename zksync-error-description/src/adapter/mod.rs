use zksync_error_model::unpacked as inner;
use crate::model;

impl Into<model::TargetLanguageType> for inner::TargetLanguageType
{
    fn into(self) -> model::TargetLanguageType {
        let inner::TargetLanguageType{ name, path } = self;
        model::TargetLanguageType { name, path }

    }
}

impl Into<model::TypeMetadata> for inner::TypeMetadata {
    fn into(self) -> model::TypeMetadata {
        let inner::TypeMetadata { description } = self;
        model::TypeMetadata { description }
    }
}

impl Into<model::TypeDescription> for inner::TypeDescription {
    fn into(self) -> model::TypeDescription {
        let inner::TypeDescription { name, meta, bindings } = self;
        model::TypeDescription {
            name,
            meta: meta.into(),
            bindings: bindings.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl Into<model::DomainMetadata> for inner::DomainMetadata {
    fn into(self) -> model::DomainMetadata {
        let inner::DomainMetadata { name, code, components, bindings, identifier, description } = self;
        model::DomainMetadata {
            name,
            code,
            components,
            bindings,
            identifier,
            description,
        }
    }
}

impl Into<model::ErrorHierarchy> for inner::UnpackedModel {
    fn into(self) -> model::ErrorHierarchy {
       let inner::UnpackedModel { types, domains, components, errors } = self;
        model::ErrorHierarchy {
            types: types.into_iter().map(|(k, v)| (k, v.into())).collect(),
            domains: domains.into_iter().map(|(k, v)| (k, v.into())).collect(),
            components: components.into_iter().map(|(k, v)| (k, v.into())).collect(),
            errors: errors.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl Into<model::ComponentMetadata> for inner::ComponentMetadata {
    fn into(self) -> model::ComponentMetadata {
        let inner::ComponentMetadata { name, code, domain_name, bindings, identifier, description } = self;
        model::ComponentMetadata {
            name,
            code,
            domain_name,
            bindings,
            identifier,
            description,
        }
    }
}

impl Into<model::ErrorDescription>for inner::ErrorDescription {
    fn into(self) -> model::ErrorDescription {
        let inner::ErrorDescription { domain, component, name, code, identifier, message, fields, documentation, bindings } = self;
        model::ErrorDescription {
            domain,
            component,
            name,
            code,
            identifier,
            message,
            fields: fields.into_iter().map(|f| f.into()).collect(),
            documentation: documentation.into(),
            bindings: bindings.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl Into<model::FieldDescription> for inner::FieldDescription {
    fn into(self) -> model::FieldDescription {
        let inner::FieldDescription { name, r#type } = self;
        model::FieldDescription { name, r#type }
    }
}

impl Into<model::ErrorDocumentation> for inner::ErrorDocumentation {
    fn into(self) -> model::ErrorDocumentation {
        let inner::ErrorDocumentation { description, short_description, likely_causes } = self;
        model::ErrorDocumentation {
            description,
            short_description,
            likely_causes: likely_causes.into_iter().map(|lc| lc.into()).collect(),
        }
    }
}

impl Into<model::LikelyCause> for inner::LikelyCause {
    fn into(self) -> model::LikelyCause {
        let inner::LikelyCause { cause, fixes, report, owner, references } = self;
        model::LikelyCause {
            cause,
            fixes,
            report,
            owner: owner.into(),
            references,
        }
    }
}

impl Into<model::VersionedOwner> for inner::VersionedOwner {
    fn into(self) -> model::VersionedOwner {
        let inner::VersionedOwner { name, version } = self;
        model::VersionedOwner { name, version }
    }
}
