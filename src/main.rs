use clap::{App, Arg, ArgMatches, SubCommand};
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use std::io;

pub fn make_app() -> App<'static, 'static> {
    App::new("nop-processor")
        .about("Why tf are all the preprocessors failing")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

pub struct NopPreprocessor;

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> Result<(), Error> {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(&renderer);
    if supported {
        Ok(())
    } else {
        Err(Error::msg("Unsupported renderer"))
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}

fn try_main() -> Result<(), Error> {
    let matches = make_app().get_matches();
    let pre = NopPreprocessor;

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&pre, sub_args)
    } else {
        handle_preprocessing(&pre)
    }
}

fn main() {
    if let Err(_) = try_main() {
        panic!("Failed to run nop-preprocessor");
    }
}

impl Preprocessor for NopPreprocessor {
    fn name(&self) -> &str {
        "nop-preprocessor"
    }

    fn run(&self, _: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        Ok(book)
    }

    fn supports_renderer(&self, _: &str) -> bool {
        true
    }
}
