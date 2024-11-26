pub mod config;
pub mod error;


use std::path::PathBuf;

use config::MDBookBackendConfig;
use error::GenerationError;
use include_dir::Dir;
use tera::Tera;

use super::Backend;
use super::File;

use crate::model::structure::flattened::flatten;
use crate::model::structure::flattened::FlatModel;
use crate::model::structure::Model;
use include_dir::include_dir;

pub struct MDBookBackend {
    model: Model,
}

impl MDBookBackend {
    pub fn new(model: Model) -> Self {
        Self { model }
    }
}

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/doc_templates/mdbook");

fn initialize_tera() -> Result<Tera, <MDBookBackend as Backend<MDBookBackendConfig>>::Error> {
    let mut tera = Tera::default();
    for file in TEMPLATES_DIR.files() {
        if let Some(path) = file.path().to_str() {
            if let Ok(contents) = std::str::from_utf8(file.contents()) {
                tera.add_raw_template(path, contents)?;
            }
        }
    }
    Ok(tera)
}

impl MDBookBackend {
    fn generate_toml(&mut self, _config: &MDBookBackendConfig) -> Result<File, GenerationError> {
        Ok(File {
            relative_path: PathBuf::from("book.toml"),
            content: r#"[book]
authors = [
    "Matter Labs",
]
language = "en"
multilingual = false
src = "src"
title = "ZKsync public errors documentation"

"#
                .into(),
        })
    }

    fn generate_summary(
        &mut self,
        tera: &Tera,
        model: &FlatModel,
        _config: &MDBookBackendConfig,
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("domains", &model.domains.values().collect::<Vec<_>>());
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        let content = tera.render("SUMMARY.md", &context)?;

        Ok(File {
            relative_path: PathBuf::from("src/SUMMARY.md"),
            content,
        })
    }

    fn generate_component(
        &mut self,
        tera: &Tera,
        component: &crate::model::structure::flattened::ComponentMetadata,
        model: &FlatModel,
        _config: &MDBookBackendConfig,
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("component", component);
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        let content = tera.render("component.md", &context)?;
        let domain_name = &component.domain_name;
        let component_name = &component.name;

        Ok(File {
            relative_path: PathBuf::from(format!("src/domains/{domain_name}/{component_name}/README.md")),
            content,
        })
    }
    fn generate_domain(
        &mut self,
        tera: &Tera,
        domain: &crate::model::structure::flattened::DomainMetadata,
        model: &FlatModel,
        _config: &MDBookBackendConfig,
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("domain", domain);
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        let content = tera.render("domain.md", &context)?;
        let domain_name = &domain.name;

        Ok(File {
            relative_path: PathBuf::from(format!("src/domains/{domain_name}/README.md")),
            content,
        })
    }

    fn generate_error(
        &mut self,
        tera: &Tera,
        domain: &crate::model::structure::flattened::DomainMetadata,
        component: &crate::model::structure::flattened::ComponentMetadata,
        error: &crate::model::structure::flattened::ErrorDescription,
        model: &FlatModel,
        _config: &MDBookBackendConfig,
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("domain", domain);
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        context.insert("error", error);
        let content = tera.render("error.md", &context)?;
        let domain_name = &domain.name;
        let component_name = &component.name;
        let error_name = &error.name;

        Ok(File {
            relative_path: PathBuf::from(format!("src/domains/{domain_name}/{component_name}/{error_name}.md")),
            content,
        })
    }
}
impl Backend<MDBookBackendConfig> for MDBookBackend {
    type Error = GenerationError;

    fn get_name() -> &'static str {
        "markdown-mdbook"
    }

    fn generate(&mut self, _config: &MDBookBackendConfig) -> Result<Vec<File>, Self::Error> {
        let tera = initialize_tera()?;

        let model = flatten(&self.model);
        let mut results = vec![
            self.generate_summary(&tera, &model, _config)?,
            self.generate_toml(_config)?,
        ];

        for domain in model.domains.values() {
            results.push(self.generate_domain(&tera, domain, &model,_config)? );
            for component in model.components.values() {
                if component.domain_name == domain.name {
                    results.push(self.generate_component(&tera, &component, &model, _config)?);
                    for error in model.errors.values() {
                        if error.component == component.name {
                            results.push(self.generate_error(&tera, &domain, &component, &error, &model, _config)?);
                        }
                    }
                }
            }
        }
        // {
        //      let mut context = tera::Context::new();
        //      context.insert("domains", &model.domains.values().collect::<Vec<_>>());
        //     let content = tera.render("SUMMARY.md", &context)?;

        //      results.push(File {
        //          relative_path: PathBuf::from(format!(
        //              "domains/{domain_name}.md
        //          )),
        //          content,
        //     });
        // }
        // for error in model.errors.values() {
        //     // Create context for Tera
        //     let mut context = tera::Context::new();
        //     context.insert("error", &error);

        //     let content = tera.render("error.md", &context)?;
        //     let domain_name = &error.domain;
        //     let component_name = &error.component;
        //     let error_name = &error.name;
        //     results.push(File {
        //         relative_path: PathBuf::from(format!(
        //             "{domain_name}/{component_name}/{error_name}.Markdown"
        //         )),
        //         content,
        //     });
        // }

        // results.push({
        //     let mut context = tera::Context::new();

        //     context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        //     context.insert("components", &model.components.values().collect::<Vec<_>>());
        //     context.insert("domains", &model.domains.values().collect::<Vec<_>>());

        //     let content = tera.render("index.Markdown", &context)?;
        //     File {
        //         relative_path: PathBuf::from("index.Markdown"),
        //         content,
        //     }
        // });

        Ok(results)
    }

    fn get_language_name() -> &'static str {
        "markdown"
    }
}
