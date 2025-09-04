---
title: Embedded Rust Workshop (Day 1)
sub_title : Prof. Bart Massey and Rustaceans Africa
theme : 
    name: catppuccin-mocha
options: 
    end_slide_shorthand: true

---

# Rust tools 

1. `rustup` : The Rust Toolchain Installer. This installs all the required Rust tools. 
2. `cargo` : The official package manager of Rust. It downloads your dependencies, compiles your packages , makes distributable binaries and also uploads them to  [](crates.io), the official Rust package registry.
3. `rustc` : The Rust compiler.

## Creating a new project in Rust

To create a new Rust project we use the below command 

```bash
cargo new <project_name>

#OR

cargo new --bin <project_name>
```

Here `--bin` makes sure that the new project created is a program and not a library.

---

# The project directory structure - 1

Once you have run the command `cargo new`, you will get a project directory with the following structure : 

```md
 .
├──  Cargo.lock
├──  Cargo.toml
└──  src
    └──  main.rs
```
## `Cargo.toml`

This is the file containing all the metadata of the project. It contains stuff like the Project Name, Rust version used, Project Dependencies etc. Here is a barebones `Cargo.toml`

```toml
[package]
name = "project_name"   # The name of your crate/project
version = "0.1.0"       # The current version of your project (semver format)
edition = "2021"        # Rust edition (2015, 2018, or 2021 are valid)

[dependencies]
# This section is where you list external crates your project depends on.
# Format: crate_name = "version"
```

---

# The project directory structure - 2

## `src.main.rs`

This is the main file where we write our Rust programs. `cargo` generates a default `Hello, World` program as shown below. If you are using `presenterm` to present this file, press `CTRL + E` to execute the code.

```rust +exec
fn main() {
  println!("Hello, world!");
}
```

---
# The project directory structure - 3

## Building the code

To build the code, run the below command in your project's root directory, the executable binary (ELF for Linux and EXE for Windows) will be available in the `target/debug` directory) : 

```bash
cargo build
```

## Running the code

To run the Rust code, run the below command in your project's root directory : 

```bash
cargo run
```
This command first builds the code and then runs it. So `cargo build` is optional.

To optimize the binary, we use the `--release` flag since by default Rust builds for debugging which increases the binary size.

```bash
cargo run --release
```


---

# `Hello, world!` program breakdown

```rust +exec
fn main() {
  println!("Hello, world!");
}
```

## `fn main() { ... }`

1. `fn` -> Keyword for defining a function in Rust
2. `main` -> The entry point of a Rust program. All the program's logic will be written inside the `main` function. This is similar to `int main()` in C/C++.

## `println!("Hello, world!);`

`println!` is a macro which prints output to terminal with a newline `\n` at the end. To print the output without the newline use the `print!` macro instead.

Note the Semicolon (`;`) at the end. This indicates the end of the line.

---

# Hardware

1. BBC Microbit V2

![image:width:100%](./images/bbc_microbit_v2.jpg)

---
# Variables in Rust

To define a variable in Rust, we use the `let` keyword.

## Immutable variables

This is the default type of variables in Rust. Here the value cannot be modified or changed.

## Mutable Variables

The value of this variable can be modified or changed. To define a mutable variable, we use the `mut` keyword. 

```rust +exec
fn main() {
  let x = 45;      // Immutable variable
  let mut y = 56;  // Mutable variable
  println!("Immutable variable : {x}");
  println!("Mutable Variable : {y}");
  y *= 2;
  print!("Modified value : {y}");
}
```

---

# Data types in Rust - 1

## Integers

Integers in Rust are scalar types that represent whole numbers, available in both signed (iN) and unsigned (uN) forms with fixed bit-widths (8, 16, 32, 64, 128, or pointer-sized).

### Signed integers

| Type   | Size    |
|--------------- | --------------- |
| `i8`   | 8 bits   |
| `i16`   | 16 bits   |
| `i32`   | 32 bits   |
| `i64`   | 64 bits   |
| `i128`   | 128 bits   |
| `isize`   | Pointer-sized (Depends on architecture)   |

<!--new_line-->

### Unsigned Integers


| Type   | Size    |
|--------------- | --------------- |
| `u8`   | 8 bits   |
| `u16`   | 16 bits   |
| `u32`   | 32 bits   |
| `u64`   | 64 bits   |
| `u128`   | 128 bits   |
| `usize`   | Pointer-sized (Depends on architecture)   |


---

## Floating point

Floating point numbers are basically decimal numbers

### Float types

| Type | Size |
| -------------- | --------------- |
| `f32` | 32 bits |
| `f64` | 64 bits |


## Boolean (`bool`)

Boolean can have only 2 values `true` and `false`

## Character (`char`)

This has a size of 4 bytes and is used to represent a single Unicode Scalar value.


---

# Program that prints all datatypes

```rust +exec
fn main() {
  let a: i16 = -25;     // 16-bit signed integer
  let b: u32 = 45;      // 16-bit unsigned integer
  let c: f32 = 256.32;  // 32-bit Floating point value
  let d: char = 'c';    // Character
  let e: bool = true;   //Boolean

  println!("16-bit signed Integer : {}", a);
  println!("16-bit unsigned Integer : {}", b);
  println!("32-bit Floating point value : {}", c);
  println!("Character : {}", d);
  println!("Boolean value : {}", e);
}
```

---
# Strings in Rust

In Rust (Or any language for that matter), a string is a collection of characters.

## String types

1. `String` -> An owned, growable, heap-allocated UTF-8 string.
2. `&str` (**String Slice**) -> A borrowed view of `String`, often used for String literals.

```rust +exec
fn main() {
    let slice: &str = "Hello";  
    println!("slice: {}", slice);
    let mut owned: String = String::from("Hello");
    owned.push_str(", world!"); // we can modify it
    println!("owned: {}", owned);
    let borrowed: &str = &owned;
    println!("borrowed from String: {}", borrowed);
}
```

---

# Specify datatype of a number

```rust +exec
fn main() {
  let x = 3u8 - 2u8;     // Specifying datatype as u8
  let y = 4i8 - 10i8;     // Specifying datatype as i8

  println!("{x}");
  println!("{y}");
}
```
---

# Type casting

This is used to change from one datatype to another. 

Type casting uses the `as` keyword. 

```rust +exec
fn main() {
  let x = 4u32 - 3i16 as u32;

  println!("{x}");
}
```
