// copy, clone (heap vs stack)
// When data has a known size and is stored on the stack, copies are quick to make. Therefore we do not need to invalidate data and free memory, therefore you do not invalidate the first variable as with above.

fn copy_demo(){
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

// A tuple struct with resources that implements the `Clone` trait
// Taken directly from https://doc.rust-lang.org/rust-by-example/trait/clone.html
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);
fn clone_vs_move_demo(){
    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    //println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the moved original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    // println!("moved and dropped: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}

// Copy vs Clone in the eyes of the program
/*
Performance considerations
How do you know when something is moved or copied?
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clone_demo(){
        clone_vs_move_demo();
    }

    #[test]
    fn test_copy_demo(){
        copy_demo();
    }
}
