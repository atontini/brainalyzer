# Rust New Project Template

This is a custom project template for cargo projects, complete with a predefined `.gitignore`, GitHub Action for continuous integration, and other configurations.

## Usage

To create a new project using this template, you can use [`cargo-generate`](https://github.com/cargo-generate/cargo-generate).

## Steps

1. **Install `cargo-generate`** if you haven’t already:
```bash
cargo install cargo-generate
```
2. Generate a New Project from this template by running:
```bash
cargo generate --git https://github.com/atontini/rust_new_project_template --name project-name
```
* Replace project-name with the desired name for your new project.

After running this command, cargo-generate will create a new project with the custom `.gitignore`, GitHub Action for CI, and other predefined files included.

Example Directory Structure
Once generated, your project structure will look like this:

```
├── .gitignore
├── Cargo.toml
└── src
    └── main.rs
```

### GitHub Actions

This template includes a GitHub Action (`rust.yml`) that automatically compiles the `main` branch after each push, ensuring that your project remains in a buildable state.


Contributions are welcome! Please fork the repository, create a new branch, make your changes, and submit a pull request.