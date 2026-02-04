# Rust Packages, Crates, and Modules

This project demonstrates how a Rust application can be organized using **Packages**, **Crates**, and **Modules**. Multiple Rust concepts implemented in separate tasks are combined into a single, well-structured modular project.

## Overview

The project shows how to structure code by grouping related functionality into individual modules inside a utility directory (`utils`) and invoking them from the binary crate entry point (`main.rs`). This approach improves code readability, maintainability, and reusability.

## Project Structure

*   **`src/main.rs`**: The binary crate entry point. It imports the `utils` module and calls functions exposed by its submodules.
*   **`src/utils/mod.rs`**: The central module file that declares and re-exports public submodules.
*   **`src/utils/`**: Directory containing feature-specific modules:
    *   `_1_control_statements`: Programs demonstrating control flow logic.
    *   `_2_structure`: Examples using structs and associated methods.
    *   `_3_serde_json`: Handling JSON data using serialization and deserialization.
    *   `_4_static`: Demonstration of static variables with thread safety using Mutex and RwLock.

## Code Organization

### Module Declaration
In `src/utils/mod.rs`, the submodules are declared as public:
```rust
pub mod _1_control_statement;
pub mod _2_structure;
// ...
```

### Module Usage
In `src/main.rs`, the functionality is accessed via the module path:
```rust
mod all_utils;

fn main() {
    all_utils::_1_control_statement::main();
    all_utils::_2_structure::main();
    // ...
}
```

## Usage

To run the consolidated examples:

```bash
cargo run
```