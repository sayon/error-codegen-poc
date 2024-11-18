use reqwest;
use std::error::Error;
use std::fs;

fn fetch_local_file(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

fn fetch_network_file(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let content = response.text()?;
    Ok(content)
}

pub fn fetch_file(path: &str) -> Result<String, Box<dyn Error>> {
    if let Ok(local) = fetch_local_file(path) {
        Ok(local)
    } else {
        fetch_network_file(path)
    }
}
