use std::{
    fs::File,
    io::Write, path::Path,
    process::{Command, Stdio}
};

pub fn create_manifest(package_name: &str, edition: &str, name: &str) {
    let is_present = Path::new("Bubu.toml").exists();
    if is_present {
        eprintln!("error: `cargo init` cannot be run on existing Cargo packages");
        return;
    }

    let mut data_file = File::create("Bubu.toml").expect("Failed to create the manifest file");
    let file_content;

    if name.len() > 0 {
        file_content = format!(r#"[package]
name = "{name}"
version = "0.1.0"
edition = "{edition}"

[dependencies]
"#
    );
    } else {
        file_content = format!(r#"[package]
name = "{package_name}"
version = "0.1.0"
edition = "{edition}"

[dependencies]
"#
    );
    }

    data_file.write_all(file_content.as_bytes()).expect("Failed to write manifest content");
}

pub fn git_file() {
    let mut file = File::create(".gitignore").expect("Failed to create a file");

    let file_content = r#"/target"#;

    file.write_all(file_content.as_bytes()).expect("Failed to write the content");

    let status = Command::new("git")
        .arg("init")
        .stdout(Stdio::null())
        .status()
        .expect("Failed to execute the git command");

    if !status.success() {
        eprintln!("Git command failed");
    }
}