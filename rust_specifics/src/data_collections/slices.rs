/*
write a function that takes a string of words separated by spaces and returns the first word it finds in that string
 https://doc.rust-lang.org/book/ch04-03-slices.html#:~:text=%26str%3A-,Filename%3A%20src/main.rs,-fn%20first_word(s
 */

// Linked to References
// String Slices - references to a subsection of a string

// Range syntax
// You can define start and end points but if you are referencing the start or end you can leave blank
fn string_slice_demo(example_string: String){
    let slice = &example_string[0..2]; // definitive start and end
    println!("{slice}");
    let slice = &example_string[..2]; // assume at start
    println!("{slice}");
    let slice = &example_string[3..]; // assume end
    println!("{slice}");
    let slice = &example_string[..]; // slice of entire string
    println!("{slice}");
}

fn array_slice_demo(){
    let test_array = [1, 2, 3, 4, 5];
    let test_slice = &test_array[..3];
    println!("{:?}", test_slice);

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_string_slice(){
        string_slice_demo(String::from("Hello, Joel"));
    }

    #[test]
    fn test_array_slice(){
        array_slice_demo();
    }
}