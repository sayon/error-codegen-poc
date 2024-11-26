use reqwest;
use std::fmt::Display;
use std::fs;

#[derive(Debug)]
pub enum FileFormatError {
    ExpectedFullGotDomain(String),
    ExpectedFullGotComponent(String),
    ParseError(String, Box<dyn std::error::Error>),
}

impl std::fmt::Display for FileFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileFormatError::ExpectedFullGotDomain(path) =>
f.write_fmt(format_args!("File `{path}` contains just an error domain description, but a master error database should describe at least one component.")),

            FileFormatError::ExpectedFullGotComponent(path) =>
                f.write_fmt(format_args!("File `{path}` contains just an error component description, but a master error database should describe at least one domain and one component.")),
            FileFormatError::ParseError(path, error) => f.write_fmt(format_args!("Error parsing file `{path}`: {error}")),
        }
    }
}

#[derive(Debug)]
pub enum LoadError {
    IOError(std::io::Error),
    NetworkError(reqwest::Error),
    FileFormatError(FileFormatError),
}

impl From<FileFormatError> for LoadError {
    fn from(v: FileFormatError) -> Self {
        Self::FileFormatError(v)
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}
impl From<reqwest::Error> for LoadError {
    fn from(v: reqwest::Error) -> Self {
        Self::NetworkError(v)
    }
}

impl From<std::io::Error> for LoadError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

fn fetch_local_file(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

fn fetch_network_file(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    let content = response.text()?;
    Ok(content)
}

pub fn fetch_file(path: &str) -> Result<String, LoadError> {
    Ok(fetch_local_file(path).or(fetch_network_file(path))?)
}

pub enum ErrorBasePart {
    Root(crate::error_database::Root),
    Domain(crate::error_database::Domain),
    Component(crate::error_database::Component),
}
pub fn load(path: &str) -> Result<ErrorBasePart, LoadError> {
    let contents = fetch_file(path)?;
    eprintln!("Trying to load component from {path}");

    match serde_json::from_str::<crate::error_database::Component>(&contents).or(toml::from_str::<
        crate::error_database::Component,
    >(&contents))
    {
        Ok(contents) => {
            eprintln!("Loaded Component from {path}");
            Ok(ErrorBasePart::Component(contents))
        }

        Err(e) => {
            eprintln!("Error: {e}");
            match serde_json::from_str::<crate::error_database::Domain>(&contents)
                .or(toml::from_str::<crate::error_database::Domain>(&contents))
            {
                Ok(contents) => {
                    eprintln!("Loaded Domain from {path}");
                    Ok(ErrorBasePart::Domain(contents))
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    match serde_json::from_str::<crate::error_database::Root>(&contents)
                        .or(toml::from_str::<crate::error_database::Root>(&contents))
                    {
                        Ok(contents) => {
                            eprintln!("Loaded Database from {path}");
                            Ok(ErrorBasePart::Root(contents))
                        }
                        Err(error) => Err(LoadError::FileFormatError(FileFormatError::ParseError(
                            path.to_string(),
                            Box::new(error),
                        ))),
                    }
                }
            }
        }
    }
}
