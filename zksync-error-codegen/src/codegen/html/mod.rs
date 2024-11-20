pub mod config;
pub mod error;
pub mod files;

use std::path::PathBuf;

use config::HtmlBackendConfig;
use error::GenerationError;
use include_dir::Dir;
use tera::Tera;

use super::Backend;
use super::File;

use crate::model::Model;
use include_dir::include_dir;

pub struct HtmlBackend {
    model: Model,
}

impl HtmlBackend {
    pub fn new(model: Model) -> Self {
        Self { model }
    }
}

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/doc_templates");

impl Backend<HtmlBackendConfig> for HtmlBackend {
    type Error = GenerationError;

    fn get_name() -> &'static str {
        "html-doc"
    }

    fn generate(&mut self, _config: &HtmlBackendConfig) -> Result<Vec<File>, Self::Error> {
        let mut tera = Tera::default();
        for file in TEMPLATES_DIR.files() {
            if let Some(path) = file.path().to_str() {
                if let Ok(contents) = std::str::from_utf8(file.contents()) {
                    tera.add_raw_template(path, contents)?;
                }
            }
        }

        let mut results = vec![];
        // Generate HTML files for each error
        for (domain_name, domain) in &self.model.domains {
            for (component_name, component) in &domain.components {
                for error in &component.errors {
                    // Create context for Tera
                    let mut context = tera::Context::new();
                    context.insert("domain", &domain);
                    context.insert("component", &component);
                    context.insert("error", &error);

                    let content = tera.render("error.html", &context)?;
                    let error_name = &error.name;
                    results.push(File {
                        relative_path: PathBuf::from(format!(
                            "{domain_name}/{component_name}/{error_name}.html"
                        )),
                        content,
                    });
                }
            }
        }

        results.push({
            let mut context = tera::Context::new();
            let errors: Vec<_> = self
                .model
                .domains
                .values()
                .flat_map(|domain| {
                    domain
                        .components
                        .values()
                        .flat_map(|component| &component.errors)
                })
                .collect();

            context.insert("errors", &errors);

            let content = tera.render("index.html", &context)?;
            File {
                relative_path: PathBuf::from("index.html"),
                content,
            }
        });

        Ok(results)
    }

    fn get_language_name() -> &'static str {
        "html"
    }
}