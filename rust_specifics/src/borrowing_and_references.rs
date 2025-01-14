// Scope and validity
// Can only be one mutable reference to data to prevent data races
// Using curly braces to create multiple mutable references

fn references_and_functions() -> u8 {
    // Borrowing fundamentals
    6
}

fn passing_in_a_reference(){

}
fn multiple_mutable_references() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

// Cannot create a mutable reference from an already mutable variable
/*
let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

vs


let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

*/

// You cannot have a mutable reference to a non mutable value, users of an immutable reference do not expect this to suddenly change from under them and therefore change the data/ the way the program works