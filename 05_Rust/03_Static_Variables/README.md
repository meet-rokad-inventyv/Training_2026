# Rust Static Variables with Mutex and RwLock

## Overview

This project demonstrates how to safely use **static variables** in Rust using:

- `Mutex<T>`
- `RwLock<T>`

The program simulates a **simple request handling system** that processes different types of HTTP-like requests (`GET`, `POST`, `DELETE`) and keeps track of request counts using shared global state.

---

## Objective

- Understand why **static mutable variables are unsafe** in Rust
- Learn how `Mutex` and `RwLock` provide **thread-safe interior mutability**
- Compare **Mutex vs RwLock** usage patterns
- Track request statistics safely across function calls

---

## Project Structure

The code is organized into modules:
*   `src/main.rs`: The entry point that calls functions from the `mutex` and `rwlock` modules.
*   `src/mutex.rs`: Implements request handling using `std::sync::Mutex`.
*   `src/rwlock.rs`: Implements request handling using `std::sync::RwLock`.


### File Description

- **main.rs**
  - Entry point of the application
  - Calls both `mutex` and `rwlock` implementations

- **mutex.rs**
  - Uses `std::sync::Mutex`
  - Demonstrates exclusive locking for read/write

- **rwlock.rs**
  - Uses `std::sync::RwLock`
  - Demonstrates shared reads and exclusive writes

---

## Why Mutex and RwLock?

Rust does **not allow mutable global variables** by default because they are unsafe in concurrent environments.

To safely modify shared global state:
- `Mutex<T>` → Only **one thread** can access data at a time
- `RwLock<T>` → Multiple **readers** OR one **writer**

---

## Static Variables Used

Each module defines the following static counters:

```rust
TOTAL
GET_REQUESTS
POST_REQUESTS
DELETE_REQUESTS
```

## Expected Output

```text
----Use Mutex----
GET request received for endpoint '/users'.
POST request to 'login' with payload size 512 bytes.
DELETE request received for resource ID 10.
Total number of request processed : 3
Total GET request processed : 1
Total POST request processed : 1
Total DELETE request processed : 1

-----Use RwLock-----
GET request received for endpoint '/users'.
POST request to 'login' with payload size 512 bytes.
DELETE request received for resource ID 10.
Total number of request processed : 3
Total GET request processed : 1
Total POST request processed : 1
Total DELETE request processed : 1
```