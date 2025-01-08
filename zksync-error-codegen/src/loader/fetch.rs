use reqwest;
use std::fs;

use super::error::LoadError;

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
