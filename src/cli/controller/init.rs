use crate::{cli::flags::init::InitializeBubu, usecases::init::{bin::bin_usage, lib::lib_usage}};

pub fn handle_init(args: InitializeBubu) {
    if let Some(binary) = args.bin {
        bin_usage(&binary);
    }

    if let Some(library) = args.lib {
        lib_usage(&library);
    }
}