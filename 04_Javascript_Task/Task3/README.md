# JavaScript Array Manipulation with Promises

## Problem Statement

Create a JavaScript program that manipulates arrays, calculates their sum, and uses a `Promise` to determine whether the sum meets a specified condition.

---

## Approach

The solution is divided into three main functions:

### fun1()

- Initializes an array using a `Symbol` key in an object.
- Removes the first element using `shift()`.
- Passes the removed element and the remaining array to another function (`fun2`).

### fun2(first_ele, arr1)

- Creates a new array associated with a unique `Symbol` key in a new object.
- Combines:
  - The shifted element from `fun1`
  - Remaining elements of the first array
  - A second predefined array `[6, 8]`
- Passes the final array to the `check` function for validation.

### check(arr)

- Calculates the sum of all elements using `reduce()`.
- Returns a `Promise`:
  - **Resolves** with `"Resolved"` if the sum is greater than `35`.
  - **Rejects** with `"rejected"` otherwise.

---

## Working of the Program

1. `fun1` initializes `[1,2,3,4,5]` and removes the first element `1`.
2. `fun2` combines `1`, `[2,3,4,5]`, and `[6,8]` into `[1,2,3,4,5,6,8]`.
3. `check` calculates the sum (`29`) and evaluates the condition.
4. Based on the sum:
   - Logs `"Resolved"` if sum > 35
   - Logs `"rejected"` otherwise.

---

## Concepts Used

- Functions
- Array methods: `shift()`
- Spread operator (`...`)
- Symbols for unique object keys
- Promises for asynchronous validation
- `reduce()` for summing array elements
- Conditional logic within Promises
