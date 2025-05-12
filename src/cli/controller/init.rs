use crate::{cli::flags::init::InitializeBubu, usecases::init::{bin::bin_usage, lib::lib_usage}};

pub fn handle_init(args: InitializeBubu) {
    let path = args.path.canonicalize().expect("Invalid path");
    let package_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("bubu");

    let edition = args.edition.as_deref().unwrap_or("2024");

    let name = args.name.as_deref().unwrap_or("bubu");

    if args.lib.is_some() {
        lib_usage(package_name, edition, name);
    } else {
        bin_usage(package_name, edition, name);
    }
}