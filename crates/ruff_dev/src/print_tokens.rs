//! Print the token stream for a given Python file.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use rustpython_parser::{lexer, Mode};

#[derive(clap::Args)]
pub struct Args {
    /// Python file for which to generate the AST.
    #[arg(required = true)]
    file: PathBuf,
}

pub fn main(args: &Args) -> Result<()> {
    let contents = fs::read_to_string(&args.file)?;
    for (tok, range) in lexer::lex(&contents, Mode::Module) {
        println!(
            "{start:#?} {tok:#?} {end:#?}",
            start = range.start(),
            end = range.end()
        );
    }
    Ok(())
}
