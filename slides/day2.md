---
title: Embedded Rust Workshop (Day 2)
sub_title : Prof. Bart Massey and Rustaceans Africa
theme : 
    name: catppuccin-frappe
options: 
    end_slide_shorthand: true
---

# Ownership in Rust

Shown below in an example of a `struct`.

```rust +exec
#[derive(Debug)]
struct Object {
    height: String,
}

fn main() {
    let height = String::from("twelve");
    let _q = Object { height };
    // println!("Height is: {}", height); // ❌ error: borrow of moved value
    println!("Height is: {}", _q.height); // ✅ works
}
```

---

# How this program explains ownership

1. **Move semantics for non-Copy types**  
   - When `height: String` is assigned to `_q.height`, ownership of the string moves into the struct.  

<!--new_lines: 1-->
2. **Invalid after move**  
   - The original `height` variable becomes invalid and cannot be used anymore.  

<!--new_lines: 1-->
3. **Struct takes ownership**  
   - The `Object` struct now fully owns the `String` value.  

<!--new_lines: 1-->
4. **Contrast with Copy types**  
   - Unlike integers (`u64`), `String` doesn’t implement `Copy`, so no implicit duplication happens.  


---

# Cloning a `struct`

```rust +exec
#[derive(Debug, Clone)]
struct Object {
  height: u64,
}

fn main() {
  let height = 12;
  let q = Object { height };
  println!("Height (Original) : {}", q.height);
  let _r = q.clone();
  println!("Cloned Value : {}", _r.height);

}
```

---

# How this program shows every value has a unique owner

1. **Initial ownership**  
   - The value `12` is first owned by the variable `height`, and then its ownership is moved into `q.height`.  

<!--new_lines: 1-->
2. **Struct as owner**  
   - After this move, `q` (the `Object` instance) becomes the sole owner of the value.  

<!--new_lines: 1-->
3. **Cloning creates a new owner**  
   - Calling `q.clone()` makes a fresh, independent copy (`_r`), giving `_r` its own ownership of the same data.  

<!--new_lines: 1-->
4. **No shared implicit ownership**  
   - Rust enforces that each value is uniquely owned—cloning explicitly creates a new value with a new owner, instead of sharing ownership silently.  

---

# Copying a `struct`

```rust +exec
#[derive(Debug, Clone, Copy)]
struct Object {
    height: u64,
}

fn main() {
    let q = Object { height: 12 };
    let r = q; // Copies instead of moving
    println!("q.height = {}", q.height); // ✅ still valid
    println!("r.height = {}", r.height); // ✅ also valid
}
```

---

# What happens if `Copy` is added in `#[derive]`

1. **Implicit duplication**  
   - Assigning a `Copy` type (like `Object`) to another variable duplicates the value automatically, instead of moving ownership.

<!--new_lines: 1-->
2. **Both variables stay valid**  
   - After `let r = q;`, both `q` and `r` can be used safely since `q` was not invalidated.

<!--new_lines: 1-->
3. **Restriction**  
   - Only types with fixed size and no heap allocations (e.g., integers, floats, or structs made of them) can derive `Copy`.

<!--new_lines: 1-->
4. **Difference from `Clone`**  
   - `Clone` requires an explicit method call and can be expensive, while `Copy` is implicit, cheap, and compiler-enforced.

---

# `Clone` vs `Copy` in Rust


| Aspect           | Clone                              | Copy                                    |
|------------------|------------------------------------|-----------------------------------------|
|                  |                                    |                                         |
| How it's done    | Explicit call: `x.clone()`         | Implicit: happens on assignment or pass |
|                  |                                    |                                         |
| Cost             | Can be expensive (deep copy)       | Always cheap (bitwise copy)             |
|                  |                                    |                                         |
| When available   | Any type implementing `Clone`      | Only simple, fixed-size types (no heap) |
|                  |                                    |                                         |
| Ownership effect | Creates a new owner explicitly     | Creates a new owner automatically       |
|                  |                                    |                                         |
| Control          | Programmer decides when to clone   | Compiler duplicates without asking      |
|                  |                                    |                                         |
| Common usage     | Strings, Vectors, custom deep data | Integers, floats, simple `struct`s      |

---

# References  in Rust

<!--column_layout: [1, 1]-->
<!--column: 0-->
```rust +exec +id:foo
#[derive(Debug)]
struct Object {
    height: u64,
}

fn main() {
    let mut q = Object { height: 12 };
    let r1 = &q;
    println!("{:?}", r1);
    let r2 = &q;
    println!("{:?}", r2);
    // let r3 = &mut q;
    // println!("{:?}", r3);
    {
        let r3 = &mut q;
        r3.height += 5;
        println!("{:?}", r3);
    }
    println!("{:?}", q);
    // let m1 = &mut q;
    // let m2 = &mut q;
    // println!("{:?} {:?}", m1, m2);
}
```

<!--column: 1-->
<!--new_lines: 6-->
<!-- snippet_output: foo -->

---

# Explanation of References with Struct Example

1. **Multiple immutable references**  
   - `r1` and `r2` can both borrow `q` at the same time because they are read-only.

<!--new_lines: 1-->
2. **Immutable + mutable conflict**  
   - `let r3 = &mut q;` (commented out) would cause an error if used alongside active immutable borrows (`r1`, `r2`).

<!--new_lines: 1-->
3. **Mutable reference in scope**  
   - Inside the block, `r3` is a mutable borrow. It can modify `q.height`.  
   - After the block ends, the borrow is released.

<!--new_lines: 1-->
4. **Final access**  
   - After the mutable borrow ends, `q` is valid again and can be used normally.

<!--new_lines: 1-->
5. **Two mutable references error**  
   - `let m1 = &mut q; let m2 = &mut q;` (commented out) causes a compile error because only one mutable borrow is allowed at a time.

---

# Tuple `structs`

```rust +exec
#[derive(Debug, Clone)]
struct Object {
    height: u64
}

#[derive(Debug, Clone)]
struct TObject(u64);

fn main() {
  let height = 12;
  let q = Object { height };
  let r = TObject(13);

  println!("Normal struct : {:?}", q.height);
  println!("Tuple struct : {:?}", r.0);
}
```

---

# `struct` vs Tuple `struct`


| Aspect        | Struct                                | Tuple Struct                           |
|---------------|---------------------------------------|----------------------------------------|
| **Definition**    | Named fields with types               | Unnamed fields, only types specified   |
|               |                                       |                                        |
| **Field access**  | Access via field name (`obj.height`)  | Access via index (`obj.0`, `obj.1`)    |
|               |                                       |                                        |    
| **Readability**   | More descriptive and self-documenting | Less descriptive, concise              |
|               |                                       |                                        |    
| **Use case**      | Complex data with clear meaning       | Lightweight wrappers, quick grouping   |
|               |                                       |                                        |    
| **Derive traits** | Works naturally with `Debug`, etc.    | Same, but field names won’t appear     |


---

# Enums in Rust

```rust +exec
#[derive(Debug, Clone)]
enum Color {
  Red, 
  Green,
  Blue,
}

fn main() {
  let red = Color::Red;
  let green = Color::Green;
  let blue = Color::Blue;

  println!("Red : {:?}", red);
  println!("Green : {:?}", green);
  println!("Blue : {:?}", blue);
}
```

---

# Why `enum`'s


1. **Multiple related variants**  
   - Represent different but related possibilities under one type.  
<!--new_lines: 1-->
2. **Type safety**  
   - Prevents invalid states by encoding all valid cases explicitly.  

<!--new_lines: 1-->
3. **Pattern matching**  
   - Works seamlessly with `match` to handle every variant safely.  

<!--new_lines: 1-->
4. **Expressiveness**  
   - Variants can hold data, making them more powerful than plain constants.  

<!--new_lines: 1-->
5. **Clarity**  
   - Improves code readability by grouping related states together.  

<!--new_lines: 1-->
6. **Zero-cost abstraction**  
   - Compiles down efficiently without runtime overhead.  

---

# `enum` and `match`

<!--column_layout: [2, 1]-->

<!--column: 0-->

```rust +exec +id:enum
# #![allow(warnings)]
#[derive(Debug, Clone)]
enum Color {
  Red, 
  Green,
  Blue,
  Any(f32, f32, f32),      // A Tuple struct inside enum
}
fn main() {
  let color = Color::Any(0.8, 0.6, 0.4);
  match color {
      Color::Red => println!("RED"),
      Color::Green => println!("GREEN"),
      Color::Blue => println!("BLUE"),
      Color::Any(r, g, b) => {
        if r > g && r > b {
            println!("It's reddish : {} {} {}", r, g, b);
        }
      
      }
      _ => println!("Invalid color")
  }
}
```

<!--column: 1--> 

<!--snippet_output: enum-->

---

# Arrays in Rust

```rust +exec
fn main() {
  let _a: [u32; 3] = [1, 2, 3];     // Size of array here is 3
  // let _a: [u32; 3] = [1, 2, 3, 4];  // Error
  let _b: [u32; 10] = [3; 10];   // An array of 3 repeated 10 times
  //Iterating through arrays
  let mut c: [u32; 3] = [4; 3];

  for i in 0..3 {
      c[i] = i as u32 + 1;     // Converting usize to u32
  }

  println!("{} {}", c[0], c[2]);
}
```

---

# Rust Arrays

1. **Fixed Size**
   - Arrays have a length decided at compile time, and this size cannot change once defined.
<!--new_lines: 1-->
2. **Type-Safe**
   - Every element in an array must have the same data type, ensuring consistency and preventing type errors.

<!--new_lines: 1-->
3. **Length as Part of the Type**
   - The size of an array is part of its type, meaning arrays of different lengths are considered different types.

<!--new_lines: 1-->
4. **Initialization Options**
   - Arrays can be created by listing values explicitly or by repeating a single value for a set length.

<!--new_lines: 1-->
5. **Index-Based Access**
   - Elements are accessed using zero-based indexing, and invalid indices will cause runtime errors.

<!--new_lines: 1-->
6. **Efficient Storage**
   - Arrays are stored in a contiguous block of memory, making access and iteration very efficient.

---

# `type` keyword in Rust

```rust +exec
type DemoArray = [u32; 3];      // Alias for an array of u32 and size 3

fn show_array(arr: DemoArray) {
  for i in 0..arr.len() {
    println!("{}", arr[i]);
  }
}

fn main() {
  let a: DemoArray = [4; 3];
  show_array(a);
}
```

---

# Rust `type` Keyword

1. **Type Aliases**
   - The `type` keyword creates an alias for an existing type, making code shorter and more readable.
<!--new_lines: 1-->
2. **No New Type Creation**
   - Using `type` does not define a brand-new type; it only provides another name for an existing one.

<!--new_lines: 1-->
3. **Improves Clarity**
   - Complex or frequently used types can be given simpler names, which makes function signatures and code easier to understand.

<!--new_lines: 1-->
4. **Consistency in Code**
   - A single alias can be reused throughout the code, reducing duplication and potential errors.

<!--new_lines: 1-->
5. **Flexibility with Generics**
   - Type aliases can also be used with generics, helping simplify generic-heavy code.

<!--new_lines: 1-->
6. **Common in Libraries**
   - Rust libraries often use type aliases to provide clearer, domain-specific names for standard types.

---

# Closures in Rust

``` rust +exec
fn main() {
    // A closure that adds two numbers
    let add = |x: i32, y: i32| -> i32 {
        x + y
    };

    // Using the closure
    let result = add(5, 3);
    println!("Result: {}", result);

    // A closure that captures a variable from its environment
    let factor = 10;
    let multiply = |x: i32| x * factor;

    let result2 = multiply(6);
    println!("Result with capture: {}", result2);
}
```

---

## Rust Closures – Key Points

1. **Anonymous Functions**
   - Closures are functions without names that can be defined directly where they are used.
<!--new_lines: 1-->
2. **Flexible Syntax**
   - They can accept parameters and return values, often with simpler syntax than regular functions.

<!--new_lines: 1-->
3. **Environment Capture**
   - Closures can capture variables from the surrounding scope, either by borrowing or taking ownership.

<!--new_lines: 1-->
4. **Type Inference**
   - The Rust compiler usually infers parameter and return types for closures, reducing the need for explicit annotations.

<!--new_lines: 1-->
5. **Reusability**
   - Closures can be stored in variables, passed to functions, or returned from functions, making them highly versatile.

---

```rust +exec
type DemoArray = [u32; 1024];

fn main() {
  let mut a: DemoArray = core::array::from_fn(|i| i as u32 + 1);
  //from_fn here allows you to create an array from closures

  let mut show_array = || {
    a[0] += 1;
    println!("{} {}", a[0], a[1023]);
  };

  show_array();
  show_array();
}
```

---

# The `core` library

## Rust Core Library – Key Points

1. **Minimal Foundation**
   - The core library (`core`) provides Rust’s essential building blocks without relying on an operating system.
<!--new_lines: 1-->
2. **No Standard Library Dependency**
   - It is independent of the standard library (`std`) and is suitable for bare-metal or embedded systems.
<!--new_lines: 1-->

3. **Essential Traits and Types**
   - It defines fundamental traits (like `Copy`, `Clone`, `Iterator`) and types (like `Option`, `Result`).
<!--new_lines: 1-->

4. **Platform Independence**
   - Since it avoids OS features, the `core` library works across all platforms, including `no_std` environments.

<!--new_lines: 1-->
5. **Foundation for std**
   - The standard library (`std`) is built on top of `core`, adding extra functionality like I/O, threading, and heap allocation.

---

# `core` vs `std`

<!--new_lines: 2-->

| Aspect | `core` | `std` |
| --------------- | --------------- | --------------- |
| **Dependency** | No OS required | Depends on OS |
|        |        |       |
| **Scope** | Minimal, contains only essential types and traits | Full standard library |
|        |        |       |
| **Key features** | Provides `Option`, `Result`, `Iterator`, `Clone` etc | Adds I/O, networking, threads.. |
|        |        |       |
| **Use Case** | `no_std`, Embedded Systems | General-purpose applications |
|        |        |       |
| **Relationship** | Foundation for `std` | Built on top of `core` |


<!--end_slide-->

# How to use Rust in `no_std` environments

<!--new_lines: 1-->

To use rust in `no_std` environments, all we have to do is add a `no_std` tag at the top of the code

```rust
#[no_std]

fn main() {
  todo!();
}
```

This is the start of Embedded Rust.

Note that `println!()` won't work in `no_std` as `println!()` is a part of `std` library.
