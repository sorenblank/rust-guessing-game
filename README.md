# Rust Learning Notes - Number Guessing Game

## Project Overview

Built a command-line number guessing game that:

- Generates a random number between 1-100
- Takes user input
- Compares the guess with the secret number
- Provides feedback (too high/too low)
- Continues until correct guess

## Special Rust Syntax

### 1. Turbofish Operator `::<>`

- Used to specify type parameters for generic functions
- Common with `parse` method:

```rust
// Using turbofish
let four = "4".parse::<u32>().unwrap();

// Alternative using type annotation
let four: u32 = "4".parse().unwrap();

// Both achieve the same result
```

### 2. Associated Functions (Static Methods)

- Called using `::` syntax
- Don't take `self` as parameter
- Common examples:

```rust
String::new()           // Create new String
Vec::<i32>::new()      // Create new vector with type specification
```

### 3. Method Chaining

```rust
// Multiple methods can be chained
let num = "  45\n".trim().parse::<u32>().unwrap();
```

### 4. Function vs Method Syntax

- Functions: `function_name(argument)`
- Methods: `object.method_name()`
- Associated functions: `Type::function_name()`

## Key Rust Concepts Learned

### 1. Variables and Mutability

- Variables are immutable by default
- Use `mut` keyword to make variables mutable

```rust
let foo = 5;        // immutable
let mut bar = 5;    // mutable
```

### 2. String Handling

- Create new empty string:

```rust
let mut guess = String::new();
```

- String methods:
  - `trim()`: Removes whitespace/newlines
  - `parse()`: Converts string to another type

### 3. Input/Output

- Import IO module: `use std::io;`
- Read user input:

```rust
io::stdin()
    .read_line(&mut variable_name)
    .expect("Error message");
```

### 4. Error Handling

- `Result` type has two variants: `Ok` and `Err`
- Handle errors using:
  - `expect()`: Crashes with message if error
  - `match` expression for more graceful handling:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

### 5. Match Expression

- Pattern matching syntax:

```rust
match value {
    pattern1 => action1,
    pattern2 => action2,
    pattern3 => action3,
}
```

- Used for control flow and error handling
- Must cover all possible cases

### 6. Loops

- Create infinite loop: `loop { }`
- Break out using `break` keyword
- Skip iteration using `continue`

### 7. External Dependencies (Crates)

- Add dependencies in `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
```

- Import in code: `use rand::Rng;`
- Use semantic versioning for dependencies

### 8. Type System

- Strong static typing
- Type annotation syntax: `let guess: u32`
- Common types:
  - `String`: Text
  - `u32`: Unsigned 32-bit integer
  - Supports type inference

### 9. References and Borrowing

- `&` indicates a reference
- `&mut` for mutable reference
- Allows sharing data without copying

### 10. Cargo Commands

```bash
cargo new project_name    # Create new project
cargo build              # Compile
cargo run               # Compile and run
cargo update            # Update dependencies
cargo doc --open        # Open documentation
```

## Important Files

- `Cargo.toml`: Project configuration and dependencies
- `Cargo.lock`: Ensures reproducible builds
- `src/main.rs`: Main source code

## Best Practices Learned

1. Use meaningful error messages
2. Handle errors gracefully
3. Keep code organized with proper formatting
4. Use type annotations for clarity
5. Comment code for better understanding
6. Use semantic versioning for dependencies

## Memory Management

- Rust enforces ownership rules at compile time
- Variables are scoped
- Memory is automatically freed when variables go out of scope

## Next Learning Steps

- [ ] Understanding ownership in detail
- [ ] Exploring structs and enums
- [ ] Learning about traits
- [ ] Diving into modules and packages
- [ ] Understanding lifetimes

## Useful Resources

- The Rust Book (https://doc.rust-lang.org/book/)
- Rust by Example (https://doc.rust-lang.org/rust-by-example/)
- Rust Standard Library Documentation
- Crates.io for finding packages
