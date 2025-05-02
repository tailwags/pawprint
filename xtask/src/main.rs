use anyhow::Result;
use clap::Parser;
use xtask_common::{Cli, Empty, clap};

fn main() -> Result<()> {
    let cli: Cli<Empty> = Cli::parse();

    cli.command.execute("unnamed-js-engine")?;

    Ok(())
}
