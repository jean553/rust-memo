# rust-memo

A personal memo about all the basic stuffs to know about the Rust language. Useful when I code...

## Sources

Simplified list of Rust basic features (and benefits!) written
using the official Rust documentation (https://doc.rust-lang.org/book/second-edition).

## Compile the examples

```sh
cd example_folder/
cargo run
```

## Table of contents
- [Variables and mutability](#variables-and-mutability)
- [Constants](#constants)
- [Shadowing](#shadowing)

## Variables and mutability
Check the project `variables_and_mutability`.

To remember:
* every variable is immutable by default, declare them with `mut` to make them mutable,
* if an object is immutable, that means all its attributes are immutable,
* if an array is immutable, that means all its items are immutable,
* variables declared with `let` cannot be declared into the global scope

Benefits:
* you know what variable might be modified or not,
* by default, a variable cannot be modified, it prevents bad surprises
if a part of the program accidently tries to change the variable,
especially when using multiple threads.

## Constants
Check the project `constants`.

To remember:
* when declaring a constant, the data type is mandatory,
* a constant is computed at compilation, so Rust must be able to deduce its value during the compilation,
* a constant can be declared as global,
* a constant can be built from raw values, from another constant, from `const fn` functions,
* `const fn` functions are evaluated at compilation time and cannot declare `let` variables

Benefits:
* give an explicit name to raw values instead of losing insignificant digits, strings... in the code,
* you only need to modify the constant declaration value in order to modify its value everywhere it is used,
* explicitly calculate a value at compilation-time and not at runtime

## Shadowing
Check the project `shadowing`.

To remember:
* "shadow" a variable means create another variable with the same name, calling this name means getting the second variable value,
* shadowing can be performed with totally different types between the shadowed variables

Benefits:
* stop variables mutability (or start variables mutability),
* limit the amount of variables
