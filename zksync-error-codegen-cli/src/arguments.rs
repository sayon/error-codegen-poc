use structopt::StructOpt;

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
}
