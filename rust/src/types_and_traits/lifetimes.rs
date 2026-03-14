// Keywords: lifetime, lifetime elision, borrow checker, 'a, &'a
/*
String and longest function as example in Rust book
Custom trait example? */

fn basic_lifetime_examples<'a, 'b>() {
    // Examples of syntax
    let example_1: &i32 = &6; // a reference
    let example_2: &'a i32 = &17; // a reference with an explicit lifetime
    let example_3: &'b mut i32 = Box::leak(Box::new(15)); // a mutable reference with an explicit lifetime

    /*
    You see above I had to allocate a new variable to the heap in order to create a mutable reference, if i were to do:

    `let example_3: &'b mut i32 = &mut 15;`

    This would create a dangling reference

    15 is a literal (just a number, not a variable).
    Rust does not allocate a named memory location for it.
    The expression &mut 15 tries to create a mutable reference to this temporary value.
    However, because 15 has no named storage, it gets dropped immediately after the statement.
    The reference example_3 would then point to an invalid memory location, which Rust prevents.

     */
}

// Without lifetimes you would get
// missing lifetime specifier
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from
// Lifetimes go in the function signature
fn longest<'input_lifetime>(
    x: &'input_lifetime str,
    y: &'input_lifetime str,
) -> &'input_lifetime str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// structs and lifetimes
// If you declare a struct you have to define the lifetime so the compiler knows the struct will outlive the reference
pub(super) struct ImportantStruct<'struct_lifetime> {
    name: &'struct_lifetime str,
}

// Generic Lifetime Parameters, Trait Bounds and Lifetimes together

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_lifetimes() {
        basic_lifetime_examples();
        let string_return = longest("Hello", "Hellllooo");
        assert_eq!(string_return, "Hellllooo");
    }
}
