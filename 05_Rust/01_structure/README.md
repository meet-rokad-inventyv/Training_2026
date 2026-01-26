# Rust Structures – Student & Marks Management

## Table of Contents

- [Introduction](#introduction)
- [Task Overview](#project-overview)
- [Structures Used](#structures-used)
- [Implemented Methods](#implemented-methods)
- [Working of the Program](#working-of-the-program)
- [Learning Outcomes](#learning-outcomes)
- [Conclusion](#conclusion)
- [Resources](#resources)


## Introduction

This task is focused on understanding and implementing **structures (`struct`)**, **methods (`impl`)**, **ownership**, and **borrowing** concepts in Rust.

The program models a real-world scenario where a **Student** has personal details and **Marks** in different subjects. It demonstrates how to define nested structures, update values using mutable references, and safely access data using getter methods.


## Task Overview

The task consists of:
- A `Student` structure containing student details
- A `Marks` structure containing subject-wise marks
- Getter and setter methods for controlled access
- Printing structured information before and after updates


## Structures Used

### Marks Structure

```rust
#[derive(Debug)]
struct Marks {
    maths: i32,
    chemistry: i32,
    physics: i32,
}
```

### Student Structure

```rust
#[derive(Debug)]
struct Student {
    name: String,
    id: i32,
    email: String,
    phone_number: String,
    grade: char,
    marks: Marks,
}
```

## Implemented Methods

### Student Methods

- `set_name()` / `get_name()`
- `set_id()` / `get_id()`
- `set_email()` / `get_email()`
- `set_phone_number()` / `get_phone_number()`
- `set_grade()` / `get_grade()`
- `get_marks()`
- `get_student_info()`
- `get_student_info_with_args()`

These methods demonstrate:

- Mutable borrowing using `&mut self`
- Immutable borrowing using `&self`
- Safe data access without violating ownership rules

### Marks Methods

- `set_maths()` / `get_maths()`
- `set_chemistry()` / `get_chemistry()`
- `set_physics()` / `get_physics()`

These methods manage subject marks using mutable and immutable references.


## Working of the Program

- A `Student` object is created with initial values
- Student and marks details are printed
- The student data is updated using setter methods
- Updated details are printed again
- Debug formatting (`{:?}`) is used to display the `Marks` structure

## Learning Outcomes

By completing this task, you will understand:

- How to define and use Rust structures
- How to implement methods using `impl`
- Ownership and borrowing rules in practice
- Difference between returning values and references
- Usage of the `Debug` trait for structured printing
- Updating nested structures safely


## Conclusion

This Rust task provides hands-on experience with **structures, methods, and borrowing rules**.


## Resources

- [Rust Documentation](https://doc.rust-lang.org/)



