use std::path::PathBuf;
use strum_macros::Display;

use clap::{Args, ValueEnum};

#[derive(Clone, Debug, Args)]
pub struct InitializeBubu {
    #[clap(default_value = ".")]
    pub path: PathBuf,
    
    #[clap(short, long)]
    pub bin: Option<String>,

    #[clap(short, long)]
    pub lib: Option<String>,

    #[clap(short, long)]
    pub edition: Option<String>,

    #[clap(short, long)]
    pub name: Option<String>,

    #[clap(value_enum, short, long)]
    pub vcs: VCS
}

#[derive(Clone, Debug, ValueEnum, Display)]
pub enum VCS {
    Git,
}