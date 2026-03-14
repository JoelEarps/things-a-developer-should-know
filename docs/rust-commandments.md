Key Principles in Rust

## Ownership

Each Value in Rust has an owner
There can only be one owner at a time
When the owner goes out of scope the value will be dropped
If something takes ownership, it will be `moved` - thus rendering the place it moved from as redundant and freed back to the program.

Things to remember and ask yourself:
1. Is this data a fixed size at compile time, i.e. stack or the heap?
2. Do i need to pass the data into a different scope?
3. Passing and returning data to and from functions can result in a change of ownership.
4. Always check for scope change
5. Declaring a variable from another variable will render the free the initial variable.

Often you will need to copy or clone data to be able to pass it without ownership:

### Copy

Often used for scalar data types or compound types with inner scalar values i.e. values with a fixed size on the stack. 
Relatively inexpensive to perform.

### Clone 

A super trait of copy, often used for heap data or for types that cannot be copied.
Can be very expensive.

## Borrowing

Passing a value to a different part of the program without the need to take ownership of it.

At any given time you can have either one mutable reference or unlimited immutable references must always be valid

The rules of borrowing and references:

1. At any given time you can have either one mutable reference or unlimited immutable references
2. References must always be valid


## Lifetimes

What are they?

How to enforce proper lifetimes

When to do your own vs when to let the lifetime be defined generically?


## Pattern Matching

Why is it different to an if?

When to use?

## Custom Error Handling 

ThisError and Anyhow

## Memory Management 

### String vs &str

use &str when you are working with slices or strings known at compile time, this will be placed on the stack and will
will make you more efficient.
Often use when you don't need to transfer ownership
Use when you do not need the data to be mutable and require to be read only.

Use String when you need a mutable heap allocated string
When working with APIs that require ownership (e.g., storing strings in a collection like a Vec<String>).
When you need to transfer the string's ownership to another function or thread.

See `rust_specifics/src/string_and_ampersand_str.rs` for more info.