use std::{fs::{self, File}, io::Write};

use crate::common::common::create_manifest;

pub fn lib_usage(package_name: &str, edition: &str, name: &str) {
    create_manifest(package_name, edition, name);
    lib_file();
}

fn lib_file() {
    fs::create_dir("srces").expect("Failed to create a directory");

    let mut file = File::create("srces/lib.rs").expect("Failed to create a file");

    let file_content = r#"pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}"#;

    file.write_all(file_content.as_bytes()).expect("Failed to write the lib content");
}