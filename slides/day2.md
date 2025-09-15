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

```rust +exec
# #![allow(warnings)]
#[derive(Debug, Clone)]
enum Color {
  Red, 
  Green,
  Blue,
}

fn main() {
  let color = Color::Red;
  match color {
      Color::Red => println!("RED"),
      Color::Green => println!("GREEN"),
      Color::Blue => println!("BLUE"),
      _ => println!("Invalid color")
  }
}
```
