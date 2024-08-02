use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "cimengine", bin_name = "cimengine")]
#[command(about = "CIMEngine build tools")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
#[clap(author, version, about)]
pub enum Commands {}
