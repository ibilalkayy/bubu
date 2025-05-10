use std::{
    fs::File,
    io::Write, path::Path
};

pub fn create_manifest(package_name: &str, edition: &str) {
    let is_present = Path::new("Bubu.toml").exists();
    if is_present {
        eprintln!("error: `cargo init` cannot be run on existing Cargo packages");
        return;
    }

    let mut data_file = File::create("Bubu.toml").expect("Failed to create the manifest file");

    let file_content = format!(
        r#"[package]
name = "{package_name}"
version = "0.1.0"
edition = "{edition}"

[dependencies]
"#
    );

    data_file.write_all(file_content.as_bytes()).expect("Failed to write manifest content");
}