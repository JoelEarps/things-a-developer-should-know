Key Principles in Rust

## Ownership

What are the rules

How to employ them

## Borrowing

What is it?

At any given time you can have either one mutable reference or unlimited immutable references must always be valid


How to do proper borrowing

1. References

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