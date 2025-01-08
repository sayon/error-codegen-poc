use std::path::PathBuf;

pub struct GenerationArguments {
    pub verbose: bool,
    pub root_link: String,
    pub outputs: Vec<(PathBuf, Backend)>,
}

#[derive(Debug)]
pub enum Backend {
    DocHtml,
    Rust,
    MDBook,
}

impl std::str::FromStr for Backend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "doc-html" => Ok(Backend::DocHtml),
            "rust" => Ok(Backend::Rust),
            "markdown-mdbook" => Ok(Backend::MDBook),
            _ => Err("Unrecognized backend".into()),
        }
    }
}
