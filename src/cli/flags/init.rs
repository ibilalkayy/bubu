use clap::Args;

#[derive(Debug, Args)]
pub struct InitializeBubu {
    #[clap(short, long)]
    pub bin: Option<String>,

    #[clap(short, long)]
    pub lib: Option<String>
}