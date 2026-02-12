# HashMap 

This project demonstrates the usage of Rust’s **HashMap** collection from the standard library.  
It focuses on storing, retrieving, removing, filtering, and extending data using a custom `Student` struct.

---

## Overview

The application uses a **HashMap** to manage student records where:

- **Key** → `u32` (Student ID)
- **Value** → `Student` struct (id, name, marks)

This example showcases common real-world operations performed on a `HashMap`.

---

## Student Struct

```rust
#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    marks: u32,
}
```

The `Student` struct represents individual student data stored as values inside the `HashMap`.

---

## HashMap Operations Demonstrated

### Creation
- `HashMap::new()` creates an empty map.

### Memory Reservation
- `try_reserve()` pre-allocates memory to reduce reallocation overhead.

### Insertion
- `insert()` adds key-value pairs to the map.

### Retrieval
- `get()` accesses values using keys.
- `unwrap()` extracts the value when it is guaranteed to exist.

### Removal & Ownership
- `remove()` deletes an entry and returns `Option<V>`.
- `take()` moves the value out of `Option`, leaving `None`.

### Filtering
- `retain()` keeps only entries that satisfy a condition  
  (e.g., students with marks ≥ 50).

### Extension
- `extend()` merges another `HashMap` into the existing one.

---

## What This Project Demonstrates

- Key-value storage using `HashMap`
- Working with structured data as values
- Safe access using `Option`
- Ownership transfer using `take()`
- Filtering data efficiently
- Optimizing memory usage
- Merging collections

---

## How to Run

```bash
cargo run
```

---

## Expected Output

The program prints:

1. Student details retrieved using `get()`
2. Removed student information
3. Filtered students after applying `retain()`
4. Updated student list after `extend()`
5. Final state of the `HashMap`

---
