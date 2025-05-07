use clap::{Parser, Subcommand};
use crate::cli::flags::init::InitializeBubu;

#[derive(Debug, Parser)]
#[clap(
    author = "Bilal Khan",
    version,
    about = "🦀 Bubu is a local package manager and build tool for Rust projects. It helps you create, compile, run, and manage Rust code efficiently without requiring internet access or remote APIs."
)]
pub struct Bubu {
    #[clap(subcommand)]
    pub command: BubuCommand,
}

#[derive(Debug, Subcommand)]
pub enum BubuCommand {
    /// Initialize a new Rust project with standard layout
    Init(InitializeBubu),

    /// Build the Rust project into an executable binary
    Build,

    /// Run the compiled Rust binary
    Run,

    /// Check the project for compile errors without producing an executable
    Check,

    /// Generate local documentation for your Rust project
    Doc,

    /// List local dependencies or packages
    List,

    /// Install a local Rust package into your workspace
    Install,

    /// Run unit or integration tests in the current Rust project
    Test,

    /// Delete target directory and other temporary build files
    Clean,

    /// Create a compressed package archive of your project
    Package,

    /// Update the lockfile from local dependencies (no registry access)
    UpdateLock,

    /// Format your Rust source files using rustfmt standards
    Fmt,

    /// Lint your Rust code for warnings or style issues
    Lint,

    /// Show stats like lines of code, file count, and function count
    Stats,

    /// Show current user or author info from your config
    Whoami,

    /// Run performance benchmarks on your Rust code
    Bench,

    /// Display a dependency tree from local `mypkg.toml` and `mypkg.lock`
    Tree,

    /// Publish the project to a local package directory
    Publish
}