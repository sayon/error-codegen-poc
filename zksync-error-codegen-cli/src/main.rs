pub mod arguments;

use arguments::Arguments;
use zksync_error_codegen::arguments::GenerationArguments;
use zksync_error_codegen::error::ProgramError;
use zksync_error_codegen::load_and_generate;

use structopt::StructOpt as _;

impl From<Arguments> for GenerationArguments {
    fn from(val: Arguments) -> Self {
        let Arguments {
            root: definitions,
            backend,
            verbose,
            output_directory,
            additional_inputs,
        } = val;
        GenerationArguments {
            verbose,
            root_link: definitions,
            outputs: vec![(output_directory.into(), backend)],
            input_links: additional_inputs,
        }
    }
}

fn main_inner(arguments: Arguments) -> Result<(), ProgramError> {
    load_and_generate(arguments.into())
}

fn main() {
    let arguments = Arguments::from_args();

    if let Err(error) = main_inner(arguments) {
        eprintln!("{error:#?}")
    }
}
