/* Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

Example 1:
Input: nums = [1,2,3,1]
Output: true

Example 2:
Input: nums = [1,2,3,4]
Output: false

Constraints: 1 <= nums.length <= 105, -109 <= nums[i] <= 109
*/

use std::collections::HashSet;

fn find_duplicates(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for n in nums {
        if !seen.insert(n) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_duplicates() {
        assert!(find_duplicates(vec![1, 2, 3, 1]));
        assert!(!find_duplicates(vec![1, 2, 3, 4]));
        assert!(find_duplicates(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}
