use std::str::FromStr;

use structopt::StructOpt;

#[derive(Debug)]
pub enum Backend {
    Doc,
    Rust,
}

impl FromStr for Backend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "doc" => Ok(Backend::Doc),
            "rust" => Ok(Backend::Rust),
            _ => Err("Unrecognized backend".into()),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Generator of the error handling code in ZKsync components",
    global_settings = &[structopt::clap::AppSettings::ArgRequiredElseHelp],
)]
pub struct Arguments {
    /// Path to the master JSON file.
    #[structopt(long = "definitions")]
    pub definitions: String,

    /// Selected backend.
    #[structopt(long = "backend", possible_values=&["rust", "doc"])]
    pub backend: Backend,

    /// Be verbose and produce debug output.
    #[structopt(long = "verbose")]
    pub verbose: bool,
}
