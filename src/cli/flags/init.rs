use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
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
    pub name: Option<String>
}