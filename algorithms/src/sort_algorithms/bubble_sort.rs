/*
The bubble sort algorithm iterates through a collection of elements
During each iteration it will compare two adjacent elements and bubble i.e. move the larger element to the right
This process continues in a loop until all the elements are in ascending order with the smallest being on the left and larger to the right

Advantages:
1. Simple to implement and understand
2. Memory usage is low and consistent, doesn't require lots to perform.

Disadvantages:
1. Slow - speed directly related to the length of list
*/

pub fn bubble_sort(array_to_be_sorted: &mut Vec<i64>) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for vector_element_index in 0..array_to_be_sorted.len() - 1 {
            if array_to_be_sorted[vector_element_index]
                > array_to_be_sorted[vector_element_index + 1]
            {
                println!("Smaller value detected to the right of a larger value, swapping...");
                array_to_be_sorted.swap(vector_element_index, vector_element_index + 1);
                swapped = true;
            } else {
                println!(
                    "Larger or Equal value detected to the right of a smaller value, continuing..."
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;
    #[test]
    fn test_bubble_sort_no_change() {
        let mut array_under_test: Vec<i64> = vec![1, 2, 3];
        bubble_sort(&mut array_under_test);
        // assert_eq!(sorted_array, &mut vec![1, 2, 3]);
        println!("{:?}", array_under_test);
    }

    #[test]
    fn test_bubble_sort_changes() {
        let mut array_under_test: Vec<i64> = vec![5, 3, 4, 1, 9];
        bubble_sort(&mut array_under_test);
        assert_eq!(array_under_test, [1, 3, 4, 5, 9]);
    }

    #[test]
    fn test_bubble_equal_values() {
        let mut array_under_test: Vec<i64> = vec![1, 1, 1];
        bubble_sort(&mut array_under_test);
        assert_eq!(array_under_test, [1, 1, 1]);
    }
}
