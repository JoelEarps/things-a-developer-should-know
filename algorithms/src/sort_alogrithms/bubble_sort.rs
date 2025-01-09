/* 
The bubble sort algorithm iterates through a collection of elements
During each iteration it will compare two adjacent elements and bubble i.e. move the larger element to the right
This process continues in a loop until all the elements are in ascending order with the smallest being on the left and larger to the right 

Advantages:
1. Simple

Disadvantages:
1. Slow
*/

fn bubble_sort(array_to_be_sorted: Vec<i64>) ->  Vec<i64> {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for item in 0..array_to_be_sorted.len() - 1 {
            
        }
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test_bubble_sort(){

    }
}