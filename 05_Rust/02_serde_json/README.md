# Rust Serialization & Deserialization – Student & Marks (Serde JSON)

## Table of Contents

- [Introduction](#introduction)
- [Task Overview](#task-overview)
- [Libraries Used](#libraries-used)
- [Structures Used](#structures-used)
- [Serialization Task](#serialization-task)
- [Deserialization Task](#deserialization-task)
- [Working of the Program](#working-of-the-program)
- [Learning Outcomes](#learning-outcomes)
- [Conclusion](#conclusion)
- [Resources](#resources)



## Introduction

This task focuses on understanding **serialization and deserialization** in Rust using the **Serde JSON** library.

The program demonstrates how Rust structures can be converted into JSON format (serialization) and how JSON data can be converted back into Rust structures (deserialization). This is a common requirement when working with APIs, configuration files, and data exchange between systems.



## Task Overview

The task includes:

- Defining nested Rust structures (`Student` and `Marks`)
- Using `serde` traits to enable JSON conversion
- Serializing a Rust struct into a JSON string
- Deserializing a JSON string into a Rust struct
- Accessing deserialized data safely



## Libraries Used

- `serde`
- `serde_json`

These libraries provide powerful and efficient JSON serialization and deserialization support in Rust.



## Structures Used

### Marks Structure

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Marks {
    maths: i32,
    chemistry: i32,
    physics: i32,
}
```

### Student Structure

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Student {
    name: String,
    id: i32,
    email: String,
    phone_number: String,
    grade: char,
    marks: Marks,
}
```
Both structures derive:

- Serialize for converting Rust data to JSON
- Deserialize for converting JSON data to Rust
- Debug for structured printing

## Serialization Task

In the serialization task:

- A `Student` object is created with predefined values  
- The object is converted into a JSON string using `serde_json::to_string()`  
- The resulting JSON string is printed  

This demonstrates how Rust data structures can be easily transformed into JSON format.



## Deserialization Task

In the deserialization task:

- A JSON string containing student data is defined
- The JSON string is converted into a `Student` object using `serde_json::from_str()`
- Individual fields of the deserialized object are accessed and printed
- In this task, **both approaches of deserialization** are demonstrated:  

  1. **First serialize and deserialize**  
     - The `Student` struct is first serialized into a `String` using `serde_json::to_string()`.  
     - The resulting `String` is then deserialized back into a `Student` object using `serde_json::from_str(&json_string2)`.  
     - Individual fields are accessed and printed.  

  2. **Deserialize using a raw string literal**  
     - A JSON string literal (`&str`) is directly deserialized into a `Student` object using `serde_json::from_str(json_string1)`.  
     - This demonstrates that raw string literals can be used without taking a reference, since they are already of type `&str`.


This demonstrates how external JSON data can be safely mapped into Rust structures.



## Working of the Program

- Define Rust structures with Serde traits  

### For Serialization:
- Create a `Student` instance  
- Convert it into JSON using `to_string()`  

### For Deserialization:
- Read JSON data as a string  
- Convert it into a `Student` object using `from_str()`  
- Access and print student details and marks  
- Use debug formatting (`{:?}`) to display nested structures  



## Learning Outcomes

By completing this task, you will understand:

- How to use Serde for JSON serialization and deserialization  
- How to derive `Serialize` and `Deserialize` traits  
- Converting Rust structs to JSON strings  
- Parsing JSON strings into Rust structs  
- Handling nested structures during serialization  
- Safe data access after deserialization  



## Conclusion

This Rust task provides practical experience with **Serde JSON**, enabling efficient data exchange between Rust programs and external systems. It builds a strong foundation for working with APIs, file-based data storage, and advanced Rust applications.

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)

