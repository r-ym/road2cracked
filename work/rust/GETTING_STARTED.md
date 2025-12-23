# Getting Started with Rust

## Installation

### Install Rust using rustup (recommended)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Verify installation
```bash
rustc --version
cargo --version
```

### Update Rust
```bash
rustup update
```

## Your First Rust Program

Create a new project:
```bash
cargo new hello_world
cd hello_world
cargo run
```

## Essential Cargo Commands

- `cargo new <project>` - Create a new Rust project
- `cargo build` - Compile the project
- `cargo run` - Build and run the project
- `cargo test` - Run tests
- `cargo check` - Check code without producing executable (faster)
- `cargo clippy` - Run the Rust linter
- `cargo fmt` - Format code according to Rust style

## Project Structure

```
project_name/
├── Cargo.toml       # Project manifest (dependencies, metadata)
├── Cargo.lock       # Locked dependency versions
└── src/
    └── main.rs      # Entry point
```

## Editor Setup

### VS Code (Recommended)
- Install "rust-analyzer" extension
- Install "CodeLLDB" for debugging

### Other Options
- CLion with Rust plugin
- Vim/Neovim with rust.vim and rust-analyzer
- Emacs with rust-mode

## Next Steps

1. Complete "The Rust Programming Language" book (see RESOURCES.md)
2. Work through exercises in the `exercises/` folder
3. Solve problems on Exercism or LeetCode in Rust
4. Build small projects to practice concepts
