---
title: Embedded Rust Workshop (Day 2)
sub_title : Prof. Bart Massey and Rustaceans Africa
theme : 
    name: catppuccin-mocha
options: 
    end_slide_shorthand: true
---

# Ownership in Rust

Shown below in an example of a `struct`.

```rust +exec
#[derive(Debug)]   //Allows printing in Debug mode
struct Object {
  height: u64,
}

fn main() {
  let height: u64 = 12;
  let _q = Object { height };
  println!("Height is : {}", _q.height);
}
```
