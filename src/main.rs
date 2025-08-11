use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser as _;
use oxc::{allocator::Allocator, parser::Parser, span::SourceType};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: PathBuf,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let Args { file } = Args::parse();

    let source = fs::read_to_string(&file)?;
    let source_type = SourceType::from_path(&file)?;
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
