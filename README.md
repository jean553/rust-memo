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
- [References](#references)
    * [The rules for references](#the-rules-for-references)
    * [Mutable reference](#mutable-reference)
    * [Immutable reference](#immutable-reference)
    * [Invalid references](#invalid-references)
    * [Borrowing](#borrowing)
- [Lifetimes](#lifetimes)
    * [Concept](#concept)
    * [The problem](#the-problem)
    * [References lifetimes into functions declarations](#references-lifetimes-into-functions-declarations)
    * [References lifetimes into structures](#references-lifetimes-into-structures)
    * [References lifetimes into implementations](#references-lifetimes-into-implementations)

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

## References

The Rust rules about references are:

### The rules for references

```
At any given time, you can have either but not both of:
* One mutable reference.
* Any number of immutable references.

References must always be valid.
```

Benefits:
* Only the owner of a variable can modify its value,
* If the variable is borrowed by a reference, only one borrowing reference can modify its value,
* Safety along ownership and borrowing ensures that only one variable access can modify it at a time (multi-threading)

### Mutable reference

A variable can have exactly one mutable reference.

```rust
let mut variable = String::from("one string");
let reference = &mut variable;
```

### Immutable reference

A variable can have many immutable references.

```rust
let variable = String::from("one string");
let reference = &variable;
let other_reference = &variable;
```

### Invalid references

This causes a compilation error. This happens when the variable
goes out of the scope before its reference.

```rust
let variable = String::from("one string");
let mut reference = &variable; // "reference" is a reference to "variable"

let other_variable = String::from("other string");
reference = &other_variable; // "reference" is a reference to "other_variable"

// "other_variable" goes out of the scope before the others variables;
// at this moment, "reference" is still its reference, so there is an error.
```

### Borrowing

When a variable is borrowed by a reference, only the reference can access it until the reference goes out of the scope.

```rust
let mut variable = Structure {
    value: 10,
};
let reference = &mut variable;
variable.value = 20; // error: "variable" is borrowed by "reference"
```

```rust
let mut variable = Structure {
    value: 10,
};

{
    let reference = &mut variable;
    reference.value = 20;
}

variable.value = 30; // works well a "reference" does not borrow "variable" anymore
```

## Lifetimes

### Concept

Using Rust, every variable and reference has a lifetime.
The lifetime determines how long a variable/reference exists.

```rust
{
    let value = 10; // lifetime of "value" starts here

    /* ... */

    let other_value = 20; // lifetime of "other_value" starts here

    /* ... */

    {
        let other_reference = &value; // lifetime of "other_reference" starts here

        /* ... */

        // lifetime of "other_reference" ends here
    }

    let reference = &value; // lifetime of "reference" starts here

    /* ... */

    // lifetimes of "value", "other_value", "reference" ends here
}
```

One of the reference rules is: "a reference cannot be invalid". That means a reference must
always refered a variable that really exists (still accessible).

For instance, the following code does not compile:

```rust
{
    let value = 10;
    let mut reference = &value; // lifetime of "reference" starts here

    {
        let other_value = 20; // lifetime of "other_value" starts here
        reference = &other_value;

        // lifetime of "other_value" ends here
    }

    // error: "reference" lifetime is longer than its refered value "other_value",
    // calling "reference" from here makes no sense
}
```

### The problem

Let's take the following example:

```rust
fn get_reference(reference: &i32) -> &i32 {
    reference
}

fn main() {

    let value = 10;
    let reference = get_reference(&value);
}
```

This code compiles without any error.

First, `get_reference` returns for sure a reference that has the same lifetime
as the passed reference parameter.

Why ? Because there is now way to return a reference to an object created within the function,
as the object lifetime would be smaller than the returned reference lifetime.
There is only one passed argument, so for sure, the returned reference can only be a reference
to the same object, so with the same lifetime.

Let's now consider the following code:

```rust
fn get_reference(
    first: &i32,
    second: &i32,
) -> &i32
{
    if *first == 0 {
        first
    } else {
        second
    }
}

fn main() {

    let first = 10;
    let second = 20;

    let reference = get_reference(
        &first,
        &second,
    );
}
```

The compilation of the code above fails and Rust requires us to indicate the lifetime
of the returned reference.

In fact, the returned reference will refer for sure to the first or the second parameter
reference value. But at compilation time, there is absolutely no way to find out which one.
In that case, Rust is not able to deduce the lifetime of `reference`.

It may not seem to be a problem in the code above, but let's now check the code here:

```rust
let reference;

let first = 10;

{
    let second = 20;

    reference = get_reference(
        &first,
        &second,
    );
}

println!("{}", reference); // error if "reference" refers to "second" !
```

### Reference lifetimes into functions declarations

In order to use a lifetime, it is first mandatory to declare it.
The simple way to do it when using it into a function is:

```rust
fn function_name<'a> {
    ...
```

`'a` is a declared lifetime, that's all.

In order to solve our problem above, we have to indicate that every lifetime of parameters
passed to the function have to be the same, and the same as the returned reference.

```rust
fn get_reference<'a>(
    first: &'a i32,
    second: &'a i32,
) -> &'a i32 {
    ...
}
```

Calling this function with the following code now works well:

```rust
let first = 10;
let second = 20;

let reference = get_reference(
    &first,
    &second,
);
```

The two passed references must have the same lifetime,
the returned reference also has the same lifetime,
there is no risk of invalid reference.

On the other hand, using the following client code won't work:

```rust
let first = 10;
let reference;
{
    let second = 20;
    reference = get_reference(
        &first,
        &second,
    );
}
```

In that case, Rust excepts `&first` and `&second` to have the same lifetime (`'a`)
when calling the function `get_reference`, and this lifetime should be the same as `reference`
(as the returned reference `&'a i32` also has the lifetime `'a`).

In the case below, this is wrong: `second` goes out the scope when still borrowed by `reference`.
The compilation fails.

One solution would be to modify the function signature (note `'b` for the second param):

```rust
fn get_reference<'a, 'b>(
    first: &'a i32,
    second: &'b i32,
) -> &'a i32
{
    ...
}
```

This also fails because the function "might" return `second`. `second` has a `'b` lifetime
and the returned reference has a `'a` lifetime, so this code cannot compile.

References explicit lifetimes are not a kind of syntax rule to respect but more "an additional
information to add to the code in order to ensure that you know what you are doing".

Meanwhile, if the function returns for sure always the same reference,
there is no need to attribute reference lifetimes to other parameters:

```rust
fn get_reference<'a>(
    first: &'a i32,
    second: &i32,
) -> &'a i32
{
    first
}
```

In the example above, the `second` parameter is never returned,
so there is no need to indicate its lifetime.

### References lifetimes into structures

It is possible to store references into structures.
In that case, it is required for the reference to have a lifetime.
The name of the lifetime has to be declared as the same way as for a function.

```rust
struct MyStructure<'a> {
    reference: &'a i32,
}
```

For example, the following code does not compile as "other_value" lifetime
is not the same as the one used when the object has been created with "value" lifetime.

```rust
let value = 10;
let object = MyStructure {
    reference: &value,
};
{
    let other_value = 20;
    object.reference = &other_value;
}
```

### References lifetimes into implementations

Lifetimes can (or must) be used into implementations.
If the structure of the implementation used a lifetime,
then this is required to specify the lifetime when indicating the structure.
As the structure uses a lifetime, this is also require to declare it with 'impl'.

Example:

```rust
impl<'a> MyStructure<'a> {
}
```

It is then possible to indicate the reference lifetime into the method definition:

```rust
impl<'a> MyStructure<'a> {

    pub fn get_reference(&'a self) -> &'a i32 {
        self.reference
    }
}
```

As the same way, reference lifetimes into structures gives information about the returned references:

```rust
struct<'a> Structure<'a> {
    first_reference: &'a i32,
    second_reference: &'a i32,
}

impl<'a> Structure<'a> {

    pub fn get_reference(&self) -> &'a i32 {

        if (true) {
            first_reference
        } else {
            second_reference
        }
    }
}
```

The code above can only compile if the references set for "first_reference" and "second_reference"
have the same lifetime.
