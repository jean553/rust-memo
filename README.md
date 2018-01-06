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
- [Statements and expressions](#statements-and-expressions)
- [Ownership](#ownership)
    * [The rules of ownership](#the-rules-of-ownership)
    * [Move semantics](#move-semantics)
    * [Copy instead of move](#copy-instead-of-move)
    * [`Copy` trait](#copy-trait)
    * [Function call](#function-call)

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

## Statements and expressions
Check the project `statements_and_expressions`.

To remember:
* a `statement` is a code instruction that does not return a value,
an `expression` is a code instruction that returns a value,
* declaring a variable with `let` is a statement, defining a function is also a statement,
* statements contains expressions, expressions are defined with blocks (`{}`),
* expression that returns nothing return an unit type (`()`) (counterpart of `void` in C),
* statements end with a semicolon, expressions have no semicolon at the end

## Ownership
Check the project `ownership`.

### The rules of ownership

The three rules of ownership in Rust:

```
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
```

### Move semantics

The following situation happens to objects
that are allocated on the heap memory.

```rust
let text = String::from("my content");
let other_text = text;

// "text" is not callable anymore, its stack part has been copied
// into an "other_text" variable, the heap part is unchanged,
// the previous stack part ("text") cannot be accessed anymore
```

In the example above, a `String` is allocated on the heap.
Meta-data of the string is allocated on the stack,
including a pointer to the heap allocation.

When `other_text` is declared from `text`, the stack part of the object
is copied and is now accessible through `other_text`.
The heap allocated data is not affected.

![Image 1](images/ownership_1.png)

As a value can only have one owner at a time, `other_text` becomes the owner
and `text` cannot be used anymore.

![Image 2](images/ownership_2.png)

### Copy instead of move

The following situation happens to objects
that are allocated on the heap memory.

```rust
let text = String::from("my content");
let other_text = text.clone();
```

In the example above, both of the stack and heap memory
is copied and a brand new variable is created.
Both `text` and `other_text` are accessible within the context.

### `Copy` trait

Most of the types that do not require to set any data on the heap memory
(and that only live on the stack memory) generally implement the `Copy` trait.

This trait makes the variables from those types to be copied by default.
This is the case for all the scalar types.

```rust
let x = 5;
let y = x;
println!("{}", x); // 5
println!("{}", y); // 0
```

### Function call

Exactly the same situation happens when a variable is passed to function.
If the variable has heap memory allocation, the variable is moved and won't
be accessible anymore from the client scope.
If the variable has stack memory allocation only, the variable is copied
(if it implements the `Copy` trait).

```rust
fn function(value: String)
{
    println!("{}", value);
}

fn main()
{
    let value = String::from("my value");
    function(value);
    println!("{}", value); // error: value has been moved
}
```

```rust
fn function(value: i32)
{
    println!("{}", value);
}

fn main()
{
    let value = 50;
    function(value);
    println!("{}", value); // no error
}
```
