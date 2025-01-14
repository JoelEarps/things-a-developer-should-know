/*
write a function that takes a string of words separated by spaces and returns the first word it finds in that string
 https://doc.rust-lang.org/book/ch04-03-slices.html#:~:text=%26str%3A-,Filename%3A%20src/main.rs,-fn%20first_word(s
 */

// Linked to References
// String Slices - references to a subsection of a string

// Range syntax
// You can define start and end points but if you are referencing the start or end you can leave blank
// let slice = &s[0..2]; - definitive start and end
// let slice = &s[..2]; - assume at start
// let slice = &s[3..]; - assume end
// let slice = &s[..]; - slice of entire string