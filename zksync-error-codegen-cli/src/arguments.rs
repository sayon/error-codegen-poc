use structopt::StructOpt;
use std::error::Error;

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Generator of the error handling code in ZKsync components.",
    global_settings = &[structopt::clap::AppSettings::ArgRequiredElseHelp],
)]
pub struct Arguments {
    /// Link to the master JSON file.
    #[structopt(long = "root-definitions")]
    pub root: String,

    /// Links to additional JSON file.
    #[structopt(long = "additional-definitions")]
    pub additional_inputs: Vec<String>,

    /// Selected backend.
    #[structopt(long = "backend", possible_values=&["rust", "doc-html", "markdown-mdbook"])]
    pub backend: zksync_error_codegen::arguments::Backend,

    /// Be verbose and produce debug output.
    #[structopt(long = "verbose")]
    pub verbose: bool,

    /// Output files in this directory.
    #[structopt(long = "output", default_value = "zksync-error")]
    pub output_directory: String,

    /// Backend-specific arguments.
    #[structopt(long = "backend-arg", parse(try_from_str = parse_key_val),)]
    pub backend_args: Vec<(String,String)>,
}
