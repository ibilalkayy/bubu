mod cli;
mod usecases;
mod common;

use clap::Parser;
use crate::cli::{
    command::command::Bubu,
    controller::cli::handle_commands
};

fn main() {
    let args: Bubu = Bubu::parse();
    handle_commands(args);
}