# Day 1: Rust Functions and Basic Operations

Welcome to Day 1 of your Rust learning journey! Today we'll focus on functions and basic arithmetic operations in Rust.

## Solved Examples

### 1. Basic Function with Print
```rust
pub fn function_demo() {
    let x = 5;
    let y = 10;
    println!("x + y = {}", x + y);
}
```
This example demonstrates:
- Function declaration with `pub` (public) visibility
- Variable declaration using `let`
- Basic arithmetic operation
- String formatting with `println!`

### 2. Function with Parameters and Return Value
```rust
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```
This example shows:
- Function parameters with type annotations
- Return type specification (`-> i32`)
- Implicit return (no semicolon needed)

## Practice Questions

### Level 1: Basic Functions
1. **Greet Me**
   - Write a function that prints a greeting
   - Function signature: `fn greet(name: &str)`
   - Example: `greet("Alice")` should print "Hello, Alice!"

2. **Add Two Numbers**
   - Create a function that adds two integers
   - Function signature: `fn add(a: i32, b: i32) -> i32`
   - Example: `add(5, 3)` should return `8`

3. **Is Positive?**
   - Write a function that checks if a number is positive
   - Function signature: `fn is_positive(num: i32) -> bool`
   - Example: `is_positive(5)` should return `true`

### Level 2: Working with Data
4. **Find Minimum**
   - Write a function that finds the smallest number in a slice
   - Function signature: `fn find_min(slice: &[i32]) -> i32`
   - Example: `find_min(&[5, 2, 8, 1])` should return `1`

5. **Count Vowels**
   - Create a function that counts vowels in a string
   - Function signature: `fn count_vowels(s: &str) -> usize`
   - Example: `count_vowels("hello")` should return `2`

6. **Reverse an Array**
   - Write a function that reverses an array in-place
   - Function signature: `fn reverse_array(arr: &mut [i32])`
   - Example: `let mut arr = [1, 2, 3]; reverse_array(&mut arr);` should make `arr` equal to `[3, 2, 1]`

### Level 3: Logic & Algorithms
7. **Factorial**
   - Implement a recursive factorial function
   - Function signature: `fn factorial(n: u32) -> u32`
   - Example: `factorial(5)` should return `120`

8. **Is Prime?**
   - Write a function that checks if a number is prime
   - Function signature: `fn is_prime(n: u32) -> bool`
   - Example: `is_prime(17)` should return `true`

9. **Fibonacci Iterative**
   - Create an iterative Fibonacci function
   - Function signature: `fn fibonacci_iter(n: u32) -> u32`
   - Example: `fibonacci_iter(7)` should return `13`

### Level 4: Real-World Scenarios
10. **Validate Email**
    - Write a function that checks email validity
    - Function signature: `fn validate_email(email: &str) -> bool`
    - Example: `validate_email("user@example.com")` should return `true`

11. **Calculate BMI**
    - Create a function that calculates BMI
    - Function signature: `fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64`
    - Example: `calculate_bmi(70.0, 1.75)` should return the BMI value

12. **Palindrome Checker**
    - Write a function that checks if a string is a palindrome
    - Function signature: `fn is_palindrome(s: &str) -> bool`
    - Example: `is_palindrome("radar")` should return `true`

### Level 5: Challenges
13. **Merge Two Arrays**
    - Write a function that merges and sorts two arrays
    - Function signature: `fn merge_arrays(a: &[i32], b: &[i32]) -> Vec<i32>`
    - Example: `merge_arrays(&[1, 3], &[2, 4])` should return `[1, 2, 3, 4]`

14. **Anagram Checker**
    - Create a function that checks if two strings are anagrams
    - Function signature: `fn are_anagrams(s1: &str, s2: &str) -> bool`
    - Example: `are_anagrams("listen", "silent")` should return `true`

15. **Simple Calculator**
    - Implement a function that performs basic arithmetic
    - Function signature: `fn calculate(op: char, a: f64, b: f64) -> Option<f64>`
    - Example: `calculate('+', 5.0, 3.0)` should return `Some(8.0)`

## Tips for Practice
- Always use type annotations for function parameters and return types
- Test your functions with different input values
- Use `println!` to debug your code
- Remember that Rust is strict about types, so be careful with number types (i32, f64, etc.)
- For string operations, remember that Rust strings are UTF-8 encoded
- When working with slices and arrays, be mindful of ownership and borrowing rules

## How to Test Your Solutions
1. Create a new function in `functions.rs`
2. Add a call to your function in `main.rs`
3. Run your code using `cargo run`
4. Write test cases to verify your functions work correctly

Happy coding! 🦀 