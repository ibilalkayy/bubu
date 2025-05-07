# 🦀 Bubu

**Bubu** is a local package manager and build tool for Rust projects. It helps you create, compile, run, and manage Rust code efficiently — all without requiring internet access or remote APIs.

- **Author**: Bilal Khan
- **Project Name Taken From**: My Son Nickname
- **License**: Apache License 2.0
- **Contribute**: [GitHub Repository](https://github.com/ibilalkayy/bubu)

---

## 🚀 Features & Commands

Bubu supports various subcommands to streamline your Rust development workflow:

| Command       | Description                                           |
| ------------- | ----------------------------------------------------- |
| `init`        | Initialize a new Rust project with standard layout    |
| `build`       | Build the Rust project into an executable binary      |
| `run`         | Run the compiled Rust binary                          |
| `check`       | Check the project for compile errors without building |
| `doc`         | Generate local documentation for your Rust project    |
| `list`        | List local dependencies or packages                   |
| `install`     | Install a local Rust package into your workspace      |
| `test`        | Run unit or integration tests                         |
| `clean`       | Delete the `target` directory and temporary files     |
| `package`     | Create a compressed package archive of your project   |
| `update-lock` | Update the lockfile from local dependencies           |
| `fmt`         | Format Rust source files using `rustfmt`              |
| `lint`        | Lint your Rust code for warnings and style issues     |
| `stats`       | Show lines of code, file count, and function count    |
| `whoami`      | Show current user or author info from your config     |
| `bench`       | Run performance benchmarks                            |
| `tree`        | Display a local dependency tree                       |
| `publish`     | Publish the project to a local package directory      |

---

## 📦 Installation

```bash
# Clone the repository
git clone https://github.com/ibilalkayy/bubu.git

# Navigate to the project folder
cd bubu

# Build the CLI tool
cargo build --release
```

---

## 🤝 Contributing

Contributions are welcome! If you’d like to contribute, please check out the [GitHub repo](https://github.com/ibilalkayy/bubu), fork it, and submit a pull request.

## 📜 License

This project is licensed under the [Apache License 2.0](LICENSE).
