use std::{
    fs::File,
    io::Write
};

pub fn create_manifest(package_name: &str) {
    let mut data_file = File::create("Bubu.toml").expect("Failed to create the manifest file");

    let file_content = format!(
        r#"[package]
name = "{package_name}"
version = "0.1.0"
edition = "2024"

[dependencies]
"#
    );

    data_file.write_all(file_content.as_bytes()).expect("Failed to write manifest content");
}