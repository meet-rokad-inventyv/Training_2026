## Problem Statement

Create a JavaScript program that manipulates arrays, calculates their sum, and uses a Promise to determine whether the sum meets a specified condition.

---

## Approach

The solution is divided into three main functions:

### fun1()

- Initializes an array  
- Removes the first element using `shift()`  
- Passes the removed element and remaining array to another function  

### fun2(first_element, array1)

- Creates a new array  
- Combines:
  - The shifted element  
  - Remaining elements of the first array  
  - A second predefined array  
- Passes the final array to a validation function  

### check(array2)

- Calculates the sum of all elements using `reduce()`  
- Returns a Promise:
  - Resolves if the sum is greater than `35`  
  - Rejects otherwise

---

## Concepts Used
- Functions
- Array methods - shift, unshift, push
- Spread operator
- Promise