# rust-basics

Here are contained basic Rust projects, which main objective is just to train the basics of the language.
Each project is either:

- A standalone script (which executable can be created with `rustc`).
- A cargo crate, for those projects that are a little bit more complicated.

Alongside the projects, in this document is presented a cheat-sheet over some basic rust usages.

## Index

- [**Theory**](#rust-basic-theory)
- [**Projects**](#uploaded-projects)
    - [Scripts](#scripts)
    - [Crates](#crates)
- [**Rust installation**](#rust-installation)

---
## Rust basic theory

### 1. Formatting

| Single files (scripts)  | Cargo pkgs                            |
|-------------------------|---------------------------------------|
| `rustfmt FILE_PATH.rs`  |  `fmt` (from within the cargo folder) |

> Note: The formatting will also return linting messages.

### 2. Compiling

**Single files**
```
rustc FILE_PATH.rs
```

**Cargo**

- `cargo build`: Create an executable at _target/debug/CRATE-NAME_.
- `cargo run`: Creates the previous executable and runs it.
- `cargo check`: Sanity checks over the code without compiling.
- `cargo build --release`: Create an executable at _target/release/CRATE-NAME_. The code compiles with optimizations for the finla executable to run faster.

### 3. Documentation

`cargo doc` builds locally the documentation of the cargo project.<br>
See more at https://doc.rust-lang.org/cargo/commands/cargo-doc.html
 

---
## Uploaded projects

Here are listed all projects that are uploaded in the repository.
They all link to the main file, that is either the script RS file for the scripts or the 'src/main.rs' file at the crates.

### Scripts

- [Temperature converter](./scripts/temperature_converter.rs)
- [`rustlings` exercises](./scripts/rustlings/)

### Crates

- [Number guessing game](./crates/guessing_game/src/main.rs)

---
## Rust installation

The following guide is created assuming Windows OS.

1. Download the rust executable from the original source: https://www.rust-lang.org/tools/install
2. Launch and install the executable. Rust will require of C++ tools for development.
3. Check a correct installed version by writing `rustc --version` within the terminal.

With this, rust is correctly installed through `rustup`.

At last, two additional commands are required to make `rust` runnable on windows:

```
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

At last, it also provides commands to update it or uninstall it:

- `rustup update`
- `rustup self uninstall`

