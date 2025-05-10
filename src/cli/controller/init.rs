use crate::{cli::flags::init::InitializeBubu, usecases::init::{bin::bin_usage, lib::lib_usage}};

pub fn handle_init(args: InitializeBubu) {
    let path = args.path.canonicalize().expect("Invalid path");
    let package_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("bubu");

    println!("{:?}", package_name);

    let edition = args.edition.as_deref().unwrap_or("2024");

    if args.lib.is_some() {
        lib_usage(package_name, edition);
    } else {
        bin_usage(package_name, edition);
    }
}