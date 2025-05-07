use crate::cli::command::command::{Bubu, BubuCommand};
use super::init::handle_init;

pub fn handle_commands(args: Bubu) {
    match args.command {
        BubuCommand::Init(details) => handle_init(details),
        BubuCommand::Build => println!("Building the Rust project..."),
        BubuCommand::Run => println!("Running the Rust project..."),
        BubuCommand::Check => println!("Checking the Rust code for errors..."),
        BubuCommand::Doc => println!("Generating Rust documentation..."),
        BubuCommand::List => println!("Listing local packages..."),
        BubuCommand::Install => println!("Installing a local Rust package..."),
        BubuCommand::Test => println!("Running tests..."),
        BubuCommand::Clean => println!("Cleaning build artifacts..."),
        BubuCommand::Package => println!("Packaging the Rust project..."),
        BubuCommand::UpdateLock => println!("Updating the dependency lockfile..."),
        BubuCommand::Fmt => println!("Formatting source code..."),
        BubuCommand::Lint => println!("Linting the Rust code..."),
        BubuCommand::Stats => println!("Gathering code statistics..."),
        BubuCommand::Whoami => println!("Showing current user info..."),
        BubuCommand::Bench => println!("Running benchmarks..."),
        BubuCommand::Tree => println!("Displaying dependency tree..."),
        BubuCommand::Publish => println!("Publishing to local package registry..."),
    }    
}