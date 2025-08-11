use std::{fs, path::PathBuf};

use clap::Parser as _;
use miette::{Context, IntoDiagnostic, Result, miette};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: PathBuf,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let Args { file } = Args::parse();

    let source = fs::read_to_string(&file)
        .into_diagnostic()
        .wrap_err("Failed to read source file")?;
    let source_type =
        SourceType::from_path(&file).map_err(|ext| miette!("Unknown extension {ext}"))?;

    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source, source_type).parse();

    dbg!(ret.program);

    // Report parsing results
    if ret.errors.is_empty() {
        println!("Parsed Successfully.");
    } else {
        for error in ret.errors {
            let error = error.with_source_code(source.clone());
            println!("{error:?}");
        }
        println!("Parsed with Errors.");
    }

    Ok(())
}
