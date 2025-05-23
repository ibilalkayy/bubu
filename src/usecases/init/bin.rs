use std::{
    fs::{self, File},
    io::Write
};

use crate::common::common::{
    create_manifest, git_file
};

pub fn bin_usage(package_name: &str, edition: &str, name: &str) {
    create_manifest(package_name, edition, name);
    main_file();
    git_file();
}

fn main_file() {
    fs::create_dir("srces").expect("unable to create a src directory");
    
    let mut file = File::create("srces/main.rs").expect("Failed to create a file");

    let file_content = r#"fn main() {
    println!("Hello, World!");
}"#;    

    file.write_all(file_content.as_bytes()).expect("Failed to write the main content");
}