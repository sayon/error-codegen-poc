
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
pub struct FullyQualifiedTargetLanguageType {
    pub name: String,
    pub path: String,
}
pub struct TargetLanguageType {
    pub name: String,
}
pub struct TypeMetadata {
    pub description: String,
}
pub struct TypeBindings<T> {
    pub bindings: HashMap<LanguageName, T>,
}
pub struct TypeDescription {
    pub name: TypeName,
    pub meta: TypeMetadata,
    pub bindings: TypeBindings<FullyQualifiedTargetLanguageType>,
}
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
pub struct DomainDescription {
    pub name: DomainName,
    pub code: DomainCode,
    pub bindings: HashMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
    pub components: HashMap<ComponentName, ComponentDescription>,
}
pub struct ComponentDescription {
    pub name: ComponentName,
    pub code: ComponentCode,
    pub bindings: HashMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
    pub errors: Vec<ErrorDescription>,
}
pub struct ErrorDescription {
    pub name: ErrorName,
    pub code: ErrorCode,
    pub message: ErrorMessageTemplate,
    pub fields: Vec<FieldDescription>,
    pub documentation: Option<ErrorDocumentation>,
    pub bindings: TypeBindings<TargetLanguageType>,
}
pub struct FieldDescription {
    pub name: FieldName,
    pub r#type: TypeName,
}
pub struct ErrorDocumentation {
    pub description: String,
    pub short_description: Option<String>,
    pub likely_causes: Vec<LikelyCause>,
}
pub struct LikelyCause {
    pub cause: String,
    pub fixes: Vec<String>,
    pub report: String,
    pub owner: VersionedOwner,
    pub references: Vec<String>,
}
pub struct VersionedOwner {
    pub name: String,
    pub version: Semver,
}
