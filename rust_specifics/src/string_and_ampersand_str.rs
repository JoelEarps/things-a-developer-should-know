// String reference - &str
// You know the size at compile time ,and it is immutable
// It is a borrowed reference
// Commonly used for passing read only data therefore ownership changes are not required

// String Type - String::new();
// Difference is in memory management - the String type is intended for when you needed a string that is defined at run time
// It is growable, heap-allocated, UTF-8 encoded string.
// String is responsible for its own ownership and handles its own allocation and de-allocation
fn function_using_string_literals(test_string_literal: &str) -> &str {
    println!("{}", test_string_literal);
    let test_slice: &str = &test_string_literal[..4];
    println!("Test slice with string reference {}", test_slice);

    test_slice
}

fn playing_with_string_references(string_literal_for_ownership_demo: &str) {
    println!("I never wanted to own this string: {}", string_literal_for_ownership_demo);
}

fn working_with_strings(mut test_string_type: String) -> String {
    let mut string_1 = String::new();
    string_1.push_str("hello");
    let mut string_2 = String::from("hello, ");
    string_2.push_str("world!");
    println!("string_1: {}, string_2: {}", string_1, string_2);

    test_string_type.push_str("Mutable Ownership");

    // TRY ME: If the line below (33) is uncommented you will get the error
    // Value used after being moved [E0382]
    // playing_with_strings(test_string_type);
    // You have to uses, clone the data or transfer ownership in and out, seems inefficient right?
    playing_with_strings(&test_string_type);
    // The perfect use for string references as you are not changing anything and are needing to pass a reference!
    test_string_type
}

fn playing_with_strings(string_input_for_ownership_demo: &String){
    println!("If I wanted to, I could own the string or be a clone: {string_input_for_ownership_demo}")
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn string_reference_tests(){
        let slice = function_using_string_literals("Hello Joel");
        assert_eq!(slice, "Hell");
    }

    #[test]
    fn std_string_tests(){
        let ownership_returned = working_with_strings(String::from("What do we want? "));
        assert_eq!(ownership_returned, "What do we want? Mutable Ownership");
    }
}