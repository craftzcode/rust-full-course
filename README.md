# 🦀 Rust Full Course

A comprehensive guide to learning Rust programming language.

---

## 📚 Table of Contents

- [Project Structure](#project-structure)
- [Cargo Basics](#cargo-basics)
- [Project Conventions](#project-conventions)
- [Commands Reference](#commands-reference)
- [Type and Size of Variables](#type-and-size-of-variables)

## 🏗️ Project Structure

### Cargo Project Names

- Must be written in `snake_case` format
- Example: `my_rust_project`

### Rust Projects

- Projects in Rust are also known as **packages** or **crates**
- Two main types of crates:
  - 📦 **Binary Crate**: Standalone application meant to be executed
  - 📚 **Library Crate**: Code intended to be used by other projects

### Target Directory

- Located in `./target/`
- Contains compiled artifacts:
  - Executable files
  - Debug symbols
  - Intermediate compilation files
- The final executable program is stored here after compilation

## 📄 Project Configuration

### Cargo.toml

- Main configuration file for Rust projects
- Contains:
  - Project metadata
  - Dependencies
  - Build settings
  - Package information

## 🛠️ Commands Reference

### Essential Cargo Commands

- `cargo new <project_name>` - Create a new Binary crate or Rust project
- `cargo build` - Compile the project in debug mode (larger file size, includes debug info, faster compilation)
- `cargo build --release` - Compile the project in release mode (smaller file size, optimized performance, longer compilation time)
- **Note**: Always start with `cargo build` for debugging and development, then use `cargo build --release` for final production builds
- `cargo run` - Compile and run the project in one command (no need to run `cargo build` first)
- `cargo check` - Quickly verify if your code has any compilation errors (syntax errors, type mismatches, missing dependencies, etc.) without creating an executable file (faster than `cargo build`)
- `cargo test` - Run project tests

## 📝 Type and Size of Variables

- Rust requires the type and size of a variable to be known at compile time
- This is important for memory safety and preventing runtime errors
- Rust is a statically typed language, which means it checks the types of variables at compile time
- By knowing the type and size of a variable at compile time, Rust can allocate the necessary memory and prevent common errors like buffer overflows

## 📝 Notes

- Keep your code organized and follow Rust's conventions
- Use `cargo doc --open` to generate and view documentation
- Check [rust-lang.org](https://www.rust-lang.org) for official documentation

---

_Happy Coding with Rust! 🦀_
