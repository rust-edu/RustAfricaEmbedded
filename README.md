# Rust Africa Embedded
Bart Massey 2025

This repo is the "home page" for the Rust Africa Embedded
2025 course, containing course links and documents.

* [Course Outline](course-outline.md)
* [Rustup](https://rustup.rs)
* [*The Rust Programming Language*](https://doc.rust-lang.org/book/)
* [*Rust Embedded Discovery Book*](https://rust-embedded.github.io/discovery-mb2)
* [Rust-Edu Zulip](https://zulip.rust-edu.org) `#RustAfricaEmbedded`
* [Portland State Embedded Code](https://github.com/pdx-cs-rust-embedded)
* [Exercism Rust Track](https://exercism.org/tracks/rust)
* [Rustlings](https://rustlings.rust-lang.org/)


## Running the files

Before attempting to run the Rust files in this repo, ensure that you already [installed Rust for your operating system](https://www.rust-lang.org/tools/install) and have at least `cargo` or `rustc` available on your path (system or user).

To run the files in this course, navigate to the directory of the particular lesson and run the `cargo run` command. An example is shown below:

```sh
cd day-1/rust-basics
cargo run
```

If you wish to build a binary which you can run at a later time, you can use the `cargo build --release` command for an optimized release or the `cargo build` for an unoptimized release (debug release).

If you'd rather not navigate into directories to run files, you can utilize the `runfiles.sh` script on the root directory. It gives you an overview of all the `main.rs` files in the course and allows you run them iteractively. It uses `rustc` which Cargo also uses under the hood.