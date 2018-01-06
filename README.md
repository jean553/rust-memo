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
- [Scalar types](#scalar-types)
- [Compound types](#compound-types)

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

## Scalar types
Check the project `scalar_types`.

To remember:
* "scalar" types are integers (u8, i8, u16, i16, u32, i32, u64, i64, usize, isize),
floating-point numbers (f32, f64), booleans (bool) and characters,
* "usize" and "isize" size depends of the architecture (32 bits long or 64 bits long),
* if the type is not specified when declaring an integer variable, the default type is `i32`,
(BEWARE: Rust does not switch to i64/u64 if the value is too high for i32,
it adapts it in order to store it into the i32, a warning is raised though)
* f32 and f64 use IEEE-754 standard (https://en.wikipedia.org/wiki/IEEE_754),
f32 has a single precision, f64 has a double precision,
* char is four bytes long, and can store any unicode scalar value (http://www.unicode.org/glossary/#unicode_scalar_value),

Benefits:
* "usize" gives the guarantee to be always good enough to hold any pointer

## Compound types
Check the project `compound_types`.

To remember:
* there are two compound types: tuples and arrays,
* a tuple can have different data types, an array cannot have different data types,
* a tuple types can be implicit or explicit,
* array items are accessed using `array[index]` notation, tuples items are accessed using `tuple.index` notation,
* arrays and tuples are affected by mutability and the `mut` keyword
* arrays are simple chunks of memory allocated on the stack,
* arrays and tuples have a fixed size, on arrays, that size may be implicit or explicit
