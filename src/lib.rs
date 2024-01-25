use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {}

#[derive(Debug, Clone, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

pub fn get_args() -> Result<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> Result<()> {
    dbg!(args);
    Ok(())
}
