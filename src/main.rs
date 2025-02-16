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
    let Args { file } = Args::parse();

    let source = fs::read_to_string(&file)?;
    let source_type = SourceType::from_path(&file)?;
    let allocator = Allocator::default();
    let ast = Parser::new(&allocator, &source, source_type).parse();

    dbg!(ast.program);

    Ok(())
}
