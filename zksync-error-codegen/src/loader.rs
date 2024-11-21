use reqwest;
use std::fmt::Display;
use std::fs;

#[derive(Debug)]
pub enum LoadError {
    IOError(std::io::Error),
    NetworkError(reqwest::Error),
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
