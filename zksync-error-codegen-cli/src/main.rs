pub mod arguments;

use arguments::Arguments;
use zksync_error_codegen::arguments::GenerationArguments;
use zksync_error_codegen::error::ProgramError;
use zksync_error_codegen::load_and_generate;

use structopt::StructOpt as _;

impl Into<GenerationArguments> for Arguments {
    fn into(self) -> GenerationArguments {
        let Self {
            definitions,
            backend,
            verbose,
            output_directory,
        } = self;
        GenerationArguments {
            verbose,
            root_link: definitions,
            outputs: vec![(output_directory.into(), backend)],

        }
    }
}

fn main_inner(arguments: Arguments) -> Result<(), ProgramError> {
    load_and_generate(arguments.into())
}

fn main() {
    let arguments = Arguments::from_args();

    if let Err(error) = main_inner(arguments) {
        eprintln!("{error:?}")
    }
}
