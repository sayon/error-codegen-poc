use cargo::get_resolution_context;
use error::FileFormatError;
use error::LinkError;
use error::LoadError;
use error::ResolutionError;
use link::Link;
use link::{link_matches, parse_link};
use std::path::PathBuf;

pub mod cargo;
pub mod error;
pub mod fetch;
pub mod link;
pub mod builder;

pub enum ErrorBasePart {
    Root(crate::error_database::Root),
    Domain(crate::error_database::Domain),
    Component(crate::error_database::Component),
}

#[derive(Clone, Debug)]
pub struct DescriptionFile {
    pub package: String,
    pub absolute_path: PathBuf,
}

#[derive(Clone, Debug, Default)]
pub struct ResolutionContext {
    pub files: Vec<DescriptionFile>,
}

impl ResolutionContext {
    pub fn find_package(&self, package: &str) -> Option<&DescriptionFile> {
        self.files.iter().find(|df| &df.package == package)
    }
}

enum ResolvedLink {
    DescriptionFile(DescriptionFile),
    LocalPath(PathBuf),
    Url(String),
}

fn resolve(
    query_link: impl Into<String>,
    context: &ResolutionContext,
) -> Result<ResolvedLink, LinkError> {
    match parse_link(query_link)? {
        link @ Link::PackageLink { .. } => {
            if let Some(df) = context.files.iter().find(|file| link_matches(&link, file)) {
                Ok(ResolvedLink::DescriptionFile(df.clone()))
            } else {
                Err(LinkError::FailedResolution(
                    ResolutionError::CargoLinkResolutionError {
                        link,
                        context: context.clone(),
                    },
                ))
            }
        }
        Link::FileLink { path } => Ok(ResolvedLink::LocalPath(path.into())),
        Link::URL { url } => Ok(ResolvedLink::Url(url)),
    }
}

pub fn load(link: impl Into<String>) -> Result<ErrorBasePart, LoadError> {
    let context = get_resolution_context();
    match resolve(link, &context)? {
        ResolvedLink::DescriptionFile(description_file) => {
            let contents = std::fs::read_to_string(&description_file.absolute_path)?;
            load_resolved(&contents)
        }
        ResolvedLink::LocalPath(path) => {
            let contents = fetch::from_fs(&path)?;
            load_resolved(&contents)
        }
        ResolvedLink::Url(url) => {
            let contents = fetch::from_network(&url)?;
            load_resolved(&contents)
        }
    }
}

pub fn load_resolved(contents: &str) -> Result<ErrorBasePart, LoadError> {
    match serde_json::from_str::<crate::error_database::Component>(contents)
        .or(toml::from_str::<crate::error_database::Component>(contents))
    {
        Ok(contents) => Ok(ErrorBasePart::Component(contents)),

        Err(e) => {
            eprintln!("Error: {e}");
            match serde_json::from_str::<crate::error_database::Domain>(contents)
                .or(toml::from_str::<crate::error_database::Domain>(contents))
            {
                Ok(contents) => Ok(ErrorBasePart::Domain(contents)),
                Err(e) => {
                    eprintln!("Error: {e}");
                    match serde_json::from_str::<crate::error_database::Root>(contents)
                        .or(toml::from_str::<crate::error_database::Root>(contents))
                    {
                        Ok(contents) => Ok(ErrorBasePart::Root(contents)),
                        Err(error) => Err(LoadError::FileFormatError(FileFormatError::ParseError(
                            contents.to_owned(),
                            Box::new(error),
                        ))),
                    }
                }
            }
        }
    }
}
