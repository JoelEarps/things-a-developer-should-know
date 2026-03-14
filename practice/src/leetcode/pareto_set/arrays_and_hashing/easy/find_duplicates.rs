/* Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

Example 1:

Input: nums = [1,2,3,1]

Output: true

Explanation:

The element 1 occurs at the indices 0 and 3.

Example 2:

Input: nums = [1,2,3,4]

Output: false

Explanation:

All elements are distinct.

Example 3:

Input: nums = [1,1,1,3,3,4,3,2,4,2]

Output: true

Constraints:
1 <= nums.length <= 105
-109 <= nums[i] <= 109

*/

use std::collections::HashSet;

/// What does this teach you, best 
fn find_duplicates(nums: Vec<i32>) -> bool {
    let mut value_store = HashSet::new();
    let mut duplicate = false;

    for number in nums {
        if value_store.insert(number) {
            println!("Value inerted no duplciate found, continuing");
        } else {
            duplicate = true;
            break;
        }
    }
    duplicate
}

fn more_idiomatic_version_for_find_duplciates(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for n in nums {
        if !seen.insert(n) {
            return true; // already in set → duplicate
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_duplicates() {
        assert_eq!(find_duplicates(vec![1, 2, 3, 1]), true);
        assert_eq!(find_duplicates(vec![1, 2, 3, 4]), false);
        assert_eq!(find_duplicates(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }

    #[test]
    fn test_more_idiomatic_version_for_find_duplciates() {
        assert_eq!(more_idiomatic_version_for_find_duplciates(vec![1, 2, 3, 1]), true);
        assert_eq!(more_idiomatic_version_for_find_duplciates(vec![1, 2, 3, 4]), false);
        assert_eq!(more_idiomatic_version_for_find_duplciates(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
