use std::{
    fs::{self, File},
    io::Write
};

pub fn bin_usage(package_name: &str) {
    create_manifest(package_name);
    let _ = create_hello_world();
}

fn create_manifest(package_name: &str) {
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

fn create_hello_world() {
    fs::create_dir("srces").expect("unable to create a src directory");
    
    let mut file = File::create("srces/main.rs").expect("Failed to create a main.rs file");

    let file_content = r#"fn main() {
    println!("Hello, World!");
}"#;    

    file.write_all(file_content.as_bytes()).expect("Failed to write the main content");
}