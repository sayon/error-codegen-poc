use std::str::FromStr;

use structopt::StructOpt;

#[derive(Debug)]
pub enum Backend {
    DocHtml,
    Rust,
    MDBook,
}

impl FromStr for Backend {
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

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Generator of the error handling code in ZKsync components.",
    global_settings = &[structopt::clap::AppSettings::ArgRequiredElseHelp],
)]
pub struct Arguments {
    /// Path to the master JSON file.
    #[structopt(long = "definitions")]
    pub definitions: String,

    /// Selected backend.
    #[structopt(long = "backend", possible_values=&["rust", "doc-html", "markdown-mdbook"])]
    pub backend: Backend,

    /// Be verbose and produce debug output.
    #[structopt(long = "verbose")]
    pub verbose: bool,

    /// Output files in this directory.
    #[structopt(long = "output", default_value = "zksync-error")]
    pub output_directory: String,
}
